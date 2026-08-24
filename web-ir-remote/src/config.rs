use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Controllers {
    #[serde(rename = "$value")]
    pub controllers: Vec<Controller>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Controller {
    #[serde(rename = "$value")]
    pub root_node: Node,

    #[allow(dead_code)]
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Button {
    #[allow(dead_code)]
    pub label: String,

    pub action: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VBox {
    #[serde(rename = "$value")]
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HBox {
    #[serde(rename = "$value")]
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Node {
    Button(Button),
    HBox(HBox),
    VBox(VBox),
}
