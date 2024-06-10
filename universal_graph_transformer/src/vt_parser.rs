use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use crate::types::{Node, Edge};

pub fn parse_vt_json(json_str: &str) -> Result<(Vec<Node>, Vec<Edge>), Box<dyn Error>> {
    let json_value: Value = serde_json::from_str(json_str)?;

    // Initialize vectors for nodes and edges
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Assume the provided JSON has specific keys. This should be adapted according to the actual JSON structure.
    if let Some(graph) = json_value.get("graph") {
        // Parse nodes
        if let Some(nodes_array) = graph.get("nodes") {
            for node in nodes_array.as_array().unwrap() {
                let id = node.get("id").unwrap().as_str().unwrap().to_string();
                let label = node.get("label").unwrap().as_str().unwrap().to_string();
                let node_type = node.get("type").unwrap().as_str().unwrap().to_string();
                let properties = HashMap::new(); // Populate with actual properties from JSON

                nodes.push(Node { id, label, node_type, properties });
            }
        }

        // Parse edges
        if let Some(edges_array) = graph.get("edges") {
            for edge in edges_array.as_array().unwrap() {
                let source = edge.get("source").unwrap().as_str().unwrap().to_string();
                let target = edge.get("target").unwrap().as_str().unwrap().to_string();
                let relation_type = edge.get("type").unwrap().as_str().unwrap().to_string();
                let properties = HashMap::new(); // Populate with actual properties from JSON

                edges.push(Edge { source, target, relation_type, properties });
            }
        }
    }

    Ok((nodes, edges))
}