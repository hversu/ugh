use serde::Deserialize;

#[derive(Deserialize)]
pub struct TextSubmitInput {
    pub data: String,
    pub entities: Option<String>,
    pub isUrl: bool,
}