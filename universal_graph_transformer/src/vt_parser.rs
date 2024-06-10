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
    whois: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
struct Threat {
    query: String,
}

#[derive(Deserialize, Debug)]
struct ActivityAndRelationships {
    related_items: RelatedItems,
    dns: Option<Vec<DnsRecord>>,
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

#[derive(Deserialize, Debug)]
struct DnsRecord {
    #[serde(rename = "type")]
    record_type: String,
    value: String,
}

pub fn parse_vt_json(filename: &str) -> Result<(Vec<Node>, Vec<Edge>), Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let json_input: JsonInput = serde_json::from_str(&data)?;

    let threat_query = json_input.identity_and_verdict.threat.query;
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Add the threat node with properties
    let mut threat_props = HashMap::new();
    if let Some(whois) = &json_input.identity_and_verdict.whois {
        for (key, value) in whois {
            threat_props.insert(key.clone(), value.clone());
        }
    }
    nodes.push(Node {
        id: threat_query.clone(),
        label: threat_query.clone(),
        node_type: "threat".to_string(),
        properties: threat_props,
    });

    if let Some(activity) = json_input.activity_and_relationships {
        if let Some(communicating_files) = activity.related_items.communicating_files {
            for file in communicating_files {
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

        if let Some(resolutions) = activity.related_items.resolves_to {
            for resolution in resolutions {
                nodes.push(Node {
                    id: resolution.domain.clone(),
                    label: "Resolved Domain".to_string(),
                    node_type: "domain".to_string(),
                    properties: HashMap::new(),
                });
                nodes.push(Node {
                    id: resolution.ip.clone(),
                    label: "Resolved IP".to_string(),
                    node_type: "ip".to_string(),
                    properties: HashMap::new(),
                });
                edges.push(Edge {
                    source: resolution.domain.clone(),
                    target: resolution.ip.clone(),
                    relation_type: "resolves_to".to_string(),
                    properties: HashMap::new(),
                });
            }
        }

        if let Some(dns_records) = activity.dns {
            for dns_record in dns_records {
                nodes.push(Node {
                    id: dns_record.value.clone(),
                    label: dns_record.record_type.clone(),
                    node_type: "dns".to_string(),
                    properties: HashMap::new(),
                });

                edges.push(Edge {
                    source: threat_query.clone(),
                    target: dns_record.value.clone(),
                    relation_type: "has_dns".to_string(),
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
    fn test_parse_vt_json() {
        let result = parse_vt_json("example.json");
        assert!(result.is_ok());

        let (nodes, edges) = result.unwrap();
        assert!(!nodes.is_empty());
        assert!(!edges.is_empty());
    }
}