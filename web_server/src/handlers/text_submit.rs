use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use collector::{gptextract};
use universal_graph_transformer::transform::save_json;
use crate::models::text_submit::TextSubmitInput;
use crate::OUTPUT_DIRECTORY;

pub async fn handle_text_submit(payload: Json<TextSubmitInput>) -> Result<String, (StatusCode, String)> {
    println!("The text submitted is a URL: {}", payload.data);
    let input = payload.data.as_str();
    let tags = vec!["h1", "h2", "h3", "h4", "p", "article", "td", "ul", "li", "lo"];
    let proxy_url: Option<&str> = None;
    let entities: Vec<&str>;

    if let Some(entities_str) = &payload.entities {
        entities = entities_str.split(',').collect();
    } else {
        entities = vec![];
    }

    let data = match gptextract::information_extraction(input, Some(&entities), proxy_url).await {
        Ok(extraction_response) => {
            println!("Raw extraction_response: {}", extraction_response);
            extraction_response
        },
        Err(err) => {
            eprintln!("Extraction error: {}", err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
        }
    };

    let my_uuid = Uuid::new_v4();
    let output_path = format!("{}/{}.json", OUTPUT_DIRECTORY, my_uuid);

    match save_json(&output_path, &data) {
        Ok(_) => println!("Data saved to {}", output_path),
        Err(err) => eprintln!("Error saving data: {}", err)
    };

    return Ok(output_path);
}
