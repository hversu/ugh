# ugh... (universal graph harness)!

a set of tools for casting graphs of any form to efficient universal structures

- **univeresal graph transformer** to tranform multiple data sources to the universal data format
- **add hyperedges** to define, find and add hyperedges

## universal_graph_transformer

takes .graphm file, return lean.json and rich.json

`cargo run maltego.graphml`

if you add a mysecret.rs with a Virus Total API Key, you can do:

`cargo run <indicator> vtapi`

accepted indicators are domains, ips, and hashes.

supports:
- Maltego
- VirusTotal Graph

hoping to support:
- Vertex Synapse
- Relational Data
- Unstructured Data

## add_hyperedges
find and add hyperedges converted graph (e.g. `rich.json`)

find all hyperedges (noisy, but can be used for graph analysis):

`cargo run rich.json`

find hyperedges on specific property value (supports * wildcard):

`cargo run rich.json <value>`
