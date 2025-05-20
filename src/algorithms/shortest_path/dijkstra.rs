use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::{cmp::Reverse, collections::BinaryHeap, hash::Hash};

use crate::{
    graph::{GraphBase, WeightedEdge, WithID},
    Graph,
};

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Copy + Eq + Hash,
    Backend::Edge: WeightedEdge + Clone,
    <Backend::Edge as WeightedEdge>::WeightType: Copy,
{
    // type VertexID = <Backend::Vertex as WithID>::IDType;
    // type EdgeWeight = <Backend::Edge as WeightedEdge>::WeightType;
    /// Dijkstra's shortest path algorithm.
    ///
    /// Compute the length of the shortest path from `start` to every reachable node.
    ///
    /// Returns a tuple with a `HashMap` that maps `VertexID` to path cost and
    /// a `HashMap` that maps `VertexID` to the predecessor `VertexID` that can be used to reconstruct the path.
    pub fn dijkstra(
        &self,
        start: <Backend::Vertex as WithID>::IDType,
        goal: Option<<Backend::Vertex as WithID>::IDType>,
    ) -> (
        FxHashMap<<Backend::Vertex as WithID>::IDType, <Backend::Edge as WeightedEdge>::WeightType>,
        FxHashMap<<Backend::Vertex as WithID>::IDType, <Backend::Vertex as WithID>::IDType>,
    ) {
        let mut visited = FxHashSet::default();
        let mut costs = FxHashMap::default();
        let mut predecessor = FxHashMap::default();
        let mut visit_next = BinaryHeap::new();

        costs.insert(
            start,
            <Backend::Edge as WeightedEdge>::WeightType::default(),
        );
        visit_next.push(Reverse(EdgeEntry::new(
            <Backend::Edge as WeightedEdge>::WeightType::default(),
            start,
        )));

        while let Some(Reverse(node_entry)) = visit_next.pop() {
            if visited.contains(&node_entry.vertex_id) {
                continue;
            }

            // If we are visiting the goal node, we can stop as we already computed the shortest path to it
            if goal.as_ref() == Some(&node_entry.vertex_id) {
                break;
            }

            // For each (unvisited) adjacent vertex, check if we can improve the cost
            // TODO: Filter elements in the iterator
            for (next_v, edge) in self.get_adjacent_vertices_with_edges(node_entry.vertex_id) {
                let next_v = next_v.get_id();
                if visited.contains(&next_v) {
                    continue;
                }
                let new_cost = node_entry.cost + edge.get_weight();
                match costs.entry(next_v) {
                    Occupied(existing_entry) => {
                        if new_cost < *existing_entry.get() {
                            *existing_entry.into_mut() = new_cost;
                            visit_next.push(Reverse(EdgeEntry::new(new_cost, next_v)));
                            predecessor.insert(next_v, node_entry.vertex_id);
                        }
                    }
                    Vacant(new_entry) => {
                        new_entry.insert(new_cost);
                        visit_next.push(Reverse(EdgeEntry::new(new_cost, next_v)));
                        predecessor.insert(next_v, node_entry.vertex_id);
                    }
                }
            }
            visited.insert(node_entry.vertex_id);
        }

        (costs, predecessor)
    }
}

/// Helper struct for Min-Heap behavior if weights are floats or need custom ordering
struct EdgeEntry<W: PartialOrd, VId> {
    cost: W,
    vertex_id: VId,
}

impl<W: PartialOrd, VId> EdgeEntry<W, VId> {
    pub fn new(cost: W, vertex_id: VId) -> Self {
        EdgeEntry { cost, vertex_id }
    }
}

impl<W: PartialOrd, VId> PartialEq for EdgeEntry<W, VId> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<W: PartialOrd, VId> Eq for EdgeEntry<W, VId> {}

impl<W: PartialOrd, VId> PartialOrd for EdgeEntry<W, VId> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<W: PartialOrd, VId> Ord for EdgeEntry<W, VId> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .partial_cmp(&other.cost)
            .expect("Graph weights must not contain NaN values")
    }
}
