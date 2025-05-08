use crate::communication_backend::{GLOBAL_CLICK_CALLBACKS, GLOBAL_EVENTS, GLOBAL_SENDER};
use crate::interface::{
    Button, DeletePacket, Element, ElementPacket, EventTypes, HandledPacket, Node, Progress,
    SendTypes, Text,
};

pub fn send_node<V: AsRef<[S]>, S: AsRef<str>>(path: V, card: bool) {
    GLOBAL_SENDER
        .send(SendTypes::Element(ElementPacket::new(
            path.as_ref()
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<String>>(),
            Element::Node(Node::new(card)),
        )))
        .unwrap();
}

pub fn send_text<V: AsRef<[S]>, S: AsRef<str>, S2: AsRef<str>>(path: V, text: S2, card: bool) {
    GLOBAL_SENDER
        .send(SendTypes::Element(ElementPacket::new(
            path.as_ref()
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<String>>(),
            Element::Text(Text::new(text.as_ref().to_string(), card)),
        )))
        .unwrap();
}

pub fn send_progress<V: AsRef<[S]>, S: AsRef<str>, S2: AsRef<str>>(
    path: V,
    text: Option<S2>,
    progress: f32,
    card: bool,
) {
    GLOBAL_SENDER
        .send(SendTypes::Element(ElementPacket::new(
            path.as_ref()
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<String>>(),
            Element::Progress(Progress::new(
                text.map(|s| s.as_ref().to_string()),
                progress,
                card,
            )),
        )))
        .unwrap();
}

pub fn send_button_with_callback<
    V: AsRef<[S]>,
    S: AsRef<str>,
    S2: AsRef<str>,
    F: Fn() + Send + Sync + 'static,
>(
    path: V,
    text: S2,
    card: bool,
    callback: F,
) {
    let path = path
        .as_ref()
        .iter()
        .map(|s| s.as_ref().to_string())
        .collect::<Vec<String>>();

    GLOBAL_CLICK_CALLBACKS
        .lock()
        .unwrap()
        .insert(path.clone(), Box::new(callback));

    GLOBAL_SENDER
        .send(SendTypes::Element(ElementPacket::new(
            path.clone(),
            Element::Button(Button::new(text.as_ref().to_string(), card)),
        )))
        .unwrap();
}

pub fn poll_button<V: AsRef<[S]>, S: AsRef<str>, S2: AsRef<str>>(
    path: V,
    text: S2,
    card: bool,
) -> bool {
    let path = path
        .as_ref()
        .iter()
        .map(|s| s.as_ref().to_string())
        .collect::<Vec<String>>();

    GLOBAL_SENDER
        .send(SendTypes::Element(ElementPacket::new(
            path.clone(),
            Element::Button(Button::new(text.as_ref().to_string(), card)),
        )))
        .unwrap();

    let Some(event) = GLOBAL_EVENTS.lock().unwrap().remove(&path) else {
        return false;
    };
    match event {
        EventTypes::Click(_) => {
            GLOBAL_SENDER
                .send(SendTypes::Handled(HandledPacket::new(path.clone())))
                .unwrap();
            true
        }
        #[allow(unreachable_patterns)]
        _ => false,
    }
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
