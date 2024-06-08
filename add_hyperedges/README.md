add hyperedges to normalized feature rich graph (`rich.json`) and returns `graph_with_hyperedges.json`

two types of hyperedges are currently defined (`hypertype`):
- noun_property - a list of nodes that share a noun property
- superedges - a list of edges that share an edge property

you can find and add all hyperedges:
`cargo run rich.json`

or you can just return properties matching a particular value (or value matching a wildcarded substring):
`cargo run rich.json "To Service*"`
