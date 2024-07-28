extern crate dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref VTAPI: String =
        env::var("VTAPI").unwrap_or_else(|_| "insert_api_here".to_string());
    pub static ref API_KEY_VAR: String =
        env::var("API_KEY_VAR").unwrap_or_else(|_| "insert_openai_api_here".to_string());
}

// Function to get the VTAPI as &str
pub fn get_vtapi() -> &'static str {
    &VTAPI
}

// Function to get the API_KEY_VAR as &str
pub fn get_api_key_var() -> &'static str {
    &API_KEY_VAR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_vtapi() {
        assert_eq!(get_vtapi(), "insert_api_here");
    }

    #[test]
    fn test_get_api_key_var() {
        assert_eq!(get_api_key_var(), "insert_openai_api_here");
    }
}
