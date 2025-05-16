use crate::interface::{Event, EventTypes, HandledPacket, SendTypes};
use crossbeam::channel::{Receiver, Sender, unbounded};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use std::{io, thread};
use tungstenite::{Message, WebSocket};

/// Queue and thread for sending messages to the web interface
pub static GLOBAL_SENDER: Lazy<Sender<SendTypes>> = Lazy::new(|| {
    let (s, r) = unbounded();
    thread::spawn(move || websocket_handler(r));
    s
});

/// Queue for received messages from the web interface
pub static GLOBAL_EVENTS: Lazy<Mutex<HashMap<Vec<String>, EventTypes>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

type Lam<T> = Lazy<Arc<Mutex<T>>>;
type BFn = Box<dyn Fn() + Send + Sync>;
pub static GLOBAL_CLICK_CALLBACKS: Lam<HashMap<Vec<String>, BFn>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Binds a TcpListener to a port in the specified range, if possible.
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

/// Handler for sending/receiving from the web interface
fn websocket_handler(r: Receiver<SendTypes>) {
    let websocket_port_range = PORT_RANGE.get_or_init(|| DEFAULT_PORT_RANGE);
    let (server, _server_port) =
        bind_in_range(websocket_port_range.0, websocket_port_range.1).unwrap();

    for stream in server.incoming() {
        let stream = stream.unwrap();

        let mut websocket = tungstenite::accept(stream).unwrap();

        // Hello message expected from web interface upon connection
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

/// Handler for receiving data from the web interface
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
                        if GLOBAL_CLICK_CALLBACKS
                            .lock()
                            .unwrap()
                            .contains_key(event.path())
                        {
                            GLOBAL_SENDER
                                .send(SendTypes::Handled(HandledPacket::new(event.path().clone())))
                                .unwrap();

                            let path = event.path().clone();
                            thread::spawn(move || {
                                if let Some(f) = GLOBAL_CLICK_CALLBACKS.lock().unwrap().get(&path) {
                                    f();
                                };
                            });
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

/// The range of ports that can be used by websockets - this can only be set once, after which it is
/// read-only to prevent a web interface communicating on one set of ports that the library is
/// ignoring
pub static PORT_RANGE: OnceLock<(u16, u16)> = OnceLock::new();
