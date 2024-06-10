use std::env;
use std::process;
use crate::transform::transform_graph;

mod graphml_parser;
mod transform;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [mode]", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let mode = if args.len() == 3 {
        &args[2]
    } else {
        "auto"
    };

    match transform_graph(filename, mode) {
        Ok(_) => println!("Transformation successful!"),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}