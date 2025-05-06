use std::net::{SocketAddr, TcpListener};
use std::sync::OnceLock;
use std::{io, thread};
use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use serde::Serialize;
use tiny_http::{Header, Response, Server};
use tungstenite::Message;

const DEFAULT_WEB_PORT: u16 = 8080;
const DEFAULT_PORT_RANGE: (u16, u16) = (3030, 3030);

pub static PORT_RANGE: OnceLock<(u16, u16)> = OnceLock::new();

pub fn set_port_range(range: (u16, u16)) {
    if let Err(e) = PORT_RANGE.set(range) {
        panic!("Port range already set to {}-{}. Did you use any grass-latte functions before calling set_port_range?", e.0, e.1);
    }
}

pub fn serve_webpage() {
    serve_webpage_with_port(DEFAULT_WEB_PORT);
}
pub fn serve_webpage_with_port(web_port: u16) {
    let websocket_port_range = PORT_RANGE.get_or_init(|| DEFAULT_PORT_RANGE);

    thread::spawn(move || {
        let mut html = String::from(include_str!("index.html"));
        const SEARCHING_FOR: &str = "<div id=\"port-marker-he9RYeXH5Psd7vcKOzWs\" style=\"display: none;\">";
        let pos = html.rfind(SEARCHING_FOR).unwrap() + SEARCHING_FOR.len();
        html.insert_str(pos, &format!("{}-{}", websocket_port_range.0, websocket_port_range.1));

        let server_address = format!("127.0.0.1:{}", web_port);
        let server = Server::http(&server_address).unwrap();
        println!("Serving on http://{server_address}");

        for request in server.incoming_requests() {

            let response = Response::from_string(&html).with_header(
                Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap()
            );
            request.respond(response).unwrap();
        }
    });
}

pub fn send_node<V: AsRef<[S]>, S: AsRef<str>>(path: V) {
    GLOBAL_SENDER.send(SendTypes::Element(
        ElementPacket {
            path: path.as_ref().iter().map(|s| s.as_ref().to_string()).collect::<Vec<String>>(),
            element: Element::Node(Node)
        }
    )).unwrap();
}

pub fn delete_element<V: AsRef<[S]>, S: AsRef<str>>(path: V) {
    GLOBAL_SENDER.send(SendTypes::Delete(
        DeletePacket {
            path: path.as_ref().iter().map(|s| s.as_ref().to_string()).collect::<Vec<String>>()
        }
    )).unwrap();
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
enum SendTypes {
    #[serde(rename = "element")]
    Element(ElementPacket),
    #[serde(rename = "delete")]
    Delete(DeletePacket),
}

#[derive(Debug, Serialize)]
struct DeletePacket {
    path: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ElementPacket {
    path: Vec<String>,
    element: Element,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
enum Element {
    #[serde(rename = "node")]
    Node(Node)
}

#[derive(Debug, Serialize)]
struct Node;

static GLOBAL_SENDER: Lazy<Sender<SendTypes>> = Lazy::new(|| {
    let (s, r) = unbounded();
    thread::spawn(move || {
        sender(r)
    });
    s
});

fn bind_in_range(start_port: u16, end_port: u16) -> io::Result<(TcpListener, u16)> {
    let addrs: Vec<SocketAddr> = (start_port..=end_port).map(|i| SocketAddr::from(([127, 0, 0, 1], i))).collect();
    match TcpListener::bind(addrs.as_slice()) {
        Ok(listener) => {
            let port = listener.local_addr()?.port();
            Ok((listener, port))
        },
        Err(e) => Err(e), // Try the next port
    }
}


fn sender(r: Receiver<SendTypes>) {
    let websocket_port_range = PORT_RANGE.get_or_init(|| DEFAULT_PORT_RANGE);
    let (server, _server_port) = bind_in_range(websocket_port_range.0, websocket_port_range.1).unwrap();

    for stream in server.incoming() {
        let stream = stream.unwrap();

        let mut websocket = tungstenite::accept(stream).unwrap();

        let hello = websocket.read().unwrap();

        // println!("{:?}", hello); // Wait for hello (webpage is ready)
        
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