use std::{fmt::Debug, hash::Hash, marker::PhantomData, vec};

use rustc_hash::FxHashSet;

use crate::{
    graph::{GraphBase, WithID},
    Graph, GraphError,
};

pub struct DfsIter<'a, Vertex: 'a, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: WithID,
{
    graph: &'a Graph<Vertex, Edge, Dir, Backend>,
    stack: Vec<Vertex::IDType>,
    visited: FxHashSet<Vertex::IDType>,
    _phantom: PhantomData<&'a Edge>,
}

impl<'a, Vertex: 'a, Edge, Dir, Backend> DfsIter<'a, Vertex, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: WithID + Debug,
    Vertex::IDType: Eq + Hash + Copy,
    Edge: Debug,
{
    fn new(
        graph: &'a Graph<Vertex, Edge, Dir, Backend>,
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

impl<'a, Vertex, Edge, Dir, Backend> Iterator for DfsIter<'a, Vertex, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: 'a + WithID + Debug,
    Vertex::IDType: Eq + Hash + Copy,
    Edge: Debug,
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

impl<'a, Vertex, Edge, Dir, Backend> Graph<Vertex, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: 'a + WithID + Debug,
    Vertex::IDType: Eq + Hash + Copy,
    Edge: Debug,
{
    pub fn dfs_iter(
        &'a self,
        start_vertex: Vertex::IDType,
    ) -> Result<DfsIter<'a, Vertex, Edge, Dir, Backend>, GraphError<Vertex::IDType>> {
        DfsIter::new(self, start_vertex)
    }
}
