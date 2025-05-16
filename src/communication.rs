use crate::communication_backend::{GLOBAL_CLICK_CALLBACKS, GLOBAL_EVENTS, GLOBAL_SENDER};
use crate::interface::{
    ButtonWidget, DeletePacket, Widget, WidgetPacket, EventTypes, HandledPacket, NodeWidget, ProgressWidget,
    SendTypes, TextWidget,
};

/// Create/update a node widget on the web interface
pub fn send_node<V: AsRef<[S]>, S: AsRef<str>>(path: V, card: bool) {
    GLOBAL_SENDER
        .send(SendTypes::Widget(WidgetPacket::new(
            path.as_ref()
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<String>>(),
            Widget::Node(NodeWidget::new(card)),
        )))
        .unwrap();
}

/// Create/update a text widget on the web interface
pub fn send_text<V: AsRef<[S]>, S: AsRef<str>, S2: AsRef<str>>(path: V, text: S2, card: bool) {
    GLOBAL_SENDER
        .send(SendTypes::Widget(WidgetPacket::new(
            path.as_ref()
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<String>>(),
            Widget::Text(TextWidget::new(text.as_ref().to_string(), card)),
        )))
        .unwrap();
}

/// Create/update a progress widget on the web interface
pub fn send_progress<V: AsRef<[S]>, S: AsRef<str>, S2: AsRef<str>>(
    path: V,
    text: Option<S2>,
    progress: f32,
    card: bool,
) {
    GLOBAL_SENDER
        .send(SendTypes::Widget(WidgetPacket::new(
            path.as_ref()
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<String>>(),
            Widget::Progress(ProgressWidget::new(
                text.map(|s| s.as_ref().to_string()),
                progress,
                card,
            )),
        )))
        .unwrap();
}

/// Create/update a button on the web interface. `callback` will be called every time the button on
/// the web interface is clicked
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
        .send(SendTypes::Widget(WidgetPacket::new(
            path.clone(),
            Widget::Button(ButtonWidget::new(text.as_ref().to_string(), card)),
        )))
        .unwrap();
}

/// Create/update a button on the web interface. This function will return true once when a button
/// with the specified path is clicked. This function is recommended to be called in a loop.
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
        .send(SendTypes::Widget(WidgetPacket::new(
            path.clone(),
            Widget::Button(ButtonWidget::new(text.as_ref().to_string(), card)),
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

/// Deletes a specified widget from the web interface
pub fn delete_widget<V: AsRef<[S]>, S: AsRef<str>>(path: V) {
    GLOBAL_SENDER
        .send(SendTypes::Delete(DeletePacket::new(
            path.as_ref()
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<String>>(),
        )))
        .unwrap();
}

/// Delete all widgets on the web interface
pub fn clear_widgets() {
    GLOBAL_SENDER.send(SendTypes::Clear).unwrap()
}
