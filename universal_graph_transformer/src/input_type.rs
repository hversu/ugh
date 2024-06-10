use std::fs;

pub fn is_file(filename: &str) -> bool {
    fs::metadata(filename).is_ok()
}