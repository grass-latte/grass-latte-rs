use crate::{DEFAULT_PORT_RANGE, PORT_RANGE};
use crossbeam::channel::{Receiver, Sender, unbounded};
use derive_new::new;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::net::{SocketAddr, TcpListener};
use std::{io, thread};
use tungstenite::Message;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum SendTypes {
    #[serde(rename = "element")]
    Element(ElementPacket),
    #[serde(rename = "delete")]
    Delete(DeletePacket),
}

#[derive(Debug, Serialize, new)]
pub struct DeletePacket {
    path: Vec<String>,
}

#[derive(Debug, Serialize, new)]
pub struct ElementPacket {
    path: Vec<String>,
    element: Element,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Element {
    #[serde(rename = "node")]
    Node(Node),
    #[serde(rename = "text")]
    Text(Text),
}

#[derive(Debug, Serialize)]
pub struct Node;

#[derive(Debug, Serialize, new)]
pub struct Text {
    text: String,
}

pub static GLOBAL_SENDER: Lazy<Sender<SendTypes>> = Lazy::new(|| {
    let (s, r) = unbounded();
    thread::spawn(move || sender(r));
    s
});

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

        for message in r.iter() {
            let json = serde_json::to_string(&message).unwrap();
            websocket.send(json.into()).unwrap();
        }
    }
}
