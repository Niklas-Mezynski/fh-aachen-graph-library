use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::{cmp::Reverse, collections::BinaryHeap, hash::Hash};

use crate::{
    graph::{GraphBase, WeightedEdge, WithID},
    Graph,
};

use super::single_source_shortest_paths::SingleSourceShortestPaths;

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Copy + Eq + Hash,
    Backend::Edge: WeightedEdge + Clone,
    <Backend::Edge as WeightedEdge>::WeightType: Copy,
{
    /// Dijkstra's shortest path algorithm.
    ///
    /// Compute the length of the shortest path from `start` to every reachable node.
    ///
    /// Returns a tuple with a `HashMap` that maps `VertexID` to path cost and
    /// a `HashMap` that maps `VertexID` to the predecessor `VertexID` that can be used to reconstruct the path.
    ///
    pub fn dijkstra(
        &self,
        start: <Backend::Vertex as WithID>::IDType,
        goal: Option<<Backend::Vertex as WithID>::IDType>,
    ) -> SingleSourceShortestPaths<
        <Backend::Vertex as WithID>::IDType,
        <Backend::Edge as WeightedEdge>::WeightType,
    > {
        // Final map of costs from start to each v
        let mut costs = FxHashMap::default();
        // Which vertex was visited before each other. Can be used to reconstruct the exact path
        let mut predecessor = FxHashMap::default();
        // Track visited vertices
        let mut visited = FxHashSet::default();
        // Keep track of which vertex to visit next, by storing in a ordered data structure ("cheapest" first)
        let mut visit_next = BinaryHeap::new();

        // Initialize the cost to the start vertex with 0
        costs.insert(
            start,
            <Backend::Edge as WeightedEdge>::WeightType::default(),
        );
        visit_next.push(Reverse(EdgeEntry::new(
            <Backend::Edge as WeightedEdge>::WeightType::default(),
            start,
        )));

        // For each cheapest, reachable node
        while let Some(Reverse(node_entry)) = visit_next.pop() {
            // If already visited, continue
            if visited.contains(&node_entry.vertex_id) {
                continue;
            }

            // If we are visiting the goal node, we can early stop as we already computed the shortest path to it
            if goal.as_ref() == Some(&node_entry.vertex_id) {
                break;
            }

            // For each (unvisited) adjacent vertex, check if we can improve the cost
            for (next_v, edge) in self
                .get_adjacent_vertices_with_edges(node_entry.vertex_id)
                .map(|(v, e)| (v.get_id(), e))
                .filter(|(v, _e)| !visited.contains(v))
            {
                let new_cost = node_entry.cost + edge.get_weight();
                match costs.entry(next_v) {
                    Occupied(existing_entry) => {
                        // Check if we the cost to `next_v` can be improved
                        if new_cost < *existing_entry.get() {
                            *existing_entry.into_mut() = new_cost;
                            visit_next.push(Reverse(EdgeEntry::new(new_cost, next_v)));
                            predecessor.insert(next_v, node_entry.vertex_id);
                        }
                    }
                    Vacant(new_entry) => {
                        // First time we visit `next_v` -> just insert the cost
                        new_entry.insert(new_cost);
                        visit_next.push(Reverse(EdgeEntry::new(new_cost, next_v)));
                        predecessor.insert(next_v, node_entry.vertex_id);
                    }
                }
            }
            visited.insert(node_entry.vertex_id);
        }

        SingleSourceShortestPaths::new(start, costs, predecessor)
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
