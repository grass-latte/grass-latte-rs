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
    Clear
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
}

#[derive(Debug, Serialize)]
pub struct Node;

#[derive(Debug, Serialize, new)]
pub struct Text {
    text: String,
}
