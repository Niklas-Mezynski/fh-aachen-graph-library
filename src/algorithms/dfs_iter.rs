use std::{collections::VecDeque, fmt::Debug, hash::Hash};

use rustc_hash::FxHashSet;

use crate::{graph::WithID, Graph, GraphError};

pub struct DfsIterator<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + 'static,
    Vertex: WithID<VId> + 'static,
    Edge: 'static,
{
    graph: &'a Graph<VId, Vertex, Edge>,
    stack: VecDeque<VId>,
    visited: FxHashSet<VId>,
}

impl<'a, VId, Vertex, Edge> DfsIterator<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    fn new(
        graph: &'a Graph<VId, Vertex, Edge>,
        start_vertex: VId,
    ) -> Result<Self, GraphError<VId>> {
        let _ = graph.get_vertex_by_id(&start_vertex)?; // Check if it exists

        let stack = VecDeque::from([start_vertex]);

        let mut visited = FxHashSet::default();
        visited.insert(start_vertex);

        Ok(DfsIterator {
            graph,
            stack,
            visited,
        })
    }
}

impl<'a, VId, Vertex, Edge> Iterator for DfsIterator<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug + 'static,
    Vertex: WithID<VId> + 'static,
    Edge: Clone + 'static,
{
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_id) = self.stack.pop_back() {
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
                    self.stack.push_back(vid); // Push to back for LIFO behavior
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
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    pub fn dfs_iter(
        &self,
        start_vertex: VId,
    ) -> Result<DfsIterator<VId, Vertex, Edge>, GraphError<VId>> {
        DfsIterator::new(self, start_vertex)
    }
}

pub struct DfsRecursiveIterator<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + 'static,
    Vertex: WithID<VId> + 'static,
    Edge: 'static,
{
    graph: &'a Graph<VId, Vertex, Edge>,
    final_queue: VecDeque<VId>,
}

impl<'a, VId, Vertex, Edge> DfsRecursiveIterator<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    /// Careful! this iterator constructor already traverses the whole graph, so this operation may take a while
    fn new(
        graph: &'a Graph<VId, Vertex, Edge>,
        start_vertex_id: VId,
    ) -> Result<Self, GraphError<VId>> {
        let _ = graph.get_vertex_by_id(&start_vertex_id)?; // Check if it exists

        let mut final_queue: VecDeque<VId> = VecDeque::new();

        let mut visited: FxHashSet<VId> = FxHashSet::default();

        dfs(graph, start_vertex_id, &mut final_queue, &mut visited);

        Ok(DfsRecursiveIterator { graph, final_queue })
    }
}

fn dfs<VId, Vertex, Edge>(
    graph: &Graph<VId, Vertex, Edge>,
    vertex_id: VId,
    final_queue: &mut VecDeque<VId>,
    visited: &mut FxHashSet<VId>,
) where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    visited.insert(vertex_id);
    final_queue.push_back(vertex_id);

    let neighbors = graph
        .get_adjacent_vertices(&vertex_id)
        .expect("get_adjacent_vertices should not error as the vertices in the stack must exist");

    for v in neighbors {
        let vid = v.get_id();
        if !visited.contains(&vid) {
            dfs(graph, vertex_id, final_queue, visited);
        }
    }
}

impl<'a, VId, Vertex, Edge> Iterator for DfsRecursiveIterator<'a, VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug + 'static,
    Vertex: WithID<VId> + 'static,
    Edge: Clone + 'static,
{
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        self.final_queue.pop_front().map(|vid| {
            self.graph
                .get_vertex_by_id(&vid)
                .expect("Vertex must exist as it was discovered during graph traversal")
        })
    }
}

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Copy + Debug,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    pub fn dfs_recursive_iter(
        &self,
        start_vertex: VId,
    ) -> Result<DfsRecursiveIterator<VId, Vertex, Edge>, GraphError<VId>> {
        DfsRecursiveIterator::new(self, start_vertex)
    }
}
