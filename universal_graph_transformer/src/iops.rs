use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn read_file_to_string(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}