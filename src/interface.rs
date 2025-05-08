use derive_new::new;
use serde::Serialize;
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum SendTypes {
    #[serde(rename = "element")]
    Element(ElementPacket),
    #[serde(rename = "delete")]
    Delete(DeletePacket),
    #[serde(rename = "clear")]
    Clear,
}

#[derive(Debug, Serialize, new)]
pub struct DeletePacket {
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
