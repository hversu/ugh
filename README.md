# ugh... (universal graph harness)!

a set of tools for casting "any" data to an efficient universal, graph structures

- **univeresal graph transformer** to tranform multiple data sources to the universal data format
- **add hyperedges** to define, find and add hyperedges

supports:
- Maltego
- VirusTotal Graph

in progress:
- Relational Data

to support:
- Vertex Synapse
- Unstructured Data

## universal_graph_transformer

takes .graphm file, return lean.json and rich.json

`cargo run maltego.graphml`

if you add a mysecret.rs with a Virus Total API Key, you can do:

`cargo run <indicator> vtapi`

verified indicators are domains, ips, and hashes.


## add_hyperedges
find and add hyperedges converted graph (e.g. `rich.json`)

find all hyperedges (noisy, but can be used for graph analysis):

`cargo run rich.json`

find hyperedges on specific property value (supports * wildcard):

`cargo run rich.json <value>`
