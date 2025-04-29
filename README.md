# Rust based Graph library

This project is part of the Praktikum in _Mathematische Algorithmen und Programmierung_

## Benchmarks

All benchmarks have been conducted on an Apple MacBook Pro 2023 with M2 Pro Chip and 16 GB memory.

### Graph Creation

| Input File              | Time      |
| ----------------------- | --------- |
| Graph1.txt              | 7.9301 µs |
| Graph2.txt              | 248.81 µs |
| Graph3.txt              | 169.43 µs |
| Graph_gross.txt         | 31.893 ms |
| Graph_ganzgross.txt     | 162.54 ms |
| Graph_ganzganzgross.txt | 898.13 ms |

### Connected Subgraphs

| Input File              | Time BFS  | Time DFS  |
| ----------------------- | --------- | --------- |
| Graph1.txt              | 1.2248 µs | 1.2211 µs |
| Graph2.txt              | 85.558 µs | 83.231 µs |
| Graph3.txt              | 82.195 µs | 81.051 µs |
| Graph_gross.txt         | 11.234 ms | 11.023 ms |
| Graph_ganzgross.txt     | 136.72 ms | 138.05 ms |
| Graph_ganzganzgross.txt | 473.59 ms | 468.20 ms |

### Minimum Spanning Tree

| Input File    | Time Prim | Time Kruskal |
| ------------- | --------- | ------------ |
| G_1_2.txt     | 265.69 µs | 264.20 µs    |
| G_1_20.txt    | 1.037 ms  | 900.37 µs    |
| G_1_200.txt   | 6.425 ms  | 6.1875 ms    |
| G_10_20.txt   | 3.495 ms  | 3.1350 ms    |
| G_10_200.txt  | 13.373 ms | 10.309 ms    |
| G_100_200.txt | 49.644 ms | 40.924 ms    |
