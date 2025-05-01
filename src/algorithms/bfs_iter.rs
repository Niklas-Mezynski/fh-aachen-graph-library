use std::{collections::VecDeque, fmt::Debug, hash::Hash, marker::PhantomData};

use rustc_hash::FxHashSet;

use crate::{
    graph::{GraphBase, WithID},
    Graph, GraphError,
};

pub struct BfsIter<'a, Vertex: 'a, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: WithID,
{
    graph: &'a Graph<Vertex, Edge, Dir, Backend>,
    queue: VecDeque<Vertex::IDType>,
    visited: FxHashSet<Vertex::IDType>,
}

impl<'a, Vertex: 'a, Edge, Dir, Backend> BfsIter<'a, Vertex, Edge, Dir, Backend>
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

impl<'a, Vertex, Edge, Dir, Backend> Iterator for BfsIter<'a, Vertex, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: 'a + WithID + Debug,
    Vertex::IDType: Eq + Hash + Copy,
    Edge: Debug,
{
    type Item = &'a Vertex;

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

pub struct BfsIterMut<'a, Vertex: 'a, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: WithID,
{
    graph: &'a mut Graph<Vertex, Edge, Dir, Backend>,
    queue: VecDeque<Vertex::IDType>,
    visited: FxHashSet<Vertex::IDType>,
    _phantom: PhantomData<&'a Edge>,
}

impl<'a, Vertex: 'a, Edge, Dir, Backend> BfsIterMut<'a, Vertex, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: WithID + Debug,
    Vertex::IDType: Eq + Hash + Copy,
    Edge: Debug,
{
    fn new(
        graph: &'a mut Graph<Vertex, Edge, Dir, Backend>,
        start_vertex: Vertex::IDType,
    ) -> Result<Self, GraphError<Vertex::IDType>> {
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

impl<'a, Vertex, Edge, Dir, Backend> Iterator for BfsIterMut<'a, Vertex, Edge, Dir, Backend>
where
    Backend: GraphBase<Vertex, Edge, Dir>,
    Vertex: 'a + WithID + Debug,
    Vertex::IDType: Eq + Hash + Copy,
    Edge: Debug,
{
    type Item = &'a mut Vertex;

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
                ) as *mut Vertex;

                Some(&mut *vertex_ptr)
            }
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
    pub fn bfs_iter(
        &'a self,
        start_vertex: Vertex::IDType,
    ) -> Result<BfsIter<'a, Vertex, Edge, Dir, Backend>, GraphError<Vertex::IDType>> {
        BfsIter::new(self, start_vertex)
    }

    pub fn bfs_iter_mut(
        &'a mut self,
        start_vertex: Vertex::IDType,
    ) -> Result<BfsIterMut<'a, Vertex, Edge, Dir, Backend>, GraphError<Vertex::IDType>>
    where
        Backend: GraphBase<Vertex, Edge, Dir>,
        Vertex: 'a + WithID,
        Vertex::IDType: Eq + Hash + Copy,
    {
        BfsIterMut::new(self, start_vertex)
    }
}
