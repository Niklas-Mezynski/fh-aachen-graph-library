use crate::{
    algorithms::{bfs_iter::BfsIterator, dfs_iter::DfsIterator},
    graph::WithID,
    Graph, GraphError,
};
use std::fmt::Display;
use std::{fmt::Debug, hash::Hash};

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
pub enum GraphIterator<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + 'static,
    Vertex: WithID<VId> + 'static,
    Edge: 'static,
{
    BFS(BfsIterator<'a, VId, Vertex, Edge>),
    DFS(DfsIterator<'a, VId, Vertex, Edge>),
}

impl<'a, VId, Vertex, Edge> Iterator for GraphIterator<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug + 'static,
    Vertex: WithID<VId> + 'static,
    Edge: Clone + 'static,
{
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BFS(iter) => iter.next(),
            Self::DFS(iter) => iter.next(),
        }
    }
}

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    /// Creates an iterator that traverses the graph starting from the given vertex
    /// using the specified traversal algorithm.
    pub fn iter(
        &self,
        start_vertex: VId,
        iter_type: TraversalType,
    ) -> Result<GraphIterator<VId, Vertex, Edge>, GraphError<VId>> {
        match iter_type {
            TraversalType::BFS => Ok(GraphIterator::BFS(self.bfs_iter(start_vertex)?)),
            TraversalType::DFS => Ok(GraphIterator::DFS(self.dfs_iter(start_vertex)?)),
        }
    }
}
