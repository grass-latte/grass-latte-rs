use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum SendTypes {
    #[serde(rename = "element")]
    Element(ElementPacket),
    #[serde(rename = "delete")]
    Delete(DeletePacket),
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "handle")]
    Handle(HandlePacket),
}

#[derive(Debug, Serialize, new)]
pub struct DeletePacket {
    path: Vec<String>,
}

#[derive(Debug, Serialize, new)]
pub struct HandlePacket {
    path: Vec<String>,
}

#[derive(Debug, Serialize, new)]
pub struct ElementPacket {
    path: Vec<String>,
    element: Element,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Element {
    #[serde(rename = "node")]
    Node(Node),
    #[serde(rename = "text")]
    Text(Text),
    #[serde(rename = "progress")]
    Progress(Progress),
    #[serde(rename = "button")]
    Button(Button),
}

#[derive(Debug, Serialize, new)]
pub struct Node {
    card: bool,
}

#[derive(Debug, Serialize, new)]
pub struct Text {
    text: String,
    card: bool,
}

#[derive(Debug, Serialize, new)]
pub struct Progress {
    text: Option<String>,
    progress: f32,
    card: bool,
}

#[derive(Debug, Serialize, new)]
pub struct Button {
    text: String,
    card: bool,
}

#[derive(Deserialize, Getters)]
pub struct Event {
    path: Vec<String>,
    data: EventTypes
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
