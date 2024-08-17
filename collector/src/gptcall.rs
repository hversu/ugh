use reqwest::Client;
use std::error::Error;
use serde_json::json;
use crate::types::ChatRequest;
use crate::types::Message;
use crate::types::ChatResponse;

pub async fn call_openai_chat(
    system_prompt: &str,
    prompt: &str,
    api_key: &str,
) -> Result<String, Box<dyn Error>> {
    // Create the request body
    let request_body = ChatRequest {
        model: "gpt-4-turbo".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
        response_format: json!({ "type": "json_object" }),  // Include the response format
    };

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let chat_response: ChatResponse = response.json().await?;
        if let Some(choice) = chat_response.choices.get(0) {
            Ok(choice.message.content.clone())
        } else {
            Err("No choices in response".into())
        }
    } else {
        let error_message = response.text().await?;
        Err(error_message.into())
    }
}

