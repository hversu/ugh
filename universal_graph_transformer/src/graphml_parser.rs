use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader};
use std::error::Error;

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

pub fn parse_graphml(filename: &str) -> Result<(Vec<Node>, Vec<Edge>), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.trim_text(true);

    if !reader.decoder().encoding().is_ascii_compatible() {
        return Err("Unsupported encoding detected".into());
    }

    let mut buf = Vec::new();
    let mut current_node: Option<Node> = None;
    let mut current_edge: Option<Edge> = None;
    let mut current_property_name: Option<String> = None;

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"node" => {
                        let mut properties = HashMap::new();
                        for attr in e.attributes() {
                            let attr = attr?;
                            let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                            let value = attr.decode_and_unescape_value(&reader)?.to_string();
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
                            let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                            let value = attr.decode_and_unescape_value(&reader)?.to_string();
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
                            if String::from_utf8_lossy(attr.key.as_ref()) == "name" {
                                current_property_name = Some(attr.decode_and_unescape_value(&reader)?.to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                if let Some(property_name) = &current_property_name {
                    if let Some(node) = &mut current_node {
                        let value = e.unescape()?.to_string();
                        node.properties.insert(property_name.clone(), value.clone());

                        // Generalizing node type and label assignment based on observed properties
                        if property_name.starts_with("ipv4-address") || property_name.starts_with("email.address") || property_name.starts_with("domain") {
                            node.label = value.clone();
                            node.node_type = property_name.clone();
                        } else if property_name == "name" || property_name == "label" {
                            node.label = value.clone();
                        }
                    } else if let Some(edge) = &mut current_edge {
                        edge.properties.insert(property_name.clone(), e.unescape()?.to_string());
                    }
                }
            }
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
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