mod interface;
mod webpage;
mod communication;
mod communication_backend;

pub use webpage::serve_webpage;
pub use webpage::serve_webpage_at_port;
pub use webpage::set_port_range;

pub use communication::delete_element;
pub use communication::send_node;
pub use communication::send_text;
pub use communication::clear;
