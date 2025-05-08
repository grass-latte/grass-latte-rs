use crate::interface::{Event, EventTypes, HandledPacket, SendTypes};
use crossbeam::channel::{Receiver, Sender, unbounded};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use std::{io, thread};
use tungstenite::{Message, WebSocket};

pub static GLOBAL_SENDER: Lazy<Sender<SendTypes>> = Lazy::new(|| {
    let (s, r) = unbounded();
    thread::spawn(move || sender(r));
    s
});

// #[derive(Default)]
// pub struct TreePart(HashMap<String, (Option<EventTypes>, TreePart)>);
//
// impl TreePart {
//     pub fn get_path_mut(&mut self, path: &[String]) -> &mut (Option<EventTypes>, TreePart) {
//         let (part, path_remaining) = path.split_first().unwrap();
//
//         if let Some(path_mut) = self.0.get_mut(part) {
//             return if path_remaining.is_empty() {
//                path_mut
//             }
//             else {
//                 path_mut.1.get_path_mut(path_remaining)
//             };
//         }
//
//         self.0.insert(part.clone(), (None, Default::default()));
//         let path_mut = self.0.get_mut(part).unwrap();
//         if path_remaining.is_empty() {
//             path_mut
//         }
//         else {
//             path_mut.1.get_path_mut(path_remaining)
//         }
//     }
// }

pub static GLOBAL_EVENTS: Lazy<Mutex<HashMap<Vec<String>, EventTypes>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static GLOBAL_CLICK_CALLBACKS: Lazy<Mutex<HashMap<Vec<String>, Box<dyn Fn() + Send>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn bind_in_range(start_port: u16, end_port: u16) -> io::Result<(TcpListener, u16)> {
    let addrs: Vec<SocketAddr> = (start_port..=end_port)
        .map(|i| SocketAddr::from(([127, 0, 0, 1], i)))
        .collect();
    match TcpListener::bind(addrs.as_slice()) {
        Ok(listener) => {
            let port = listener.local_addr()?.port();
            Ok((listener, port))
        }
        Err(e) => Err(e), // Try the next port
    }
}

fn sender(r: Receiver<SendTypes>) {
    let websocket_port_range = PORT_RANGE.get_or_init(|| DEFAULT_PORT_RANGE);
    let (server, _server_port) =
        bind_in_range(websocket_port_range.0, websocket_port_range.1).unwrap();

    for stream in server.incoming() {
        let stream = stream.unwrap();

        let mut websocket = tungstenite::accept(stream).unwrap();

        let hello = websocket.read().unwrap();

        match hello {
            Message::Text(_) => {}
            Message::Binary(_) => {}
            Message::Ping(_) => {}
            Message::Pong(_) => {}
            Message::Close(_) => {
                continue;
            }
            Message::Frame(_) => {}
        }

        websocket.get_mut().set_nonblocking(true).unwrap();

        let websocket = Arc::new(Mutex::new(websocket));

        let ws_clone = Arc::clone(&websocket);

        thread::spawn(move || {
            receiver(ws_clone);
        });

        for message in r.iter() {
            let json = serde_json::to_string(&message).unwrap();
            let mut lock = websocket.lock().unwrap();
            while lock.send(json.clone().into()).is_err() {
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

fn receiver(websocket: Arc<Mutex<WebSocket<TcpStream>>>) {
    loop {
        thread::sleep(Duration::from_millis(100));
        let msg = {
            if let Ok(msg) = websocket.lock().unwrap().read() {
                msg
            } else {
                continue;
            }
        };

        match msg {
            Message::Text(t) => {
                let event: Event = serde_json::from_str(t.as_ref()).unwrap();

                match event.data() {
                    EventTypes::Click(_) => {
                        if let Some(callback) =
                            GLOBAL_CLICK_CALLBACKS.lock().unwrap().get(event.path())
                        {
                            GLOBAL_SENDER
                                .send(SendTypes::Handled(HandledPacket::new(event.path().clone())))
                                .unwrap();

                            callback();
                        }
                    }
                }

                GLOBAL_EVENTS
                    .lock()
                    .unwrap()
                    .insert(event.path().clone(), event.into_data());
            }
            Message::Binary(_) => {}
            Message::Ping(_) => {}
            Message::Pong(_) => {}
            Message::Close(_) => {
                return;
            }
            Message::Frame(_) => {}
        }
    }
}

pub const DEFAULT_PORT_RANGE: (u16, u16) = (3030, 3035);
pub static PORT_RANGE: OnceLock<(u16, u16)> = OnceLock::new();
