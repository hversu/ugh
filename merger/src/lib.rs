use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: Option<i32>,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge {
    pub source: String,
    pub target: String,
    pub relation_type: String,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Properties {
    pub id: Option<i32>,
    #[serde(flatten)]
    pub other: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub fn merge_graphs(graph1: Graph, graph2: Graph) -> Graph {
    let mut node_map: HashMap<String, Node> = HashMap::new();
    let mut merged_edges: Vec<Edge> = Vec::new();

    let add_or_merge_node = |node: Node, map: &mut HashMap<String, Node>| {
        map.entry(node.label.clone()).and_modify(|existing_node| {
            // Merge properties
            existing_node.properties.other.extend(node.properties.other.clone());

            // Update node type and id if necessary
            if node.id.is_some() {
                existing_node.id = node.id;
            }
            if !node.node_type.is_empty() {
                existing_node.node_type = node.node_type.clone();
            }
        }).or_insert(node);
    };

    for node in graph1.nodes.into_iter().chain(graph2.nodes.into_iter()) {
        add_or_merge_node(node, &mut node_map);
    }

    let node_labels: HashSet<String> = node_map.keys().cloned().collect();

    let mut update_edge = |edge: Edge| {
        if node_labels.contains(&edge.source) && node_labels.contains(&edge.target) {
            merged_edges.push(edge);
        }
    };

    for edge in graph1.edges.into_iter().chain(graph2.edges.into_iter()) {
        update_edge(edge);
    }

    Graph {
        nodes: node_map.into_values().collect(),
        edges: merged_edges,
    }
}

fn main() {
    // Example usage
    let graph1 = Graph {
        nodes: vec![
            Node {
                id: Some(1),
                label: "A".to_string(),
                node_type: "Type1".to_string(),
                properties: Properties {
                    id: Some(1),
                    other: [("key1".to_string(), "value1".to_string())].iter().cloned().collect(),
                },
            },
            Node {
                id: Some(2),
                label: "B".to_string(),
                node_type: "Type2".to_string(),
                properties: Properties {
                    id: Some(2),
                    other: [("key2".to_string(), "value2".to_string())].iter().cloned().collect(),
                },
            },
        ],
        edges: vec![
            Edge {
                source: "A".to_string(),
                target: "B".to_string(),
                relation_type: "connects".to_string(),
                properties: Properties {
                    id: Some(1),
                    other: HashMap::new(),
                },
            },
        ],
    };

    let graph2 = Graph {
        nodes: vec![
            Node {
                id: Some(3),
                label: "A".to_string(),
                node_type: "Type1".to_string(),
                properties: Properties {
                    id: Some(3),
                    other: [("key3".to_string(), "value3".to_string())].iter().cloned().collect(),
                },
            },
            Node {
                id: Some(4),
                label: "C".to_string(),
                node_type: "Type3".to_string(),
                properties: Properties {
                    id: Some(4),
                    other: [("key4".to_string(), "value4".to_string())].iter().cloned().collect(),
                },
            },
        ],
        edges: vec![
            Edge {
                source: "A".to_string(),
                target: "C".to_string(),
                relation_type: "connects".to_string(),
                properties: Properties {
                    id: Some(2),
                    other: HashMap::new(),
                },
            },
        ],
    };

    let merged_graph = merge_graphs(graph1, graph2);
    println!("{:#?}", merged_graph);
}
