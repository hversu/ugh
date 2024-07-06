use std::fs;

pub fn is_file(filename: &str) -> bool {
    fs::metadata(filename).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_file() {
        assert_eq!(is_file("Cargo.toml"), true);
        assert_eq!(is_file("Cargo.toml2"), false);
    }
}