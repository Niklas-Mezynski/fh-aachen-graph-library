use crate::{
    algorithms::{
        bfs_iter::{BfsIter, BfsIterMut},
        dfs_iter::DfsIter,
    },
    graph::{GraphBase, WithID},
    Graph, GraphError,
};
use std::fmt::Display;
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
pub enum GraphIter<'a, Backend, Vertex: 'a, Edge>
where
    Backend: GraphBase<Vertex, Edge>,
    Vertex: WithID,
{
    BFS(BfsIter<'a, Backend, Vertex, Edge>),
    DFS(DfsIter<'a, Backend, Vertex, Edge>),
}

impl<'a, Backend, Vertex, Edge> Iterator for GraphIter<'a, Backend, Vertex, Edge>
where
    Backend: GraphBase<Vertex, Edge>,
    Vertex: 'a + WithID,
    Vertex::IDType: Eq + Hash + Copy,
{
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BFS(iter) => iter.next(),
            Self::DFS(iter) => iter.next(),
        }
    }
}

impl<Backend> Graph<Backend> {
    /// Creates an iterator that traverses the graph starting from the given vertex
    /// using the specified traversal algorithm.
    pub fn iter<'a, Vertex, Edge>(
        &'a self,
        start_vertex: Vertex::IDType,
        iter_type: TraversalType,
    ) -> Result<GraphIter<'a, Backend, Vertex, Edge>, GraphError<Vertex::IDType>>
    where
        Backend: GraphBase<Vertex, Edge>,
        Vertex: 'a + WithID,
        Vertex::IDType: Eq + Hash + Copy,
    {
        match iter_type {
            TraversalType::BFS => Ok(GraphIter::BFS(self.bfs_iter(start_vertex)?)),
            TraversalType::DFS => Ok(GraphIter::DFS(self.dfs_iter(start_vertex)?)),
        }
    }
}

/// A wrapper enum around different graph iterator implementations
pub enum GraphIterMut<'a, Backend, Vertex: 'a, Edge>
where
    Backend: GraphBase<Vertex, Edge>,
    Vertex: WithID,
{
    BFS(BfsIterMut<'a, Backend, Vertex, Edge>),
}

impl<'a, Backend, Vertex, Edge> Iterator for GraphIterMut<'a, Backend, Vertex, Edge>
where
    Backend: GraphBase<Vertex, Edge>,
    Vertex: 'a + WithID,
    Vertex::IDType: Eq + Hash + Copy,
{
    type Item = &'a mut Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BFS(iter) => iter.next(),
        }
    }
}

impl<Backend> Graph<Backend> {
    /// Creates an iterator that traverses the graph starting from the given vertex
    /// using the specified traversal algorithm.
    pub fn iter_mut<'a, Vertex, Edge>(
        &'a mut self,
        start_vertex: Vertex::IDType,
        iter_type: TraversalType,
    ) -> Result<GraphIterMut<'a, Backend, Vertex, Edge>, GraphError<Vertex::IDType>>
    where
        Backend: GraphBase<Vertex, Edge>,
        Vertex: 'a + WithID,
        Vertex::IDType: Eq + Hash + Copy,
    {
        match iter_type {
            TraversalType::BFS => Ok(GraphIterMut::BFS(self.bfs_iter_mut(start_vertex)?)),
            TraversalType::DFS => todo!(),
        }
    }
}
