use reqwest::{Client};
use select::document::Document;
use select::predicate::Any;
use std::error::Error;

use crate::types::TagValuePair;

pub async fn fetch_and_extract(url: &str, tags: Vec<&str>, proxy_url: Option<&str>) -> Result<Vec<TagValuePair>, Box<dyn Error>> {
    let client = if let Some(proxy_url) = proxy_url {
        let proxy = reqwest::Proxy::all(proxy_url)?;
        Client::builder().proxy(proxy).build()?
    } else {
        Client::new()
    };
    println!("{:?}", client);
    // Fetch the content from the URL
    let res = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    // Parse the HTML document
    let document = Document::from(res.as_str());

    // Extract the contents of the specified tags or paths
    let mut results = Vec::new();
    for node in document.find(Any) {
        if let Some(tag) = node.name() {
            for &tag_spec in &tags {
                if tag_spec.contains('.') {
                    let parts: Vec<&str> = tag_spec.split('.').collect();
                    if parts.len() == 2 && tag == parts[0] {
                        if let Some(attr_value) = node.attr(parts[1]) {
                            results.push(TagValuePair {
                                tag: tag_spec.to_string(),
                                value: attr_value.to_string(),
                            });
                        }
                    }
                } else if tag == tag_spec {
                    results.push(TagValuePair {
                        tag: tag.to_string(),
                        value: node.text(),
                    });
                }
            }
        }
    }

    Ok(results)
}
