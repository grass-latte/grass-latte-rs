mod interface;
mod webpage;
mod communication;

use std::sync::OnceLock;

pub use webpage::serve_webpage;
pub use webpage::serve_webpage_at_port;
pub use webpage::set_port_range;

pub use communication::delete_element;
pub use communication::send_node;
pub use communication::send_text;

const DEFAULT_PORT_RANGE: (u16, u16) = (3030, 3030);
pub static PORT_RANGE: OnceLock<(u16, u16)> = OnceLock::new();
