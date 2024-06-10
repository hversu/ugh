use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub properties: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    pub source: String,
    pub target: String,
    pub relation_type: String,
    pub properties: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}