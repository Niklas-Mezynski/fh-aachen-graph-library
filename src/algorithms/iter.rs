use crate::{
    algorithms::{
        bfs_iter::{BfsIter, BfsIterMut},
        dfs_iter::DfsIter,
    },
    graph::{GraphBase, WithID},
    Graph, GraphError,
};
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Specifies which graph traversal algorithm to use
#[derive(Debug, Clone, Copy, Default)]
pub enum TraversalType {
    /// Breadth-first search traversal
    #[default]
    BFS,
    /// Depth-first search traversal
    DFS,
}

impl Display for TraversalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraversalType::BFS => write!(f, "BFS"),
            TraversalType::DFS => write!(f, "DFS"),
        }
    }
}

/// A wrapper enum around different graph iterator implementations
pub enum GraphIter<'a, Vertex: 'a, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: WithID,
{
    BFS(BfsIter<'a, Vertex, Edge, Dir, Backend>),
    DFS(DfsIter<'a, Vertex, Edge, Dir, Backend>),
}

impl<'a, Vertex, Edge, Dir, Backend> Iterator for GraphIter<'a, Vertex, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: 'a + WithID + Debug,
    Vertex::IDType: Eq + Hash + Copy,
    Edge: Debug,
{
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BFS(iter) => iter.next(),
            Self::DFS(iter) => iter.next(),
        }
    }
}

impl<'a, Vertex, Edge, Dir, Backend> Graph<Vertex, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: 'a + WithID + Debug,
    Vertex::IDType: Eq + Hash + Copy,
    Edge: Debug,
{
    /// Creates an iterator that traverses the graph starting from the given vertex
    /// using the specified traversal algorithm.
    pub fn iter(
        &'a self,
        start_vertex: Vertex::IDType,
        iter_type: TraversalType,
    ) -> Result<GraphIter<'a, Vertex, Edge, Dir, Backend>, GraphError<Vertex::IDType>> {
        match iter_type {
            TraversalType::BFS => Ok(GraphIter::BFS(self.bfs_iter(start_vertex)?)),
            TraversalType::DFS => Ok(GraphIter::DFS(self.dfs_iter(start_vertex)?)),
        }
    }
}

/// A wrapper enum around different graph iterator implementations
pub enum GraphIterMut<'a, Vertex: 'a, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: WithID,
{
    BFS(BfsIterMut<'a, Vertex, Edge, Dir, Backend>),
}

impl<'a, Vertex, Edge, Dir, Backend> Iterator for GraphIterMut<'a, Vertex, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: 'a + WithID + Debug,
    Vertex::IDType: Eq + Hash + Copy,
    Edge: Debug,
{
    type Item = &'a mut Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BFS(iter) => iter.next(),
        }
    }
}

impl<'a, Vertex, Edge, Dir, Backend> Graph<Vertex, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: 'a + WithID + Debug,
    Vertex::IDType: Eq + Hash + Copy,
    Edge: Debug,
{
    /// Creates an iterator that traverses the graph starting from the given vertex
    /// using the specified traversal algorithm.
    pub fn iter_mut(
        &'a mut self,
        start_vertex: Vertex::IDType,
        iter_type: TraversalType,
    ) -> Result<GraphIterMut<'a, Vertex, Edge, Dir, Backend>, GraphError<Vertex::IDType>> {
        match iter_type {
            TraversalType::BFS => Ok(GraphIterMut::BFS(self.bfs_iter_mut(start_vertex)?)),
            TraversalType::DFS => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::{
        algorithms::iter::TraversalType,
        graph::{Directed, GraphBase, ListGraph, WithID},
        Graph, GraphError,
    };
    use std::collections::HashSet;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestVertex {
        id: usize,
        value: String,
    }

    impl WithID for TestVertex {
        type IDType = usize;

        fn get_id(&self) -> usize {
            self.id
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestEdge {
        weight: usize,
    }

    #[fixture]
    fn create_test_graph() -> ListGraph<TestVertex, TestEdge, Directed> {
        // Create a graph with the following structure:
        //    0
        //   / \
        //  1   2
        //     / \
        //    3   4
        //   /
        //  5
        let mut graph = Graph::new();

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
    #[case(
        TraversalType::BFS, 
        vec![0, 1, 2, 3, 4, 5], // expected from 0
        vec![2, 3, 4, 5]        // expected from 2
    )]
    #[case(
        TraversalType::DFS, 
        vec![0, 2, 4, 3, 5, 1], // expected from 0
        vec![2, 4, 3, 5]        // expected from 2

    )]
    fn test_iter_traversal_order(
        create_test_graph: ListGraph<TestVertex, TestEdge, Directed>,
        #[case] traversal_type: TraversalType,
        #[case] expected_from_0: Vec<usize>,
        #[case] expected_from_2: Vec<usize>,
    ) {
        let graph = create_test_graph;

        // Start traversal from vertex 0
        let iter = graph.iter(0, traversal_type).unwrap();
        let visited_ids: Vec<usize> = iter.map(|v| v.get_id()).collect();
        assert_eq!(visited_ids, expected_from_0);

        // Start traversal from vertex 2
        let iter = graph.iter(2, traversal_type).unwrap();
        let visited_ids: Vec<usize> = iter.map(|v| v.get_id()).collect();
        assert_eq!(visited_ids, expected_from_2);
    }

    #[rstest]
    fn test_iter_traversal_subset(
        create_test_graph: ListGraph<TestVertex, TestEdge, Directed>,
        #[values(TraversalType::BFS, TraversalType::DFS)] traversal_type: TraversalType,
    ) {
        let graph = create_test_graph;

        // Start from vertex 3, should only visit 3 and 5
        let bfs = graph.iter(3, traversal_type).unwrap();
        let visited_ids: HashSet<usize> = bfs.map(|v| v.get_id()).collect();

        assert_eq!(visited_ids, HashSet::from([3, 5]));
    }

    #[rstest]
    fn test_iter_mut_traversal(
        create_test_graph: ListGraph<TestVertex, TestEdge, Directed>,
        #[values(
            TraversalType::BFS, 
            // TraversalType::DFS // Not yet implemented
        )] traversal_type: TraversalType,
    ) {
        let mut graph = create_test_graph;

        // Use mutable BFS to modify vertex values
        {
            let bfs_mut = graph.iter_mut(0, traversal_type).unwrap();
            for vertex in bfs_mut {
                vertex.value = format!("Modified_{}", vertex.value);
            }
        }

        // Verify all vertices were modified
        assert_eq!(graph.get_vertex_by_id(0).unwrap().value, "Modified_A");
        assert_eq!(graph.get_vertex_by_id(1).unwrap().value, "Modified_B");
        assert_eq!(graph.get_vertex_by_id(2).unwrap().value, "Modified_C");
        assert_eq!(graph.get_vertex_by_id(3).unwrap().value, "Modified_D");
        assert_eq!(graph.get_vertex_by_id(4).unwrap().value, "Modified_E");
        assert_eq!(graph.get_vertex_by_id(5).unwrap().value, "Modified_F");
    }

    #[rstest]
    fn test_iter_invalid_start(
        #[values(TraversalType::BFS, TraversalType::DFS)] traversal_type: TraversalType,
    ) {
        let graph: ListGraph<TestVertex, (), Directed> = Graph::new();

        // Try to start BFS from non-existent vertex
        let result = graph.iter(999, traversal_type);
        assert!(result.is_err());

        if let Err(GraphError::VertexNotFound(id)) = result {
            assert_eq!(id, 999);
        } else {
            panic!("Expected VertexNotFound error");
        }
    }

    #[rstest]
    fn test_iter_isolated_vertex(
        #[values(TraversalType::BFS, TraversalType::DFS)] traversal_type: TraversalType,
    ) {
        let mut graph: ListGraph<TestVertex, (), Directed> = Graph::new();

        // Add a single isolated vertex
        graph
            .push_vertex(TestVertex {
                id: 42,
                value: "Isolated".to_string(),
            })
            .unwrap();

        // BFS should visit only this vertex
        let bfs = graph.iter(42, traversal_type).unwrap();
        let visited: Vec<usize> = bfs.map(|v| v.get_id()).collect();

        assert_eq!(visited, vec![42]);
    }

    #[rstest]
    fn test_iter_cycle(
        #[values(TraversalType::BFS, TraversalType::DFS)] traversal_type: TraversalType,
    ) {
        let mut graph: ListGraph<TestVertex, TestEdge, Directed> = Graph::new();

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
        let bfs = graph.iter(0, traversal_type).unwrap();
        let visited: Vec<usize> = bfs.map(|v| v.get_id()).collect();

        assert_eq!(visited.len(), 3);
        assert_eq!(visited[0], 0);
        // 1 and 2 might be visited in any order depending on graph implementation
        assert!(visited.contains(&1));
        assert!(visited.contains(&2));
    }
}
