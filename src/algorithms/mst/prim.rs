use std::{cmp::Reverse, collections::BinaryHeap, fmt::Debug, hash::Hash};

use rustc_hash::FxHashSet;

use crate::{
    graph::{WeightedEdge, WithID},
    Graph, GraphError,
};

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + PartialOrd + Debug + Copy,
    Vertex: WithID<VId> + Clone,
    Edge: WeightedEdge + Clone,
{
    /// Creates an MST using the Prim algorithm.
    ///
    /// Returns the MST as a new graph
    pub fn mst_prim(&self) -> Result<Graph<VId, Vertex, Edge>, GraphError<VId>> {
        let is_directed = self.is_directed();
        let mut mst_graph = Graph::<VId, Vertex, Edge>::new(is_directed);

        // Priority queue
        let mut edge_pq = BinaryHeap::new();

        // Step 1: Take an initial vertex from the graph
        let mut vertices_iter = self.get_all_vertices().into_iter();
        let v0 = match vertices_iter.next() {
            Some(v) => v,
            // Wenn der Graph leer ist -> stopp
            None => return Ok(mst_graph),
        };
        let start_id = v0.get_id();

        mst_graph.push_vertex(v0.clone())?;

        // Store all vertices, that still have to be processed
        let mut remaining_vertices = vertices_iter.map(|v| v.get_id()).collect::<FxHashSet<_>>();

        // Add initial edges from the start vertex to the priority queue
        for (neighbor_vertex, edge) in self.get_adjacent_vertices_with_edges(&start_id) {
            let weight = edge.get_weight();
            edge_pq.push(EdgeEntry::new(
                // Reverse is used to make BinaryHeap behave as a min-heap based on weight
                Reverse(weight),
                start_id,
                neighbor_vertex.get_id(),
                edge,
            ));
        }

        // Step 2: Loop while the new mst graph does not contain all vertices from the original graph
        while !remaining_vertices.is_empty() {
            //   Step (a): Choose the cheapest edge
            let cheapest = match edge_pq.pop() {
                Some(entry) => entry,
                None => break, // No more reachable vertices
            };

            // If the edge has already been visited -> skip
            if !remaining_vertices.remove(&cheapest.to) {
                continue;
            }

            // Step (b): Add the edge and the now reachable vertex to the new mst graph
            mst_graph.push_vertex(
                self.get_vertex_by_id(&cheapest.to)
                    .expect("vertex must exist")
                    .to_owned(),
            )?;
            mst_graph.push_edge(cheapest.from, cheapest.to, cheapest.edge.to_owned())?;

            // Also add the now reachable edges to the priority queue
            for (neighbor_vertex, next_edge) in self.get_adjacent_vertices_with_edges(&cheapest.to)
            {
                let neighbor_id = neighbor_vertex.get_id();
                // Skip if we already added that vertex
                if !remaining_vertices.contains(&neighbor_id) {
                    continue;
                }

                let next_weight = next_edge.get_weight();
                edge_pq.push(EdgeEntry::new(
                    Reverse(next_weight),
                    cheapest.to,
                    neighbor_id,
                    next_edge,
                ));
            }
        }

        Ok(mst_graph)
    }
}

// Helper struct for Min-Heap behavior if weights are floats or need custom ordering
struct EdgeEntry<W: PartialOrd, VId, E> {
    weight: W,
    from: VId,
    to: VId,
    edge: E,
}

impl<W: PartialOrd, VId, E> EdgeEntry<W, VId, E> {
    pub fn new(weight: W, from: VId, to: VId, edge: E) -> Self {
        EdgeEntry {
            weight,
            from,
            to,
            edge,
        }
    }
}

impl<W: PartialOrd, VId, E> PartialEq for EdgeEntry<W, VId, E> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<W: PartialOrd, VId, E> Eq for EdgeEntry<W, VId, E> {}

impl<W: PartialOrd, VId, E> PartialOrd for EdgeEntry<W, VId, E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<W: PartialOrd, VId, E> Ord for EdgeEntry<W, VId, E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight
            .partial_cmp(&other.weight)
            .expect("Graph weights must not contain NaN values")
    }
}
