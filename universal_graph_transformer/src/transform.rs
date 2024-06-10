use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::error::Error;
use crate::graphml_parser::{Node, Edge, Graph, parse_graphml};

pub fn transform_graph(filename: &str, mode: &str) -> Result<(), Box<dyn Error>> {
    let (nodes, edges) = if mode == "auto" || mode == "graphml" || mode == "maltego" {
        parse_graphml(filename)?
    } else {
        return Err("Unsupported mode".into());
    };

    let lean_nodes: Vec<Node> = nodes
        .iter()
        .map(|node| Node {
            id: node.id.clone(),
            label: node.label.clone(),
            node_type: node.node_type.clone(),
            properties: HashMap::new(),
        })
        .collect();

    let lean_edges: Vec<Edge> = edges
        .iter()
        .map(|edge| Edge {
            source: edge.source.clone(),
            target: edge.target.clone(),
            relation_type: edge.relation_type.clone(),
            properties: HashMap::new(),
        })
        .collect();

    let lean_graph = Graph {
        nodes: lean_nodes,
        edges: lean_edges,
    };

    let rich_graph = Graph { nodes, edges };

    save_json(&lean_graph, "lean.json")?;
    save_json(&rich_graph, "rich.json")?;

    Ok(())
}

fn save_json<T: Serialize>(data: &T, filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data)?;
    Ok(())
}