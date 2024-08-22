pub mod text_submit;

use axum::response::IntoResponse;
use crate::template::{HtmlTemplate, IndexTemplate};

pub async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}