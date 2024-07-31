# Running the web server

1. clone repo on Linux web server
2. if serving locally, change `0.0.0.0:3000` to `127.0.0.1:3000` in `~/ugh/web_server/src/main.rs`
3. in `~/ugh/web_server`, enter `cargo run` - the web server should enter a LISTENING state
4. you should be able to hit the web server on port 3000 for the graph interface
5. Can use example data from `~/ugh/example_data`

# ugh... (universal graph harness)!

a set of tools for casting "any" data to efficient universal, graph structures

- **univeresal graph transformer** to tranform multiple data sources to the universal data format
- **add hyperedges** to define, find and add hyperedges

supports:
- Maltego
- VirusTotal Graph

in progress:
- Unstructured Data (https://github.com/hversu/collector)

to support:
- Vertex Synapse
- Relational Data

## universal_graph_transformer

takes .graphm file, return lean.json and rich.json

`cargo run ..\example_data\netsupp_2806.graphml`

if you add a mysecret.rs with a Virus Total API Key, you can do:

`cargo run <indicator> vtapi`

verified indicators are domains, ips, and hashes.

or you can use the example jsons that use an intermediate data deifnition for VT data for testing if you don't have a key:

` cargo run ..\example_data\vt_domain_example.json`

## add_hyperedges
find and add hyperedges converted graph (e.g. `rich.json`)

find all hyperedges (noisy, but can be used for graph analysis):

`cargo run rich.json`

find hyperedges on specific property value (supports * wildcard):

`cargo run rich.json <value>`
