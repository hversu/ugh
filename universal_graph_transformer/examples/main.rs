extern crate dotenv;

use dotenv::dotenv;
use universal_graph_transformer::graph_transformer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    match graph_transformer("../example_data/netsupp_2806.graphml", "auto").await {
        Ok(_) => println!("Transformation successful!"),
        Err(e) => eprintln!("An error occurred: {}", e),
    };
}