use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

/// Highest level of packet type sendable to the interface
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum SendTypes {
    #[serde(rename = "widget")]
    Widget(WidgetPacket),
    #[serde(rename = "delete")]
    Delete(DeletePacket),
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "handled")]
    Handled(HandledPacket),
}

/// Delete a widget
#[derive(Debug, Serialize, new)]
pub struct DeletePacket {
    path: Vec<String>,
}

/// Confirm that an event has been handled (e.g. button click)
#[derive(Debug, Serialize, new)]
pub struct HandledPacket {
    path: Vec<String>,
}

/// Send a widget to the web interface
#[derive(Debug, Serialize, new)]
pub struct WidgetPacket {
    path: Vec<String>,
    widget: Widget,
}

/// Types of widgets available
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Widget {
    #[serde(rename = "node")]
    Node(NodeWidget),
    #[serde(rename = "text")]
    Text(TextWidget),
    #[serde(rename = "progress")]
    Progress(ProgressWidget),
    #[serde(rename = "button")]
    Button(ButtonWidget),
}

#[derive(Debug, Serialize, new)]
pub struct NodeWidget {
    card: bool,
}

#[derive(Debug, Serialize, new)]
pub struct TextWidget {
    text: String,
    card: bool,
}

#[derive(Debug, Serialize, new)]
pub struct ProgressWidget {
    text: Option<String>,
    progress: f32,
    card: bool,
}

#[derive(Debug, Serialize, new)]
pub struct ButtonWidget {
    text: String,
    card: bool,
}

/// Event receivable from the web interface
#[derive(Deserialize, Getters)]
pub struct Event {
    path: Vec<String>,
    data: EventTypes,
}

impl Event {
    pub fn into_data(self) -> EventTypes {
        self.data
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventTypes {
    #[serde(rename = "click")]
    Click(Click),
}

#[derive(Deserialize)]
pub struct Click;
