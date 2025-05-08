use crate::communication_backend::GLOBAL_SENDER;
use crate::interface::{DeletePacket, Element, ElementPacket, Node, SendTypes, Text};

pub fn send_node<V: AsRef<[S]>, S: AsRef<str>>(path: V) {
    GLOBAL_SENDER
        .send(SendTypes::Element(ElementPacket::new(
            path.as_ref()
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<String>>(),
            Element::Node(Node),
        )))
        .unwrap();
}

pub fn send_text<V: AsRef<[S]>, S: AsRef<str>>(path: V, text: S) {
    GLOBAL_SENDER
        .send(SendTypes::Element(ElementPacket::new(
            path.as_ref()
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<String>>(),
            Element::Text(Text::new(text.as_ref().to_string())),
        )))
        .unwrap();
}

pub fn delete_element<V: AsRef<[S]>, S: AsRef<str>>(path: V) {
    GLOBAL_SENDER
        .send(SendTypes::Delete(DeletePacket::new(
            path.as_ref()
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<String>>(),
        )))
        .unwrap();
}

pub fn clear() {
    GLOBAL_SENDER.send(SendTypes::Clear).unwrap()
}
