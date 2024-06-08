use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::env;
use std::process;

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

#[derive(Serialize, Deserialize, Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

fn parse_graphml(filename: &str) -> Result<(Vec<Node>, Vec<Edge>), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut current_node: Option<Node> = None;
    let mut current_edge: Option<Edge> = None;
    let mut current_property_name: Option<String> = None;

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"node" => {
                        let mut properties = HashMap::new();
                        for attr in e.attributes() {
                            let attr = attr?;
                            let key = String::from_utf8_lossy(attr.key).to_string();
                            let value = String::from_utf8_lossy(&attr.value).to_string();
                            properties.insert(key.clone(), value.clone());

                            if key == "id" {
                                current_node = Some(Node {
                                    id: value,
                                    label: "Unnamed Node".to_string(),
                                    node_type: "unknown".to_string(),
                                    properties: properties.clone(),
                                });
                            }
                        }
                    }
                    b"edge" => {
                        let mut properties = HashMap::new();
                        let mut source = String::new();
                        let mut target = String::new();
                        let mut relation_type = String::new();

                        for attr in e.attributes() {
                            let attr = attr?;
                            let key = String::from_utf8_lossy(attr.key).to_string();
                            let value = String::from_utf8_lossy(&attr.value).to_string();
                            properties.insert(key.clone(), value.clone());

                            if key == "source" {
                                source = value.clone();
                            }
                            if key == "target" {
                                target = value.clone();
                            }
                            if key == "label" {
                                relation_type = value.clone();
                            }
                        }

                        current_edge = Some(Edge {
                            source,
                            target,
                            relation_type: if relation_type.is_empty() { "linked_to".to_string() } else { relation_type },
                            properties: properties.clone(),
                        });
                    }
                    b"mtg:Property" => {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if String::from_utf8_lossy(attr.key) == "name" {
                                current_property_name = Some(String::from_utf8_lossy(&attr.value).to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                if let Some(property_name) = &current_property_name {
                    if let Some(node) = &mut current_node {
                        let value = e.unescape_and_decode(&reader)?;
                        node.properties.insert(property_name.clone(), value.clone());

                        // Generalizing node type and label assignment based on observed properties
                        if property_name.starts_with("ipv4-address") || property_name.starts_with("email.address") || property_name.starts_with("domain") {
                            node.label = value.clone();
                            node.node_type = property_name.clone();
                        } else if property_name == "name" || property_name == "label" {
                            node.label = value.clone();
                        }
                    } else if let Some(edge) = &mut current_edge {
                        edge.properties.insert(property_name.clone(), e.unescape_and_decode(&reader)?);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"node" => {
                        if let Some(mut node) = current_node.take() {
                            // Default to "unknown" type if not set
                            if node.node_type == "unknown" {
                                for (key, prop) in &node.properties {
                                    if prop.len() > node.label.len() {
                                        node.label = prop.clone();
                                        node.node_type = key.clone();
                                    }
                                }
                            }
                            nodes.push(node);
                        }
                    }
                    b"edge" => {
                        if let Some(edge) = current_edge.take() {
                            if edge.source.is_empty() || edge.target.is_empty() {
                                eprintln!("Warning: Skipping edge with missing source or target. Properties: {:?}", edge.properties);
                            } else {
                                edges.push(edge);
                            }
                        }
                    }
                    _ => {}
                }
                current_property_name = None;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok((nodes, edges))
}

fn save_json<T: Serialize>(data: &T, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data)?;
    Ok(())
}

pub fn transform_graph(filename: &str, mode: &str) -> Result<(), Box<dyn std::error::Error>> {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [mode]", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let mode = if args.len() == 3 {
        &args[2]
    } else {
        "auto"
    };

    match transform_graph(filename, mode) {
        Ok(_) => println!("Transformation successful!"),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}
