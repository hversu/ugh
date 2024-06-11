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

// pub const VTAPI: &str = "YOUR_API_KEY_HERE"; // Replace with your VirusTotal API key

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

    pub async fn call_vt_hal(&self, indicator: &str, item_type: Option<&str>) -> Result<VTData, Box<dyn Error>> {
        let item_type = match item_type {
            Some(t) => t.to_string(),
            None => self.check_string(indicator),
        };

        if item_type == "unknown" {
            return Ok(VTData { 
                threat: Some(Threat {
                    query: None,
                    last_seen: None,
                    label: None,
                    family: None,
                    judgment: None,
                    reputation: None,
                    verdicts: None,
                    jarm: None,
                    tags: None,
                }), 
                technical: None, 
                relationships: None, 
                whois: None 
            });
        }

        let vt_data = self.quick_crawl(&item_type, indicator).await?;
        // println!("{:?}", vt_data);
        Ok(vt_data)
    }

    async fn quick_crawl(&self, item_type: &str, id: &str) -> Result<VTData, Box<dyn Error>> {
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

    async fn extract_data(&self, object: Value, id: &str) -> Result<VTData, Box<dyn Error>> {
        let data = object.get("data").ok_or("No data field in response")?;

        let threat = Threat {
            query: Some(id.to_string()),
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

        let technical = Technical {
            exif: data.get("attributes").and_then(|attrs| attrs.get("exiftool")).cloned(),
            names: data.get("attributes").and_then(|attrs| attrs.get("names")).map(|names| names.as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_string()).collect()),
            sigma_analysis: data.get("attributes").and_then(|attrs| attrs.get("sigma_analysis_results")).cloned(),
        };

        let relationships = Relationships {
            c2: data.get("attributes").and_then(|attrs| attrs.get("malware_config")).and_then(|mc| mc.get("c2")).cloned(),
            c2url: data.get("attributes").and_then(|attrs| attrs.get("malware_config")).and_then(|mc| mc.get("c2urls")).map(|urls| urls.as_array().unwrap().iter().map(|url| url.as_str().unwrap_or("").to_string()).collect()),
            registrar: data.get("attributes").and_then(|attrs| attrs.get("registrar")).map(|v| v.as_str().unwrap_or("").to_string()),
            dns: data.get("attributes").and_then(|attrs| attrs.get("last_dns_records")).cloned(),
            https_certificate: Some(serde_json::json!({
                "subject": data.get("attributes").and_then(|attrs| attrs.get("last_https_certificate")).and_then(|cert| cert.get("subject")).map(|v| v.as_str().unwrap_or("").to_string()),
                "issuer": data.get("attributes").and_then(|attrs| attrs.get("last_https_certificate")).and_then(|cert| cert.get("issuer")).map(|v| v.as_str().unwrap_or("").to_string()),
                "expiry": data.get("attributes").and_then(|attrs| attrs.get("last_https_certificate")).and_then(|cert| cert.get("not_after")).map(|v| v.as_str().unwrap_or("").to_string())
            })),
            related_items: Some(self.extract_relationships(&object).await?),
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
        let whois = Whois {
            details: whois_details,
        };

        Ok(VTData {
            threat: Some(threat),
            technical: Some(technical),
            relationships: Some(relationships),
            whois: Some(whois),
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
            "files" => "collections,contacted_ips,contacted_urls,contacted_domains,itw_domains,itw_urls,itw_ips".to_string(),
            "ip_addresses" | "domains" => "collections,communicating_files,referrer_files,resolutions,downloaded_files".to_string(),
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
pub struct VTData {
    pub threat: Option<Threat>,
    pub technical: Option<Technical>,
    pub relationships: Option<Relationships>,
    pub whois: Option<Whois>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Threat {
    pub query: Option<String>,
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
pub struct Technical {
    pub exif: Option<Value>,
    pub names: Option<Vec<String>>,
    pub sigma_analysis: Option<Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Relationships {
    pub c2: Option<Value>,
    pub c2url: Option<Vec<String>>,
    pub registrar: Option<String>,
    pub dns: Option<Value>,
    pub https_certificate: Option<Value>,
    pub related_items: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Whois {
    pub details: Option<HashMap<String, String>>,
}