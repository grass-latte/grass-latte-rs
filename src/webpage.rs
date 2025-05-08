use crate::communication_backend::{DEFAULT_PORT_RANGE, PORT_RANGE};
use std::thread;
use tiny_http::{Header, Response, Server};

const DEFAULT_WEB_PORT: u16 = 8080;

pub fn set_port_range(range: (u16, u16)) {
    if let Err(e) = PORT_RANGE.set(range) {
        panic!(
            "Port range already set to {}-{}. Did you use any grass-latte functions before calling set_port_range?",
            e.0, e.1
        );
    }
}

pub fn serve_webpage() {
    serve_webpage_at_port(DEFAULT_WEB_PORT);
}
pub fn serve_webpage_at_port(web_port: u16) {
    let websocket_port_range = PORT_RANGE.get_or_init(|| DEFAULT_PORT_RANGE);

    thread::spawn(move || {
        let mut html = String::from(include_str!("index.html"));
        const SEARCHING_FOR: &str =
            "<div id=\"port-marker-he9RYeXH5Psd7vcKOzWs\" style=\"display: none;\">";
        let pos = html.rfind(SEARCHING_FOR).unwrap() + SEARCHING_FOR.len();
        html.insert_str(
            pos,
            &format!("{}-{}", websocket_port_range.0, websocket_port_range.1),
        );

        let server_address = format!("127.0.0.1:{}", web_port);
        let server = Server::http(&server_address).unwrap();
        println!("Serving on http://{server_address}");

        for request in server.incoming_requests() {
            let response = Response::from_string(&html)
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap());
            request.respond(response).unwrap();
        }
    });
}
