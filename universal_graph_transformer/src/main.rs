use std::env;
use std::process;
use crate::transform::transform_graph;

mod graphml_parser;
mod vt_parser;
mod transform;
mod input_type;
mod types;

fn main() {
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

    match transform_graph(input, mode) {
        Ok(_) => println!("Transformation successful!"),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}