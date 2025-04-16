use crate::{
    algorithms::{bfs_iter::BfsIter, dfs_iter::DfsIter},
    graph::WithID,
    Graph, GraphError,
};
use std::fmt::Display;
use std::{fmt::Debug, hash::Hash};

use super::bfs_iter::BfsIterMut;

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
pub enum GraphIter<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID<VId>,
    Edge:,
{
    BFS(BfsIter<'a, VId, Vertex, Edge>),
    DFS(DfsIter<'a, VId, Vertex, Edge>),
}

impl<'a, VId, Vertex, Edge> Iterator for GraphIter<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
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
    VId: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    /// Creates an iterator that traverses the graph starting from the given vertex
    /// using the specified traversal algorithm.
    pub fn iter(
        &self,
        start_vertex: VId,
        iter_type: TraversalType,
    ) -> Result<GraphIter<VId, Vertex, Edge>, GraphError<VId>> {
        match iter_type {
            TraversalType::BFS => Ok(GraphIter::BFS(self.bfs_iter(start_vertex)?)),
            TraversalType::DFS => Ok(GraphIter::DFS(self.dfs_iter(start_vertex)?)),
        }
    }
}

/// A wrapper enum around different graph iterator implementations
pub enum GraphIterMut<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID<VId>,
    Edge:,
{
    BFS(BfsIterMut<'a, VId, Vertex, Edge>),
}

impl<'a, VId, Vertex, Edge> Iterator for GraphIterMut<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    type Item = &'a mut Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BFS(iter) => iter.next(),
        }
    }
}

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    /// Creates an iterator that traverses the graph starting from the given vertex
    /// using the specified traversal algorithm.
    pub fn iter_mut(
        &mut self,
        start_vertex: VId,
        iter_type: TraversalType,
    ) -> Result<GraphIterMut<VId, Vertex, Edge>, GraphError<VId>> {
        match iter_type {
            TraversalType::BFS => Ok(GraphIterMut::BFS(self.bfs_iter_mut(start_vertex)?)),
            TraversalType::DFS => todo!(),
        }
    }
}
