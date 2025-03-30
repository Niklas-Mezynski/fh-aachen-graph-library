use std::{collections::VecDeque, fmt::Debug, hash::Hash};

use rustc_hash::FxHashSet;

use crate::{graph::WithID, Graph, GraphError, GraphInterface};

impl<VId, Vertex: WithID<Vertex, VId>, Edge> Graph<VId, Vertex, Edge>
where
    VId: Debug + Eq + Hash + Copy,
    Vertex: Debug,
    Edge: Debug + Clone,
{
    pub fn count_connected_subgraphs(&self) -> Result<u32, GraphError<VId>> {
        let mut vertices = VecDeque::from(self.get_all_vertices());
        let mut visited: FxHashSet<VId> = FxHashSet::default();

        let mut count: u32 = 0;

        while let Some(current_root) = vertices.pop_front() {
            let current_root_vid = current_root.get_id();
            if visited.contains(&current_root_vid) {
                continue;
            }
            for vertex in self.bfs_iter(current_root_vid)? {
                // Remember that this vertex was already visited
                visited.insert(vertex.get_id());
            }

            // We traversed one whole graph, add one to the final count
            count += 1;
        }

        dbg!("Algorithm end!");

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_connected_subgraphs() {
        // Define test cases: (file_path, expected_count)
        let test_cases = vec![
            ("resources/test_graphs/undirected/Graph1.txt", 2),
            ("resources/test_graphs/undirected/Graph2.txt", 4),
            ("resources/test_graphs/undirected/Graph3.txt", 4),
            ("resources/test_graphs/undirected/Graph_gross.txt", 222),
            ("resources/test_graphs/undirected/Graph_ganzgross.txt", 9560),
            (
                "resources/test_graphs/undirected/Graph_ganzganzgross.txt",
                306,
            ),
            // Add more test cases as needed
        ];

        for (file_path, expected_count) in test_cases {
            // Read the graph from file
            let graph = Graph::from_hoever_file(file_path, false)
                .unwrap_or_else(|e| panic!("Graph could not be constructed from file: {:?}", e));

            // Count connected subgraphs
            let count = graph
                .count_connected_subgraphs()
                .unwrap_or_else(|e| panic!("Failed to count connected subgraphs: {:?}", e));

            // Verify expected count
            assert_eq!(
                count, expected_count,
                "For graph {}, expected {} connected subgraphs, but got {}",
                file_path, expected_count, count
            );
        }
    }
}
