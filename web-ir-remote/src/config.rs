use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Controllers {
    #[serde(rename = "Controller", default)]
    pub controllers: Vec<Controller>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Controller {
    #[serde(rename = "#content")]
    pub root_node: Node,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Button {
    #[serde(rename = "@label")]
    #[allow(dead_code)]
    pub label: String,

    #[serde(rename = "@action")]
    pub action: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VBox {
    #[serde(rename = "#content")]
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HBox {
    #[serde(rename = "#content")]
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Node {
    Button(Button),
    HBox(HBox),
    VBox(VBox),
}
