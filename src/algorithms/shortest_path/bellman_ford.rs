use rustc_hash::FxHashMap;
use std::hash::Hash;

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
    ) -> Option<(
        FxHashMap<<Backend::Vertex as WithID>::IDType, <Backend::Edge as WeightedEdge>::WeightType>,
        FxHashMap<<Backend::Vertex as WithID>::IDType, <Backend::Vertex as WithID>::IDType>,
    )> {
        let mut costs = FxHashMap::default();
        let mut predecessor = FxHashMap::default();
        let edges = self.get_all_edges().collect::<Vec<_>>();

        costs.insert(
            start,
            <Backend::Edge as WeightedEdge>::WeightType::default(),
        );

        let n = self.vertex_count();
        for i in 0..n {
            let mut changed = false;
            for (v, w, cost) in edges.iter() {
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

            // if !changed {
            //     break;
            // }

            if i == n - 1 && changed {
                // negative cycle
                return None;
            }
        }

        Some((costs, predecessor))
    }
}
