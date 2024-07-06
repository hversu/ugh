use serde::{Deserialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use crate::input_type::is_file;
use crate::types::{Node, Edge, Properties};

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
    let data: String;

    if is_file(filename) {
        let mut file = File::open(&filename)?;
        let mut file_data = String::new();
        file.read_to_string(&mut file_data)?;
        data = file_data;
    } else { // direct data
        data = filename.to_string();
    }
    println!("Data: {}", &data);
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
        id: Properties::get_id_from_str(&threat_query),
        label: threat_query.clone(),
        node_type: "threat".to_string(),
        properties: Properties::map_values(threat_props)
    });

    if let Some(activity) = json_input.activity_and_relationships {
        if let Some(communicating_files) = activity.related_items.communicating_files {
            for file in communicating_files {
                nodes.push(Node {
                    id: Properties::get_id_from_str(&file),
                    label: "Communicating File".to_string(),
                    node_type: "file".to_string(),
                    properties: Properties::new(),
                });

                edges.push(Edge {
                    source: threat_query.clone(),
                    target: file,
                    relation_type: "communicates_with".to_string(),
                    properties: Properties::new(),
                });
            }
        }

        if let Some(contacted_ips) = activity.related_items.contacted_ips {
            for ip in contacted_ips {
                nodes.push(Node {
                    id: Properties::get_id_from_str(&ip),
                    label: "Contacted IP".to_string(),
                    node_type: "ip".to_string(),
                    properties: Properties::new(),
                });

                edges.push(Edge {
                    source: threat_query.clone(),
                    target: ip,
                    relation_type: "contacted".to_string(),
                    properties: Properties::new(),
                });
            }
        }

        if let Some(contacted_domains) = activity.related_items.contacted_domains {
            for domain in contacted_domains {
                nodes.push(Node {
                    id: Properties::get_id_from_str(&domain),
                    label: "Contacted Domain".to_string(),
                    node_type: "domain".to_string(),
                    properties: Properties::new(),
                });

                edges.push(Edge {
                    source: threat_query.clone(),
                    target: domain,
                    relation_type: "contacted".to_string(),
                    properties: Properties::new(),
                });
            }
        }

        if let Some(resolutions) = activity.related_items.resolves_to {
            for resolution in resolutions {
                nodes.push(Node {
                    id: Properties::get_id_from_str(&resolution.domain),
                    label: "Resolved Domain".to_string(),
                    node_type: "domain".to_string(),
                    properties: Properties::new(),
                });
                nodes.push(Node {
                    id: Properties::get_id_from_str(&resolution.ip),
                    label: "Resolved IP".to_string(),
                    node_type: "ip".to_string(),
                    properties: Properties::new(),
                });
                edges.push(Edge {
                    source: resolution.domain.clone(),
                    target: resolution.ip.clone(),
                    relation_type: "resolves_to".to_string(),
                    properties: Properties::new(),
                });
            }
        }

        if let Some(dns_records) = activity.dns {
            for dns_record in dns_records {
                nodes.push(Node {
                    id: Properties::get_id_from_str(&dns_record.value),
                    label: dns_record.record_type.clone(),
                    node_type: "dns".to_string(),
                    properties: Properties::new(),
                });

                edges.push(Edge {
                    source: threat_query.clone(),
                    target: dns_record.value.clone(),
                    relation_type: "has_dns".to_string(),
                    properties: Properties::new(),
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
        let result = parse_vt_json("example_data/vt.json");
        assert_eq!(result.is_ok(), false);
    }
}