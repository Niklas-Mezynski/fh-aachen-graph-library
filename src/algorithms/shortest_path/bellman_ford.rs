use rustc_hash::FxHashMap;
use std::hash::Hash;

use crate::{
    graph::{GraphBase, WeightedEdge, WithID},
    Directed, Graph,
};

use super::single_source_shortest_paths::SingleSourceShortestPaths;

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
    /// Returns a tuple with a `HashMap` that maps `VertexID` to path cost and
    /// a `HashMap` that maps `VertexID` to the predecessor `VertexID` that can be used to reconstruct the path.
    #[allow(clippy::type_complexity)]
    pub fn bellman_ford(
        &self,
        start: <Backend::Vertex as WithID>::IDType,
    ) -> Option<
        SingleSourceShortestPaths<
            <Backend::Vertex as WithID>::IDType,
            <Backend::Edge as WeightedEdge>::WeightType,
        >,
    > {
        // Final map of costs from start to each v
        let mut costs = FxHashMap::default();
        // Which vertex was visited before each other. Can be used to reconstruct the exact path
        let mut predecessor = FxHashMap::default();

        let edges = self.get_all_edges().collect::<Vec<_>>();

        costs.insert(
            start,
            <Backend::Edge as WeightedEdge>::WeightType::default(),
        );

        let n = self.vertex_count();
        // For |V| - 1 iterations, check all edges and see if we can decrease cost to any vertex
        for i in 1..=n {
            let mut changed = false;
            for (v, w, cost) in edges.iter() {
                // Check if the edge (v, w) can improve the current "best" cost to vertex w
                let cost_v = costs.get(v).copied();
                let cost_w = costs.get(w).copied();
                let new_cost = match (cost_v, cost_w) {
                    (Some(cost_v), Some(cost_w)) => {
                        let new_cost = cost_v + cost.get_weight();
                        if new_cost < cost_w {
                            Some(new_cost)
                        } else {
                            None
                        }
                    }

                    (Some(cost_v), None) => {
                        let new_cost = cost_v + cost.get_weight();
                        Some(new_cost)
                    }

                    _ => None,
                };

                if let Some(new_cost) = new_cost {
                    costs.insert(*w, new_cost);
                    predecessor.insert(*w, *v);
                    changed = true;
                }
            }

            if !changed {
                break;
            }

            if i == n && changed {
                // negative cycle
                return None;
            }
        }

        Some(SingleSourceShortestPaths::new(start, costs, predecessor))
    }
}
