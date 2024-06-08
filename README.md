# ugh... (universal graph harness)!

a set of tools for casting graphs of any form to efficient universal structures

- **univeresal graph transformer** to tranform multiple data sources to the universal data format
- **add hyperedges** to define, find and add hyperedges

## universal_graph_transformer

takes .graphm file, return lean.json and rich.json

`cargo run maltego.graphml`

supports:
- Maltego

hoping to support:
- Vertex Synapse
- VirusTotal Graph
- Unstructured Data

## add_hyperedges
find and add hyperedges converted graph (e.g. `rich.json`)

find all hyperedges (noisy, but can be used for graph analysis):

`cargo run rich.json`

find hyperedges on specific property value (supports * wildcard):

`cargo run rich.json <value>`
