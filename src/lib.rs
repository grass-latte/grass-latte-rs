use std::net::TcpListener;
use std::thread;
use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use tiny_http::{Header, Response, Server};

pub fn serve_webpage() {
    thread::spawn(|| {
        let mut html = String::from(include_str!("index.html"));
        const SEARCHING_FOR: &str = "<div id=\"port-marker-he9RYeXH5Psd7vcKOzWs\" style=\"display: none;\">";
        let pos = html.rfind(SEARCHING_FOR).unwrap() + SEARCHING_FOR.len();
        html.insert_str(pos, "3030");
        
        let server = Server::http("0.0.0.0:8080").unwrap();
        println!("Serving on http://0.0.0.0:8080");

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

fn sender(r: Receiver<SendTypes>) {
    let server = TcpListener::bind("127.0.0.1:3030").unwrap();
    println!("Blocking WebSocket server running on ws://localhost:3030");

    for stream in server.incoming() {
        let stream = stream.unwrap();

        // Accept the WebSocket handshake
        let mut websocket = tungstenite::accept(stream).unwrap();

        for message in r.iter() {
            match message {
                SendTypes::Alpha => { println!("Alpha") }
                SendTypes::Bravo => { println!("Bravo") }
                SendTypes::Charlie(c) => { println!("{c}") }
            }

            websocket.send("Example".into()).unwrap();
        }
    }
}