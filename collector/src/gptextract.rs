use std::error::Error;

use crate::gptcall::call_openai_chat;
use crate::simparse::fetch_and_extract;
use crate::my_secret::OPENAI_KEY;

pub async fn information_extraction(input: &str, entities: Option<&[&str]>, proxy_url: Option<&str>) -> Result<String, Box<dyn Error>> {
    
    // Handle optional entities
    let entities_list = match entities {
        Some(list) => list,
        None => &[],
    };

    let text_blob;
    // Determine if input is a URL or a text blob
    if input.starts_with("http://") || input.starts_with("https://") {
        // Fetch and extract tags from the URL
        let tags = vec!["h1", "h2", "h3", "h4", "p", "article", "td", "ul", "li", "lo"];
        let results = fetch_and_extract(input, tags, proxy_url).await?;
        // Concatenate all tag values into a single text blob
        text_blob = results.iter().map(|result| result.value.clone()).collect::<Vec<String>>().join(" ");
    } else {
        // Use the input directly as the text blob
        text_blob = input.to_string();
    }

    // Create the prompt for GPT
    let entities_str = if entities_list.is_empty() {
        "<no entities provided>".to_string()
    } else {
        entities_list.join(", ")
    };

    let constructed_prompt = format!(
        "ignoring marketing language and focusing on content, extract the following entities from this article and return a JSON with edges and nodes, both lists of objects. Each object in the nodes JSON list can have the keys (value, type) and valid types are {}. The list of edges each have keys (from, to, type) where from and to are node values and type is a verblike word or phrase.\n\n{}",
        entities_str, text_blob
    );

    // Call OpenAI Chat
    let response = call_openai_chat("You are a helpful assistant.", &constructed_prompt, OPENAI_KEY).await?;
    Ok(response)
}

