use serpapi_search_rust::serp_api_search::SerpApiSearch;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

use crate::my_secret;
use crate::types::OrganicResult;
use crate::types::ParsedResult;
use crate::types::RelatedQuestion;
use crate::types::ParseOutput;


pub async fn search_query(query: &str) -> Result<Value, Box<dyn Error>> {
    let mut params = HashMap::<String, String>::new();
    params.insert("engine".to_string(), "google".to_string());
    params.insert("q".to_string(), query.to_string());
    params.insert("google_domain".to_string(), "google.com".to_string());
    params.insert("gl".to_string(), "us".to_string());
    params.insert("hl".to_string(), "en".to_string());

    let search = SerpApiSearch::google(params, my_secret::SERP_API_KEY.to_string());
    
    let results = search.json().await?;

    Ok(results)
    }

pub fn parse_google_results(results: &Value) -> ParseOutput {
    let mut content = Vec::new();
    let mut links = Vec::new();

    if let Some(organic_results) = results.get("organic_results") {
        if let Some(organic_results_array) = organic_results.as_array() {
            for oresult in organic_results_array {
                if let Ok(oresult) = serde_json::from_value::<OrganicResult>(oresult.clone()) {
                    let temp = ParsedResult {
                        title: oresult.title,
                        date: oresult.date,
                        source: oresult.source,
                        content: oresult.snippet,
                        r#type: "search result snip".to_string(),
                    };
                    content.push(temp);
                    links.push(oresult.link);
                }
            }
        }
    }

    if let Some(related_questions) = results.get("related_questions") {
        if let Some(related_questions_array) = related_questions.as_array() {
            for oresult in related_questions_array {
                if let Ok(oresult) = serde_json::from_value::<RelatedQuestion>(oresult.clone()) {
                    let temp = ParsedResult {
                        title: oresult.title,
                        date: oresult.date,
                        source: None,
                        content: oresult.question,
                        r#type: "user comment".to_string(),
                    };
                    content.push(temp);
                }
            }
        }
    }

    ParseOutput { content, links }
}

