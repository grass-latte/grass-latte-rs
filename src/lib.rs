use std::net::{SocketAddr, TcpListener};
use std::sync::OnceLock;
use std::{io, thread};
use std::time::{Duration, Instant};
use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use tiny_http::{Header, Response, Server};
use tungstenite::protocol::WebSocketConfig;

const DEFAULT_WEB_PORT: u16 = 8080;
const DEFAULT_PORT_RANGE: (u16, u16) = (3030, 3040);

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

        let server_address = format!("0.0.0.0:{}", web_port);
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

pub fn send(text: SendTypes) {
    GLOBAL_SENDER.send(text).unwrap();
}

#[derive(Debug)]
pub enum SendTypes {
    Alpha,
    Bravo,
    Charlie(String)
}

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

        println!("{:?}", websocket.read()); // Wait for hello (webpage is ready)

        for message in r.iter() {
            match message {
                SendTypes::Alpha => { websocket.send("Alpha".into()).unwrap(); }
                SendTypes::Bravo => { websocket.send("Bravo".into()).unwrap(); }
                SendTypes::Charlie(c) => { websocket.send(c.into()).unwrap(); }
            }
        }
    }
}