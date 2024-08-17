use serde_derive::{Serialize, Deserialize};
use serde_json;

// googler
#[derive(Deserialize, Debug)]
pub struct OrganicResult {
    pub title: String,
    pub date: Option<String>,
    pub source: Option<String>,
    pub snippet: String,
    pub link: String,
}

#[derive(Deserialize, Debug)]
pub struct RelatedQuestion {
    pub title: String,
    pub date: Option<String>,
    pub question: String,
}

#[derive(Serialize, Debug)]
pub struct ParsedResult {
    pub title: String,
    pub date: Option<String>,
    pub source: Option<String>,
    pub content: String,
    pub r#type: String,
}

#[derive(Serialize, Debug)]
pub struct ParseOutput {
    pub content: Vec<ParsedResult>,
    pub links: Vec<String>,
}

// gptcall
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub response_format: serde_json::Value,  // Add this field
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

// simparse
#[derive(Serialize, Deserialize)]
pub struct TagValuePair {
    pub tag: String,
    pub value: String,
}
