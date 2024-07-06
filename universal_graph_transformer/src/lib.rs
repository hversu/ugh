extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::process;
use crate::transform::transform_graph;

mod graphml_parser;
mod vt_parser;
mod transform;
mod input_type;
mod types;
mod vt_api;
mod mysecret;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename|json_string> [mode]", args[0]);
        process::exit(1);
    }

    let input = &args[1];
    let mode = if args.len() == 3 {
        &args[2]
    } else {
        "auto"
    };

    match transform_graph(input, mode).await {
        Ok(_) => println!("Transformation successful!"),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}


pub async fn graph_transformer(input: &str, mode: &str) -> Result<(), String> {
    match transform_graph(input, mode).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    #[test]
    fn test_graph_transformer() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("../example_data/netsupp_2806.graphml")
            .arg("auto")
            .output()
            .expect("Failed to run cargo run");
    }

    #[test]
    fn test_graph_transformer_with_json() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("{\"nodes\": [{\"id\": \"1\", \"label\": \"Node 1\"}, {\"id\": \"2\", \"label\": \"Node 2\"}], \"edges\": [{\"source\": \"1\", \"target\": \"2\", \"label\": \"Edge 1\"}]}")
            .arg("auto")
            .output()
            .expect("Failed to run cargo run");
    }

}