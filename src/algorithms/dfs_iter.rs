use rustc_hash::FxHashSet;
use std::hash::Hash;

use crate::{
    graph::{GraphBase, WithID},
    Graph, GraphError,
};

pub struct DfsIter<'a, Backend>
where
    Backend: GraphBase,
{
    graph: &'a Graph<Backend>,
    stack: Vec<<Backend::Vertex as WithID>::IDType>,
    visited: FxHashSet<<Backend::Vertex as WithID>::IDType>,
}

impl<'a, Backend> DfsIter<'a, Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Eq + Hash + Copy,
{
    fn new(
        graph: &'a Graph<Backend>,
        start_vertex: <Backend::Vertex as WithID>::IDType,
    ) -> Result<Self, GraphError<<Backend::Vertex as WithID>::IDType>> {
        graph
            .get_vertex_by_id(start_vertex)
            .ok_or(GraphError::VertexNotFound(start_vertex))?;

        let stack = vec![start_vertex];

        let mut visited = FxHashSet::default();
        visited.insert(start_vertex);

        Ok(DfsIter {
            graph,
            stack,
            visited,
        })
    }
}

impl<'a, Backend> Iterator for DfsIter<'a, Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Eq + Hash + Copy,
{
    type Item = &'a Backend::Vertex;

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

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Eq + Hash + Copy,
{
    pub fn dfs_iter(
        &self,
        start_vertex: <Backend::Vertex as WithID>::IDType,
    ) -> Result<DfsIter<'_, Backend>, GraphError<<Backend::Vertex as WithID>::IDType>> {
        DfsIter::new(self, start_vertex)
    }
}
