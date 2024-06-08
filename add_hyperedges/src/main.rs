use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use serde::de::DeserializeOwned;

// Existing structs
#[derive(Serialize, Deserialize, Debug)]
struct Node {
    id: String,
    label: String,
    #[serde(rename = "type")]
    node_type: String,
    properties: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Edge {
    source: String,
    target: String,
    relation_type: String,
    properties: HashMap<String, String>,
}

// New hyperedge struct
#[derive(Serialize, Deserialize, Debug)]
struct Hyperedge {
    label: String,
    #[serde(rename = "type")]
    edge_type: String, // This will include the property field name
    hypertype: String, // This will be either "superedge" or "node_property"
    value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    #[serde(default)]
    hyperedges: Vec<Hyperedge>, // Make hyperedges optional during deserialization
}

fn load_json<T: DeserializeOwned>(filename: &str) -> Result<T, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

fn save_json<T: Serialize>(data: &T, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data)?;
    Ok(())
}

fn add_hyperedges(graph: &mut Graph, filter_property: Option<String>) {
    let mut node_prop_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut edge_prop_map: HashMap<String, Vec<String>> = HashMap::new();

    // Collecting node properties
    for node in &graph.nodes {
        for (key, value) in &node.properties {
            let property_id = format!("{}::{}", key, value);
            node_prop_map.entry(property_id).or_insert_with(Vec::new).push(node.id.clone());
        }
    }

    // Collecting edge properties
    for edge in &graph.edges {
        for (key, value) in &edge.properties {
            let property_id = format!("{}::{}", key, value);
            edge_prop_map.entry(property_id).or_insert_with(Vec::new).push(edge.source.clone() + &edge.target);
        }
    }

    // Adding hyperedges based on node properties
    for (property, nodes) in node_prop_map {
        if nodes.len() > 1 {
            let parts: Vec<&str> = property.split("::").collect();
            let key = parts[0];
            let value = parts[1];
            if filter_property.as_deref() == Some(value) || filter_property.is_none() {
                graph.hyperedges.push(Hyperedge {
                    label: value.to_string(),
                    edge_type: key.to_string(),
                    hypertype: "node_property".to_string(),
                    value: nodes,
                });
            }
        }
    }

    // Adding super edges based on edge properties
    for (property, edges) in edge_prop_map {
        if edges.len() > 1 {
            let parts: Vec<&str> = property.split("::").collect();
            let key = parts[0];
            let value = parts[1];
            if filter_property.as_deref() == Some(value) || filter_property.is_none() {
                graph.hyperedges.push(Hyperedge {
                    label: value.to_string(),
                    edge_type: key.to_string(),
                    hypertype: "superedge".to_string(),
                    value: edges,
                });
            }
        }
    }
}

fn transform_graph(filename: &str, filter_property: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut graph: Graph = load_json(filename)?;

    add_hyperedges(&mut graph, filter_property);

    save_json(&graph, "graph_with_hyperedges.json")?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [property_value]", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let filter_property = if args.len() == 3 {
        Some(args[2].clone())
    } else {
        None
    };

    match transform_graph(filename, filter_property) {
        Ok(_) => println!("Transformation with hyperedges successful!"),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}
