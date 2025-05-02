use std::{collections::VecDeque, hash::Hash, marker::PhantomData};

use rustc_hash::FxHashSet;

use crate::{
    graph::{GraphBase, WithID},
    Graph, GraphError,
};

pub struct BfsIter<'a, Backend>
where
    Backend: GraphBase,
{
    graph: &'a Graph<Backend>,
    queue: VecDeque<<Backend::Vertex as WithID>::IDType>,
    visited: FxHashSet<<Backend::Vertex as WithID>::IDType>,
}

impl<'a, Backend> BfsIter<'a, Backend>
where
    Backend: GraphBase,
    Backend::Vertex: WithID,
    <Backend::Vertex as WithID>::IDType: Eq + Hash + Copy,
{
    fn new(
        graph: &'a Graph<Backend>,
        start_vertex: <Backend::Vertex as WithID>::IDType,
    ) -> Result<Self, GraphError<<Backend::Vertex as WithID>::IDType>> {
        graph
            .get_vertex_by_id(start_vertex)
            .ok_or_else(|| GraphError::VertexNotFound(start_vertex))?;

        let queue = VecDeque::from([start_vertex]);

        let mut visited = FxHashSet::default();
        visited.insert(start_vertex);

        Ok(BfsIter {
            graph,
            queue,
            visited,
        })
    }
}

impl<'a, Backend> Iterator for BfsIter<'a, Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Eq + Hash + Copy,
{
    type Item = &'a Backend::Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_id) = self.queue.pop_front() {
            let neighbors = self.graph.get_adjacent_vertices(next_id);

            for v in neighbors {
                let vid = v.get_id();
                if !self.visited.contains(&vid) {
                    self.visited.insert(vid);
                    self.queue.push_back(vid);
                }
            }

            Some(self.graph.get_vertex_by_id(next_id).expect(
                "get_vertex_by_id should not error as the vertices in the queue must exist",
            ))
        } else {
            None
        }
    }
}

pub struct BfsIterMut<'a, Backend>
where
    Backend: GraphBase,
{
    graph: &'a mut Graph<Backend>,
    queue: VecDeque<<Backend::Vertex as WithID>::IDType>,
    visited: FxHashSet<<Backend::Vertex as WithID>::IDType>,
    _phantom: PhantomData<&'a Backend::Edge>,
}

impl<'a, Backend> BfsIterMut<'a, Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Eq + Hash + Copy,
{
    fn new(
        graph: &'a mut Graph<Backend>,
        start_vertex: <Backend::Vertex as WithID>::IDType,
    ) -> Result<Self, GraphError<<Backend::Vertex as WithID>::IDType>> {
        graph
            .get_vertex_by_id(start_vertex)
            .ok_or_else(|| GraphError::VertexNotFound(start_vertex))?;

        let queue = VecDeque::from([start_vertex]);

        let mut visited = FxHashSet::default();
        visited.insert(start_vertex);

        Ok(BfsIterMut {
            graph,
            queue,
            visited,
            _phantom: PhantomData,
        })
    }
}

impl<'a, Backend> Iterator for BfsIterMut<'a, Backend>
where
    Backend: GraphBase,
    Backend::Vertex: 'a + WithID,
    <Backend::Vertex as WithID>::IDType: Eq + Hash + Copy,
{
    type Item = &'a mut Backend::Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_id) = self.queue.pop_front() {
            let neighbors = self.graph.get_adjacent_vertices(next_id);

            for v in neighbors {
                let vid = v.get_id();
                if !self.visited.contains(&vid) {
                    self.visited.insert(vid);
                    self.queue.push_back(vid);
                }
            }

            // SAFETY: This is safe because:
            // 1. We only return one mutable reference at a time
            // 2. Each vertex is visited exactly once (tracked by the visited set)
            // 3. The reference doesn't outlive the graph (tied to lifetime 'a)
            unsafe {
                let vertex_ptr = self.graph.get_vertex_by_id_mut(next_id).expect(
                    "get_vertex_by_id_mut should not error as the vertices in the queue must exist",
                ) as *mut Backend::Vertex;

                Some(&mut *vertex_ptr)
            }
        } else {
            None
        }
    }
}

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    Backend::Vertex: WithID,
    <Backend::Vertex as WithID>::IDType: Eq + Hash + Copy,
{
    pub fn bfs_iter(
        &self,
        start_vertex: <Backend::Vertex as WithID>::IDType,
    ) -> Result<BfsIter<'_, Backend>, GraphError<<Backend::Vertex as WithID>::IDType>> {
        BfsIter::new(self, start_vertex)
    }

    pub fn bfs_iter_mut(
        &mut self,
        start_vertex: <Backend::Vertex as WithID>::IDType,
    ) -> Result<BfsIterMut<'_, Backend>, GraphError<<Backend::Vertex as WithID>::IDType>>
    where
        Backend::Vertex: WithID,
        <Backend::Vertex as WithID>::IDType: Eq + Hash + Copy,
    {
        BfsIterMut::new(self, start_vertex)
    }
}
