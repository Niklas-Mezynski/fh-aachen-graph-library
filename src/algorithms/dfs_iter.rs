use std::{hash::Hash, marker::PhantomData, vec};

use rustc_hash::FxHashSet;

use crate::{
    graph::{GraphBase, WithID},
    Graph, GraphError,
};

pub struct DfsIter<'a, Backend, Vertex: 'a, Edge>
where
    Backend: GraphBase<Vertex, Edge>,
    Vertex: WithID,
{
    graph: &'a Graph<Backend>,
    stack: Vec<Vertex::IDType>,
    visited: FxHashSet<Vertex::IDType>,
    _phantom: PhantomData<&'a Edge>,
}

impl<'a, Backend, Vertex: 'a, Edge> DfsIter<'a, Backend, Vertex, Edge>
where
    Backend: GraphBase<Vertex, Edge>,
    Vertex: WithID,
    Vertex::IDType: Eq + Hash + Copy,
{
    fn new(
        graph: &'a Graph<Backend>,
        start_vertex: Vertex::IDType,
    ) -> Result<Self, GraphError<Vertex::IDType>> {
        graph
            .get_vertex_by_id(start_vertex)
            .ok_or_else(|| GraphError::VertexNotFound(start_vertex))?;

        let stack = vec![start_vertex];

        let mut visited = FxHashSet::default();
        visited.insert(start_vertex);

        Ok(DfsIter {
            graph,
            stack,
            visited,
            _phantom: PhantomData,
        })
    }
}

impl<'a, Backend, Vertex, Edge> Iterator for DfsIter<'a, Backend, Vertex, Edge>
where
    Backend: GraphBase<Vertex, Edge>,
    Vertex: 'a + WithID,
    Vertex::IDType: Eq + Hash + Copy,
{
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_id) = self.stack.pop() {
            let current_vertex = self.graph.get_vertex_by_id(next_id).expect(
                "get_vertex_by_id should not error as the vertices in the stack must exist",
            );

            let neighbors = self.graph.get_adjacent_vertices(next_id);

            for v in neighbors {
                let vid = v.get_id();
                if !self.visited.contains(&vid) {
                    self.visited.insert(vid);
                    self.stack.push(vid);
                }
            }

            Some(current_vertex)
        } else {
            None
        }
    }
}

impl<Backend> Graph<Backend> {
    pub fn dfs_iter<'a, Vertex, Edge>(
        &'a self,
        start_vertex: Vertex::IDType,
    ) -> Result<DfsIter<'a, Backend, Vertex, Edge>, GraphError<Vertex::IDType>>
    where
        Backend: GraphBase<Vertex, Edge>,
        Vertex: 'a + WithID,
        Vertex::IDType: Eq + Hash + Copy,
    {
        DfsIter::new(self, start_vertex)
    }
}
