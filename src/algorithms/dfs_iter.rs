use std::{fmt::Debug, hash::Hash, vec};

use rustc_hash::FxHashSet;

use crate::{graph::WithID, Graph, GraphError};

pub struct DfsIter<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy ,
    Vertex: WithID<VId> ,
    Edge: ,
{
    graph: &'a Graph<VId, Vertex, Edge>,
    stack: Vec<VId>,
    visited: FxHashSet<VId>,
}

impl<'a, VId, Vertex, Edge> DfsIter<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    fn new(
        graph: &'a Graph<VId, Vertex, Edge>,
        start_vertex: VId,
    ) -> Result<Self, GraphError<VId>> {
        let _ = graph.get_vertex_by_id(&start_vertex)?; // Check if it exists

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

impl<'a, VId, Vertex, Edge> Iterator for DfsIter<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug ,
    Vertex: WithID<VId> ,
    Edge: Clone ,
{
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_id) = self.stack.pop() {
            // Get the current vertex first
            let current_vertex = self.graph.get_vertex_by_id(&next_id).expect(
                "get_vertex_by_id should not error as the vertices in the stack must exist",
            );

            // Add unvisited neighbors to stack (back) for depth-first traversal
            let neighbors = self.graph.get_adjacent_vertices(&next_id).expect(
                "get_adjacent_vertices should not error as the vertices in the stack must exist",
            );

            // In DFS, we want to explore the most recently added vertices first
            for v in neighbors {
                let vid = v.get_id();
                if !self.visited.contains(&vid) {
                    self.visited.insert(vid);
                    self.stack.push(vid); // Push to back for LIFO behavior
                }
            }

            // Return the current vertex
            Some(current_vertex)
        } else {
            None
        }
    }
}

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    pub fn dfs_iter(
        &self,
        start_vertex: VId,
    ) -> Result<DfsIter<VId, Vertex, Edge>, GraphError<VId>> {
        DfsIter::new(self, start_vertex)
    }
}
