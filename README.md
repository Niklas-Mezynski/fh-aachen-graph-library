# Rust based Graph library

This project is part of the Praktikum in _Mathematische Algorithmen und Programmierung_

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
