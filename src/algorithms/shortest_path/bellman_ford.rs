use enum_as_inner::EnumAsInner;
use rustc_hash::{FxHashMap, FxHashSet};
use std::hash::Hash;

use crate::{
    graph::{GraphBase, WeightedEdge, WithID},
    Directed, Graph,
};

use super::single_source_shortest_paths::SingleSourceShortestPaths;

#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum BellmanFordResult<VId: Hash + Eq, Cost> {
    SPT(SingleSourceShortestPaths<VId, Cost>),
    // Contains the vertices within the negative cycle
    NegativeCycle(Vec<VId>),
}

impl<Backend> Graph<Backend>
where
    Backend: GraphBase<Direction = Directed>,
    <Backend::Vertex as WithID>::IDType: Copy + Eq + Hash,
    Backend::Edge: WeightedEdge + Clone,
    <Backend::Edge as WeightedEdge>::WeightType: Copy,
{
    /// Bellman Ford's shortest path algorithm.
    ///
    /// Compute the length of the shortest path from `start` to every reachable node.
    ///
    /// The underlying implementation is a  Queue-Based-Bellman-Ford.
    /// I.e. instead of checking all edges in each iteration, it only checks outgoing edges
    /// of vertices that have changed since the last iteration
    ///
    /// Returns a tuple with a `HashMap` that maps `VertexID` to path cost and
    /// a `HashMap` that maps `VertexID` to the predecessor `VertexID` that can be used to reconstruct the path.
    #[allow(clippy::type_complexity)]
    pub fn bellman_ford(
        &self,
        start: <Backend::Vertex as WithID>::IDType,
    ) -> BellmanFordResult<
        <Backend::Vertex as WithID>::IDType,
        <Backend::Edge as WeightedEdge>::WeightType,
    > {
        // Final map of costs from start to each v
        let mut costs = FxHashMap::default();
        // Which vertex was visited before each other. Can be used to reconstruct the exact path
        let mut predecessor = FxHashMap::default();

        // Initialize the cost to the start vertex with 0
        costs.insert(
            start,
            <Backend::Edge as WeightedEdge>::WeightType::default(),
        );

        // Track the vertices, whose adjacent vertices we have to check in the next iteration
        // In the beginning, this is just the start vertex
        let mut vertices = vec![start];

        let n = self.vertex_count();
        // For |V| - 1 iterations, check all edges and see if we can decrease cost to any vertex
        for i in 1..=n {
            let mut changed_vertices = vec![];

            // Get all outgoing edges from `vertices`
            // We basically only check those vertices, where the cost has improved in the last iteration
            for (v, w, edge) in vertices.iter().flat_map(|v| {
                self.get_adjacent_vertices_with_edges(*v)
                    .map(|(w, e)| (*v, w.get_id(), e))
            }) {
                // Check if the edge (v, w) can improve the current "best" cost to vertex w
                let cost_v = costs.get(&v).copied();
                let cost_w = costs.get(&w).copied();
                let new_cost = match (cost_v, cost_w) {
                    (Some(cost_v), Some(cost_w)) => {
                        let new_cost = cost_v + edge.get_weight();
                        if new_cost < cost_w {
                            Some(new_cost)
                        } else {
                            None
                        }
                    }

                    (Some(cost_v), None) => {
                        let new_cost = cost_v + edge.get_weight();
                        Some(new_cost)
                    }

                    _ => None,
                };

                if let Some(new_cost) = new_cost {
                    costs.insert(w, new_cost);
                    predecessor.insert(w, v);
                    changed_vertices.push(w);
                }
            }

            // Nothing has improved in this iteration -> done
            if changed_vertices.is_empty() {
                break;
            }

            // If there is a change in the *n*th iteration, we have a negative cycle
            if i == n && !changed_vertices.is_empty() {
                // negative cycle
                let cycle = construct_negative_cycle(predecessor, changed_vertices[0]);
                return BellmanFordResult::NegativeCycle(cycle);
            }

            vertices = changed_vertices;
        }

        BellmanFordResult::SPT(SingleSourceShortestPaths::new(start, costs, predecessor))
    }
}

fn construct_negative_cycle<VId>(predecessors: FxHashMap<VId, VId>, initial: VId) -> Vec<VId>
where
    VId: Hash + Eq + Copy,
{
    let mut visited = FxHashSet::default();
    let mut current = initial;

    // Follow the predecessor chain until we find a cycle
    loop {
        if visited.contains(&current) {
            // We found the start of the cycle
            break;
        }
        visited.insert(current);

        if let Some(&pred) = predecessors.get(&current) {
            current = pred;
        } else {
            // This shouldn't happen if we have a negative cycle
            break;
        }
    }

    // Now current is a vertex in the cycle
    // Construct the cycle by following predecessors
    let cycle_start = current;
    let mut cycle = vec![cycle_start];

    if let Some(&pred) = predecessors.get(&current) {
        current = pred;
        while current != cycle_start {
            cycle.push(current);
            if let Some(&pred) = predecessors.get(&current) {
                current = pred;
            } else {
                break;
            }
        }
    }

    cycle.reverse(); // Reverse to get the cycle in forward direction
    cycle
}
