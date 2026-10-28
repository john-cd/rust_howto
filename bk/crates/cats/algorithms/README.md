<!-- TODO Reviewed ??-??-2026 -->

# Algorithms

This crate contains code examples for the **Algorithms** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `base16ct`, `base64ct`, `blake3`, `clap`, `data-encoding`.

## Examples Covered

- **Graph** (`examples/graph/`): Demonstrates concepts like `pathfinding_astar`, `petgraph_astar`, `petgraph_bfs`, `petgraph_dfs`, `petgraph_dijkstra`, `petgraph_mst`, `petgraph_toposort`.
- **Hashing** (`examples/hashing/`): Demonstrates concepts like `blake3`, `foldhash`, `md5`, `sha1`, `sha2`, `sha_digest`, `xxh3_checksum`.
- **Rand** (`examples/rand/`): Demonstrates concepts like `fastrand`, `rand1`, `rand_alphanumeric`, `rand_custom`, `rand_distr`, `rand_password`, `rand_range`, `rand_range1`.
- **Sort** (`examples/sort/`): Demonstrates concepts like `sort_basic`, `sort_float`, `sort_struct`.

## Usage

To check or run examples in this crate:

```console
cargo check --package algorithms --examples
cargo test --package algorithms
```

Code examples correspond to chapters in `bk/src/categories/algorithms` or related book sections.
