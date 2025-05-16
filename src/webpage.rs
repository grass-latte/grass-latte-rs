use crate::communication_backend::{DEFAULT_PORT_RANGE, PORT_RANGE};
use std::net::TcpListener;
use std::thread;
use tiny_http::{Header, Response, Server};

const DEFAULT_WEB_PORT: u16 = 8080;

/// Changest the range of ports used for websockets. Can only be set once and must be set before the
/// ports are used.
///
/// This must be aligned with other projects, should you want multiple to connect to the same web
/// interface. The amount of ports available in the range dictates the maximum number of connection
/// to the web interface, but can also slow web interface discovery of new connections.
pub fn set_port_range(range: (u16, u16)) {
    if let Err(e) = PORT_RANGE.set(range) {
        panic!(
            "Port range already set to {}-{}. Did you use any grass-latte functions before calling set_port_range?",
            e.0, e.1
        );
    }
}

/// Serve the web interface on any port
pub fn serve_webpage() {
    internal_serve_webpage(DEFAULT_WEB_PORT, true);
}

/// Serve the web interface at a specific port
pub fn serve_webpage_at_port(web_port: u16) {
    internal_serve_webpage(web_port, false);
}

/// Serve the web interface at a target port which can be changed, should the target be in use
pub fn serve_webpage_at_port_flexible(target_web_port: u16) {
    internal_serve_webpage(target_web_port, true);
}

fn internal_serve_webpage(web_port: u16, allow_move: bool) {
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

        let web_port = if allow_move {
            let (a, b) = (web_port, web_port + 1000);
            (a..=b)
                .find(|port| {
                    if let Ok(tcp) = TcpListener::bind(("127.0.0.1", *port)) {
                        drop(tcp);
                        true
                    } else {
                        false
                    }
                })
                .unwrap_or_else(|| panic!("No available ports found to serve webpage in the range {a}-{b}"))
        } else {
            web_port
        };

        let server_address = format!("127.0.0.1:{}", web_port);
        let server = Server::http(&server_address)
            .unwrap_or_else(|_| panic!("Failed to server webpage at {server_address}"));
        println!("Serving on http://{server_address}");

        for request in server.incoming_requests() {
            let response = Response::from_string(&html)
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap());
            request.respond(response).unwrap();
        }
    });
}
