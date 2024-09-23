use lazy_static::lazy_static;
use std::env;

pub const SERP_API_KEY: &str = "<get a free key at serpapi.com>";
pub const OPENAI_KEY: &str = "<your OpenAI key>";


lazy_static! {
    pub static ref SERP_API_KEY_VAR: String =
        env::var("SERP_API_KEY_VAR").unwrap_or_else(|_| SERP_API_KEY.to_string());
    pub static ref OPENAI_KEY_VAR: String =
        env::var("OPENAI_KEY_VAR").unwrap_or_else(|_| OPENAI_KEY.to_string());
}

// Function to get the SERP_API_KEY_VAR as &str
pub fn get_serp_api_key_var() -> &'static str {
    &SERP_API_KEY_VAR
}

// Function to get the OPENAI_KEY_VAR as &str
pub fn get_openai_key_var() -> &'static str {
    &OPENAI_KEY_VAR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_serp_api_key_var() {
        assert_eq!(get_serp_api_key_var(), SERP_API_KEY);
    }

    #[test]
    fn test_get_openai_key_var() {
        assert_eq!(get_openai_key_var(), OPENAI_KEY);
    }
}