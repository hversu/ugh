use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::error::Error;
use crate::graphml_parser::{Graph, parse_graphml};

pub fn transform_graph(filename: &str, mode: &str) -> Result<(), Box<dyn Error>> {
    let (nodes, edges) = if mode == "auto" || mode == "graphml" || mode == "maltego" {
        parse_graphml(filename)?
    } else {
        return Err("Unsupported mode".into());
    };

    let rich_graph = Graph { nodes, edges };
    save_json(&rich_graph, "rich.json")?;

    Ok(())
}

fn save_json<T: Serialize>(data: &T, filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data)?;
    Ok(())
}