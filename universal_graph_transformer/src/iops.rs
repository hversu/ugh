use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn read_file_to_string(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_to_string() {
        let result = read_file_to_string("Cargo.toml");
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_read_file_to_string_fail() {
        let result = read_file_to_string("Cargo.toml2");
        assert_eq!(result.is_err(), true);
    }
}