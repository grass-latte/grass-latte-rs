mod communication;
mod communication_backend;
mod interface;
mod webpage;

pub use webpage::serve_webpage;
pub use webpage::serve_webpage_at_port;
pub use webpage::set_port_range;

pub use communication::clear;
pub use communication::delete_element;
pub use communication::poll_button;
pub use communication::send_button_with_callback;
pub use communication::send_node;
pub use communication::send_progress;
pub use communication::send_text;
