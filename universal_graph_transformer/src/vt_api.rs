extern crate reqwest;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate base64;
extern crate chrono;

use reqwest::header::{HeaderMap, HeaderValue};
use regex::Regex;
use serde_json::Value;
use chrono::{Utc, TimeZone};
use std::collections::HashMap;
use std::error::Error;
use base64::encode_config;
use base64::URL_SAFE_NO_PAD;
use crate::mysecret::VTAPI;

#[derive(Debug)]
pub struct VTClient {
    client: reqwest::Client,
    headers: HeaderMap,
    params: HashMap<&'static str, &'static str>,
}

impl VTClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-apikey", HeaderValue::from_static(VTAPI));

        let mut params = HashMap::new();
        params.insert("limit", "10");

        VTClient {
            client: reqwest::Client::new(),
            headers: headers,
            params: params,
        }
    }

    pub async fn call_vt_hal(&self, indicator: &str, item_type: Option<&str>) -> Result<String, Box<dyn Error>> {
        let item_type = match item_type {
            Some(t) => t.to_string(),
            None => self.check_string(indicator),
        };

        if item_type == "unknown" {
            let vt_data = JsonInput {
                identity_and_verdict: IdentityAndVerdict {
                    threat: Threat {
                        query: indicator.to_string(),
                        last_seen: None,
                        label: None,
                        family: None,
                        judgment: None,
                        reputation: None,
                        verdicts: None,
                        jarm: None,
                        tags: None,
                    },
                    whois: None,
                },
                activity_and_relationships: None,
            };

            return Ok(serde_json::to_string(&vt_data)?);
        }

        let vt_data = self.quick_crawl(&item_type, indicator).await?;
        // println!("{:?}",vt_data);
        Ok(serde_json::to_string(&vt_data)?)
    }

    async fn quick_crawl(&self, item_type: &str, id: &str) -> Result<JsonInput, Box<dyn Error>> {
        let relations = self.define_relationships(item_type);
        let id_encoded = if item_type == "urls" {
            encode_config(id, URL_SAFE_NO_PAD)
        } else {
            id.to_string()
        };

        let vturl = format!(
            "https://www.virustotal.com/api/v3/{}/{}?relationships={}",
            item_type, id_encoded, relations
        );

        let result: Value = self.query_handler(&vturl).await?;

        self.extract_data(result, id).await
    }

    async fn query_handler(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        let response = self.client.get(url).headers(self.headers.clone()).query(&self.params).send().await?;
        let result = response.json::<Value>().await?;

        if result.get("error").is_some() {
            return Err(format!(
                "Error: {}",
                result.get("error").and_then(|v| v.get("message")).unwrap_or(&Value::String("Unknown error".to_string()))
            ).into());
        }

        Ok(result)
    }

    async fn extract_data(&self, object: Value, id: &str) -> Result<JsonInput, Box<dyn Error>> {
        let data = object.get("data").ok_or("No data field in response")?;

        let threat = Threat {
            query: id.to_string(),
            last_seen: data.get("attributes")
                .and_then(|attrs| attrs.get("last_submission_date"))
                .map(|v| Utc.timestamp_opt(v.as_i64().unwrap_or(0), 0))
                .and_then(|local_result| match local_result {
                    chrono::LocalResult::None => None,
                    chrono::LocalResult::Single(datetime) => Some(datetime.to_rfc3339()),
                    chrono::LocalResult::Ambiguous(_, _) => None,}), // Handle ambiguous cases if needed
            label: data.get("attributes").and_then(|attrs| attrs.get("popular_threat_classification")).and_then(|ptc| ptc.get("suggested_threat_label")).map(|v| v.as_str().unwrap_or("").to_string()),
            family: data.get("attributes").and_then(|attrs| attrs.get("malware_config")).and_then(|mc| mc.get("family")).cloned(),
            judgment: Some(Value::Array(data.get("attributes").and_then(|attrs| attrs.get("last_analysis_results")).unwrap_or(&Value::Null).as_object().unwrap_or(&serde_json::Map::new()).iter().filter(|(_, v)| v.get("category").map(|cat| cat == "malicious").unwrap_or(false)).map(|(k, _)| Value::String(k.clone())).collect())),
            reputation: data.get("attributes").and_then(|attrs| attrs.get("reputation")).map(|v| v.as_i64().unwrap_or(0)),
            verdicts: data.get("attributes").and_then(|attrs| attrs.get("sandbox_verdict")).cloned(),
            jarm: data.get("attributes").and_then(|attrs| attrs.get("jarn")).map(|v| v.as_str().unwrap_or("").to_string()),
            tags: data.get("attributes").and_then(|attrs| attrs.get("tags")).map(|v| v.as_array().unwrap().iter().map(|tag| tag.as_str().unwrap().to_string()).collect()),
        };

        let raw_whois = data.get("attributes").and_then(|attrs| attrs.get("whois")).map(|v| v.as_str().unwrap_or("").to_string());
        let whois_details = if let Some(raw_whois) = raw_whois {
            let mut details = HashMap::new();
            for line in raw_whois.lines() {
                if let Some((key, value)) = line.split_once(": ") {
                    details.insert(key.to_string(), value.to_string());
                }
            }
            Some(details)
        } else {
            None
        };

        let relationships = self.extract_relationships(&object).await?;
        let related_items = RelatedItems {
            communicating_files: relationships.get("communicating_files").cloned(),
            contacted_ips: relationships.get("contacted_ips").cloned(),
            contacted_domains: relationships.get("contacted_domains").cloned(),
            resolves_to: relationships.get("resolutions").map(|res| res.iter().map(|r| ResolveRelationship {
                domain: r.clone(),
                ip: id.to_string(),  // Assuming 'id' is the IP here. Adjust as needed.
            }).collect()),
        };

        Ok(JsonInput {
            identity_and_verdict: IdentityAndVerdict {
                threat: threat,
                whois: whois_details,
            },
            activity_and_relationships: Some(ActivityAndRelationships {
                related_items: related_items,
                dns: data.get("attributes").and_then(|attrs| attrs.get("last_dns_records")).map(|dns| dns.as_array().unwrap().iter().map(|record| DnsRecord {
                    record_type: record.get("type").unwrap().as_str().unwrap().to_string(),
                    value: record.get("value").unwrap().as_str().unwrap().to_string(),
                }).collect()),
            }),
        })
    }

    async fn extract_relationships(&self, result: &Value) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
        let mut relationships = HashMap::new();
        let data = result.get("data").ok_or("No data field in response")?.get("relationships").ok_or("No relationships field in data")?;

        for (key, value) in data.as_object().unwrap() {
            let ids: Vec<String> = value.get("data").unwrap().as_array().unwrap().iter().map(|entry| entry.get("id").unwrap().as_str().unwrap().to_string()).collect();
            relationships.insert(key.clone(), ids);
        }

        Ok(relationships)
    }

    fn define_relationships(&self, item_type: &str) -> String {
        match item_type {
            "files" => "collections,communicating_files,contacted_ips,contacted_urls,contacted_domains,itw_domains,itw_urls,itw_ips".to_string(),
            "ip_addresses" | "domains" => "collections,communicating_files,resolutions,referrer_files,resolutions,downloaded_files".to_string(),
            "urls" => "collections,communicating_files,referrer_files,downloaded_files".to_string(),
            _ => {
                println!("Warning: no relations defined for file type");
                "".to_string()
            },
        }
    }

    fn check_string(&self, input: &str) -> String {
        let hash_regex = Regex::new(r"^[a-fA-F0-9]{32,64}$").unwrap();
        let ip_regex = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
        let domain_regex = Regex::new(r"^[a-zA-Z0-9-]+(\.[a-zA-Z0-9-]+)*(\.[a-zA-Z]{2,})$").unwrap();
        let url_regex = Regex::new(r"^(http|https)://[a-zA-Z0-9-]+(\.[a-zA-Z0-9-]+)*(\.[a-zA-Z]{2,})(/.*)*$").unwrap();

        if hash_regex.is_match(input) {
            "files".to_string()
        } else if ip_regex.is_match(input) {
            "ip_addresses".to_string()
        } else if domain_regex.is_match(input) {
            "domains".to_string()
        } else if url_regex.is_match(input) {
            "urls".to_string()
        } else {
            "unknown".to_string()
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonInput {
    pub identity_and_verdict: IdentityAndVerdict,
    pub activity_and_relationships: Option<ActivityAndRelationships>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IdentityAndVerdict {
    pub threat: Threat,
    pub whois: Option<HashMap<String, String>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Threat {
    pub query: String,
    pub last_seen: Option<String>,
    pub label: Option<String>,
    pub family: Option<Value>,
    pub judgment: Option<Value>,
    pub reputation: Option<i64>,
    pub verdicts: Option<Value>,
    pub jarm: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityAndRelationships {
    pub related_items: RelatedItems,
    pub dns: Option<Vec<DnsRecord>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RelatedItems {
    pub communicating_files: Option<Vec<String>>,
    pub contacted_ips: Option<Vec<String>>,
    pub contacted_domains: Option<Vec<String>>,
    pub resolves_to: Option<Vec<ResolveRelationship>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResolveRelationship {
    pub domain: String,
    pub ip: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DnsRecord {
    #[serde(rename = "type")]
    pub record_type: String,
    pub value: String,
}