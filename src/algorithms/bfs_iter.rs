use std::{collections::VecDeque, fmt::Debug, hash::Hash};

use rustc_hash::FxHashSet;

use crate::{graph::WithID, Graph, GraphError};

pub struct BfsIter<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID<VId>,
    Edge:,
{
    graph: &'a Graph<VId, Vertex, Edge>,
    queue: VecDeque<VId>,
    visited: FxHashSet<VId>,
}

impl<'a, VId, Vertex, Edge> BfsIter<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    fn new(
        graph: &'a Graph<VId, Vertex, Edge>,
        start_vertex: VId,
    ) -> Result<Self, GraphError<VId>> {
        graph
            .get_vertex_by_id(&start_vertex)
            .ok_or_else(|| GraphError::VertexNotFound(start_vertex))?; // Check if it exists

        let queue = VecDeque::from([start_vertex]);

        let mut visited = FxHashSet::default();
        visited.insert(start_vertex);

        Ok(BfsIter {
            graph,
            queue,
            visited,
        })
    }
}

impl<'a, VId, Vertex, Edge> Iterator for BfsIter<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_id) = self.queue.pop_front() {
            // Add unvisited neighbors to queue
            let neighbors = self.graph.get_adjacent_vertices(&next_id);

            for v in neighbors {
                let vid = v.get_id();
                if !self.visited.contains(&vid) {
                    self.visited.insert(vid);
                    self.queue.push_back(vid);
                }
            }

            // Return the current vertex
            Some(self.graph.get_vertex_by_id(&next_id).expect(
                "get_vertex_by_id should not error as the vertices in the queue must exist",
            ))
        } else {
            None
        }
    }
}

pub struct BfsIterMut<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID<VId>,
    Edge:,
{
    graph: &'a mut Graph<VId, Vertex, Edge>,
    queue: VecDeque<VId>,
    visited: FxHashSet<VId>,
}

impl<'a, VId, Vertex, Edge> BfsIterMut<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    fn new(
        graph: &'a mut Graph<VId, Vertex, Edge>,
        start_vertex: VId,
    ) -> Result<Self, GraphError<VId>> {
        graph
            .get_vertex_by_id(&start_vertex)
            .ok_or_else(|| GraphError::VertexNotFound(start_vertex))?; // Check if it exists

        let queue = VecDeque::from([start_vertex]);

        let mut visited = FxHashSet::default();
        visited.insert(start_vertex);

        Ok(BfsIterMut {
            graph,
            queue,
            visited,
        })
    }
}

impl<'a, VId, Vertex, Edge> Iterator for BfsIterMut<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    type Item = &'a mut Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_id) = self.queue.pop_front() {
            // Add unvisited neighbors to queue
            let neighbors = self.graph.get_adjacent_vertices(&next_id);

            for v in neighbors {
                let vid = v.get_id();
                if !self.visited.contains(&vid) {
                    self.visited.insert(vid);
                    self.queue.push_back(vid);
                }
            }

            // Return mutable reference to the current vertex
            // This needs to use a method that returns &mut Vertex
            // SAFETY: This is safe because:
            // 1. We only return one mutable reference at a time
            // 2. Each vertex is visited exactly once (tracked by the visited set)
            // 3. The reference doesn't outlive the graph (tied to lifetime 'a)
            unsafe {
                let vertex_ptr = self.graph.get_vertex_by_id_mut(&next_id).expect(
                    "get_vertex_by_id_mut should not error as the vertices in the queue must exist",
                ) as *mut Vertex;

                Some(&mut *vertex_ptr)
            }
        } else {
            None
        }
    }
}

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    pub fn bfs_iter(
        &self,
        start_vertex: VId,
    ) -> Result<BfsIter<VId, Vertex, Edge>, GraphError<VId>> {
        BfsIter::new(self, start_vertex)
    }

    pub fn bfs_iter_mut(
        &mut self,
        start_vertex: VId,
    ) -> Result<BfsIterMut<VId, Vertex, Edge>, GraphError<VId>> {
        BfsIterMut::new(self, start_vertex)
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::{graph::WithID, Graph, GraphError};
    use std::collections::HashSet;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestVertex {
        id: usize,
        value: String,
    }

    impl WithID<usize> for TestVertex {
        fn get_id(&self) -> usize {
            self.id
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestEdge {
        weight: usize,
    }

    #[fixture]
    fn create_test_graph() -> Graph<usize, TestVertex, TestEdge> {
        // Create a graph with the following structure:
        //    0
        //   / \
        //  1   2
        //     / \
        //    3   4
        //   /
        //  5
        let mut graph = Graph::new(true);

        // Add vertices
        graph
            .push_vertex(TestVertex {
                id: 0,
                value: "A".to_string(),
            })
            .unwrap();
        graph
            .push_vertex(TestVertex {
                id: 1,
                value: "B".to_string(),
            })
            .unwrap();
        graph
            .push_vertex(TestVertex {
                id: 2,
                value: "C".to_string(),
            })
            .unwrap();
        graph
            .push_vertex(TestVertex {
                id: 3,
                value: "D".to_string(),
            })
            .unwrap();
        graph
            .push_vertex(TestVertex {
                id: 4,
                value: "E".to_string(),
            })
            .unwrap();
        graph
            .push_vertex(TestVertex {
                id: 5,
                value: "F".to_string(),
            })
            .unwrap();

        // Add edges
        graph.push_edge(0, 1, TestEdge { weight: 1 }).unwrap();
        graph.push_edge(0, 2, TestEdge { weight: 2 }).unwrap();
        graph.push_edge(2, 3, TestEdge { weight: 3 }).unwrap();
        graph.push_edge(2, 4, TestEdge { weight: 4 }).unwrap();
        graph.push_edge(3, 5, TestEdge { weight: 5 }).unwrap();

        graph
    }

    #[rstest]
    fn test_bfs_traversal_order(create_test_graph: Graph<usize, TestVertex, TestEdge>) {
        let graph = create_test_graph;

        // Start BFS from vertex 0
        let bfs = graph.bfs_iter(0).unwrap();
        let visited_ids: Vec<usize> = bfs.map(|v| v.get_id()).collect();

        // Expected BFS order: 0, 1, 2, 3, 4, 5
        assert_eq!(visited_ids, vec![0, 1, 2, 3, 4, 5]);

        // Start BFS from vertex 2
        let bfs = graph.bfs_iter(2).unwrap();
        let visited_ids: Vec<usize> = bfs.map(|v| v.get_id()).collect();

        // Expected BFS order: 2, 3, 4, 5
        assert_eq!(visited_ids, vec![2, 3, 4, 5]);
    }

    #[rstest]
    fn test_bfs_traversal_subset(create_test_graph: Graph<usize, TestVertex, TestEdge>) {
        let graph = create_test_graph;

        // Start from vertex 3, should only visit 3 and 5
        let bfs = graph.bfs_iter(3).unwrap();
        let visited_ids: HashSet<usize> = bfs.map(|v| v.get_id()).collect();

        assert_eq!(visited_ids, HashSet::from([3, 5]));
    }

    #[rstest]
    fn test_bfs_mut_traversal(create_test_graph: Graph<usize, TestVertex, TestEdge>) {
        let mut graph = create_test_graph;

        // Use mutable BFS to modify vertex values
        {
            let bfs_mut = graph.bfs_iter_mut(0).unwrap();
            for vertex in bfs_mut {
                vertex.value = format!("Modified_{}", vertex.value);
            }
        }

        // Verify all vertices were modified
        assert_eq!(graph.get_vertex_by_id(&0).unwrap().value, "Modified_A");
        assert_eq!(graph.get_vertex_by_id(&1).unwrap().value, "Modified_B");
        assert_eq!(graph.get_vertex_by_id(&2).unwrap().value, "Modified_C");
        assert_eq!(graph.get_vertex_by_id(&3).unwrap().value, "Modified_D");
        assert_eq!(graph.get_vertex_by_id(&4).unwrap().value, "Modified_E");
        assert_eq!(graph.get_vertex_by_id(&5).unwrap().value, "Modified_F");
    }

    #[rstest]
    fn test_bfs_invalid_start() {
        let graph: Graph<usize, TestVertex> = Graph::new(false);

        // Try to start BFS from non-existent vertex
        let result = graph.bfs_iter(999);
        assert!(result.is_err());

        if let Err(GraphError::VertexNotFound(id)) = result {
            assert_eq!(id, 999);
        } else {
            panic!("Expected VertexNotFound error");
        }
    }

    #[rstest]
    fn test_bfs_isolated_vertex() {
        let mut graph: Graph<usize, TestVertex> = Graph::new(false);

        // Add a single isolated vertex
        graph
            .push_vertex(TestVertex {
                id: 42,
                value: "Isolated".to_string(),
            })
            .unwrap();

        // BFS should visit only this vertex
        let bfs = graph.bfs_iter(42).unwrap();
        let visited: Vec<usize> = bfs.map(|v| v.get_id()).collect();

        assert_eq!(visited, vec![42]);
    }

    #[rstest]
    fn test_bfs_cycle() {
        let mut graph = Graph::new(true);

        // Create a cycle: 0 -> 1 -> 2 -> 0
        graph
            .push_vertex(TestVertex {
                id: 0,
                value: "A".to_string(),
            })
            .unwrap();
        graph
            .push_vertex(TestVertex {
                id: 1,
                value: "B".to_string(),
            })
            .unwrap();
        graph
            .push_vertex(TestVertex {
                id: 2,
                value: "C".to_string(),
            })
            .unwrap();

        graph.push_edge(0, 1, TestEdge { weight: 1 }).unwrap();
        graph.push_edge(1, 2, TestEdge { weight: 1 }).unwrap();
        graph.push_edge(2, 0, TestEdge { weight: 1 }).unwrap();

        // BFS should visit all vertices exactly once
        let bfs = graph.bfs_iter(0).unwrap();
        let visited: Vec<usize> = bfs.map(|v| v.get_id()).collect();

        assert_eq!(visited.len(), 3);
        assert_eq!(visited[0], 0);
        // 1 and 2 might be visited in any order depending on graph implementation
        assert!(visited.contains(&1));
        assert!(visited.contains(&2));
    }
}
