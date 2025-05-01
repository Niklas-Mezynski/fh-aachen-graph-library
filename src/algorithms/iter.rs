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
