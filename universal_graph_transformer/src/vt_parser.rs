use serde::{Deserialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use crate::types::{Node, Edge};

#[derive(Deserialize, Debug)]
struct JsonInput {
    identity_and_verdict: IdentityAndVerdict,
    activity_and_relationships: Option<ActivityAndRelationships>,
}

#[derive(Deserialize, Debug)]
struct IdentityAndVerdict {
    threat: Threat,
}

#[derive(Deserialize, Debug)]
struct Threat {
    query: String,
}

#[derive(Deserialize, Debug)]
struct ActivityAndRelationships {
    related_items: RelatedItems,
}

#[derive(Deserialize, Debug)]
struct RelatedItems {
    communicating_files: Option<Vec<String>>,
    contacted_ips: Option<Vec<String>>,
    contacted_domains: Option<Vec<String>>,
    resolves_to: Option<Vec<ResolveRelationship>>,
}

#[derive(Deserialize, Debug)]
struct ResolveRelationship {
    domain: String,
    ip: String,
}

pub fn parse_vt_json(filename: &str) -> Result<(Vec<Node>, Vec<Edge>), Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // Parse the JSON input
    let json_input: JsonInput = serde_json::from_str(&data)?;

    let threat_query = json_input.identity_and_verdict.threat.query;

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    if let Some(activity) = json_input.activity_and_relationships {
        if let Some(related_items) = activity.related_items.communicating_files {
            for file in related_items {
                nodes.push(Node {
                    id: file.clone(),
                    label: "Communicating File".to_string(),
                    node_type: "file".to_string(),
                    properties: HashMap::new(),
                });

                edges.push(Edge {
                    source: threat_query.clone(),
                    target: file,
                    relation_type: "communicates_with".to_string(),
                    properties: HashMap::new(),
                });
            }
        }

        if let Some(contacted_ips) = activity.related_items.contacted_ips {
            for ip in contacted_ips {
                nodes.push(Node {
                    id: ip.clone(),
                    label: "Contacted IP".to_string(),
                    node_type: "ip".to_string(),
                    properties: HashMap::new(),
                });

                edges.push(Edge {
                    source: threat_query.clone(),
                    target: ip,
                    relation_type: "contacted".to_string(),
                    properties: HashMap::new(),
                });
            }
        }

        if let Some(contacted_domains) = activity.related_items.contacted_domains {
            for domain in contacted_domains {
                nodes.push(Node {
                    id: domain.clone(),
                    label: "Contacted Domain".to_string(),
                    node_type: "domain".to_string(),
                    properties: HashMap::new(),
                });

                edges.push(Edge {
                    source: threat_query.clone(),
                    target: domain,
                    relation_type: "contacted".to_string(),
                    properties: HashMap::new(),
                });
            }
        }

        if let Some(resolves_to) = activity.related_items.resolves_to {
            for resolve in resolves_to {
                edges.push(Edge {
                    source: resolve.domain.clone(),
                    target: resolve.ip.clone(),
                    relation_type: "resolves_to".to_string(),
                    properties: HashMap::new(),
                });
            }
        }
    }

    Ok((nodes, edges))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        let result = parse_json("example.json");
        assert!(result.is_ok());

        let (nodes, edges) = result.unwrap();
        assert!(!nodes.is_empty());
        assert!(!edges.is_empty());
    }
}