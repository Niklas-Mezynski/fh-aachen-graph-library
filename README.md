# Rust based Graph library

This project is part of the Praktikum in _Mathematische Algorithmen und Programmierung_.

It implements a graph library in Rust, which includes various algorithms for graph processing. While it is not planned to be a full-featured library, it tries to be as generic as possible and can possibly be used in other projects. The library supports:

- Different graph backends
  - An adjacency list backend, which stores each node's neighbors in a list in a HashMap
  - An adjacency matrix backend, which stores the graph in a 2D vector
- Algorithms for finding connected components
- Algorithms for finding the minimum spanning tree
- Algorithms for solving the traveling salesman problem
- Algorithms for finding the shortest path

## Benchmarks

All benchmarks have been conducted on an Apple MacBook Pro 2023 with M2 Pro Chip and 16 GB memory.

### Graph Creation

#### List Graph

| Input File              | List Graph |
| ----------------------- | ---------- |
| Graph1.txt              | 8.27 µs    |
| Graph2.txt              | 259.68 µs  |
| Graph3.txt              | 184.70 µs  |
| Graph_gross.txt         | 34.91 ms   |
| Graph_ganzgross.txt     | 169.27 ms  |
| Graph_ganzganzgross.txt | 920.82 ms  |

#### Matrix Graph

| Input File | Matrix Graph |
| ---------- | ------------ |
| K_30.txt   | 36.15 µs     |
| K_50.txt   | 87.85 µs     |
| K_70.txt   | 179.66 µs    |
| K_100.txt  | 348.22 µs    |

### Connected Subgraphs

| Input File              | Time BFS  | Time DFS  |
| ----------------------- | --------- | --------- |
| Graph1.txt              | 908.62 ns | 909.15 ns |
| Graph2.txt              | 65.654 µs | 62.776 µs |
| Graph3.txt              | 57.869 µs | 55.681 µs |
| Graph_gross.txt         | 9.8099 ms | 9.6489 ms |
| Graph_ganzgross.txt     | 110.67 ms | 114.51 ms |
| Graph_ganzganzgross.txt | 413.46 ms | 412.79 ms |

### Minimum Spanning Tree

| Input File    | Time Prim | Time Kruskal |
| ------------- | --------- | ------------ |
| G_1_2.txt     | 245.84 µs | 234.70 µs    |
| G_1_20.txt    | 1.1128 ms | 869.12 µs    |
| G_1_200.txt   | 7.7507 ms | 5.8399 ms    |
| G_10_20.txt   | 3.2999 ms | 2.9662 ms    |
| G_10_200.txt  | 14.702 ms | 9.1424 ms    |
| G_100_200.txt | 49.202 ms | 35.084 ms    |

### TSP Benchmarks

The brute force and branch and bound algorithms find the exact solution, but are only feasible for small graphs. The nearest neighbor and double tree algorithms provide approximate solutions and are faster, but do not guarantee the optimal solution.

| Input File | Brute Force | Branch and Bound | Nearest Neighbor | Double Tree |
| ---------- | ----------- | ---------------- | ---------------- | ----------- |
| K_10.txt   | 5.01 ms     | 123.88 µs        | 331.05 ns        | 2.26 µs     |
| K_10e.txt  | 5.02 ms     | 48.24 µs         | 342.90 ns        | 2.34 µs     |
| K_12.txt   | 544.99 ms   | 837.38 µs        | 396.67 ns        | 2.80 µs     |
| K_12e.txt  | 545.06 ms   | 315.48 µs        | 378.60 ns        | 2.75 µs     |
| K_15.txt   | -           | 5.87 ms          | 479.86 ns        | 3.70 µs     |
| K_15e.txt  | -           | 1.00 ms          | 464.40 ns        | 3.78 µs     |
| K_20.txt   | -           | 834.84 ms        | 703.75 ns        | 5.39 µs     |
| K_30.txt   | -           | -                | 1.05 µs          | 9.69 µs     |
| K_50.txt   | -           | -                | 2.32 µs          | 20.65 µs    |
| K_70.txt   | -           | -                | 3.99 µs          | 63.52 µs    |
| K_100.txt  | -           | -                | 7.78 µs          | 143.08 µs   |

### Shortest Path Benchmarks

These benchmarks measure the time it takes to find the shortest path from a single source to all other nodes in the graph. The first `WegeX` graphs are directed graphs, designed for this task. The `G_X_Y` graphs are the same test graphs used in the MST benchmarks, but once treated as directed graphs and once as undirected graphs.

| Input File       | Dijkstra | Bellman-Ford | Dijkstra (undirected) | Bellman-Ford (undirected) |
| ---------------- | -------- | ------------ | --------------------- | ------------------------- |
| Wege1.txt        | 373 ns   | 323 ns       | -                     | -                         |
| Wege2.txt        | -        | 528 ns       | -                     | -                         |
| Wege3.txt        | -        | 812 ns       | -                     | -                         |
| **Other graphs** |          |              |                       |                           |
| G_1_2.txt        | 72.6 µs  | 78.4 µs      | 115.9 µs              | 159.6 µs                  |
| G_1_20.txt       | 423.6 µs | 593.3 µs     | 738.9 µs              | 1.07 ms                   |
| G_10_20.txt      | 1.16 ms  | 2.49 ms      | 1.87 ms               | 3.10 ms                   |

## Usage example

```rust
use graph_library::{
    graph::{GraphBase, ListGraphBackend, WeightedEdge, WithID},
    ListGraph, Undirected,
};

// Vertices have to be identifiable through some hashable ID
// (or even indexable when using the Matrix backend)
#[derive(Clone)]
struct Vertex(u32);
impl WithID for Vertex {
    type IDType = u32;

    fn get_id(&self) -> Self::IDType {
        self.0
    }
}

// Edge data can be anything
// When using algorithms that require weights, it has to implement the `WeightedEdge` trait
#[derive(Clone)]
struct Edge(f32);
impl WeightedEdge for Edge {
    type WeightType = f32;

    fn get_weight(&self) -> Self::WeightType {
        self.0
    }
}

fn main() {
    let mut graph = ListGraph::<Vertex, Edge, Undirected>::new();

    graph.push_vertex(Vertex(1)).unwrap();
    graph.push_vertex(Vertex(2)).unwrap();
    graph.push_vertex(Vertex(3)).unwrap();
    graph.push_vertex(Vertex(4)).unwrap();

    graph.push_edge(1, 2, Edge(1.0)).unwrap();
    graph.push_edge(1, 3, Edge(4.0)).unwrap();
    graph.push_edge(2, 3, Edge(2.0)).unwrap();
    graph.push_edge(2, 4, Edge(3.0)).unwrap();
    graph.push_edge(3, 4, Edge(1.0)).unwrap();
    // graph:
    //
    //     1
    //    / \
    // 1.0   4.0
    //  /     \
    // 2--2.0--3
    //  \     /
    // 3.0  1.0
    //    \ /
    //     4
    //

    // Construct an mst starting at vertex `1`.
    // The resulting graph should also be a ListGraph
    let mst = graph
        .mst_prim::<ListGraphBackend<_, _, Undirected>>(Some(1))
        .unwrap();

    let dfs_vec = mst
        .dfs_iter(1)
        .unwrap()
        .map(|v| v.get_id())
        .collect::<Vec<_>>();

    assert_eq!(dfs_vec, vec![1, 2, 3, 4]);
}
```
