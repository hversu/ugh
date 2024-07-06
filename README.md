# ugh... (universal graph harness)!

a set of tools for casting "any" data to efficient universal, graph structures

- **univeresal graph transformer** to tranform multiple data sources to the universal data format
- **add hyperedges** to define, find and add hyperedges

supports:
- Maltego
- VirusTotal Graph

in progress:
- Unstructured Data

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
