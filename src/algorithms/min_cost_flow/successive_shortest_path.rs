use std::{
    hash::Hash,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

use rustc_hash::FxHashSet;

use crate::{
    algorithms::shortest_path::bellman_ford::BellmanFordResult,
    graph::{GraphBase, WeightedEdge, WithID},
    Directed, Graph, GraphError, ListGraph,
};

#[derive(Debug, Clone)]
struct BalanceVertex<VId, Balance> {
    id: VId,
    balance: Balance,
    balance_curr: Balance,
}

impl<VId: Copy, Balance> WithID for BalanceVertex<VId, Balance> {
    type IDType = VId;

    fn get_id(&self) -> VId {
        self.id
    }
}

#[derive(Debug, Clone)]
struct CostFlowEdge<CostFlow> {
    cost: CostFlow,
    max_flow: CostFlow,
    flow: CostFlow,
}

#[derive(Debug, Clone)]
pub struct ResidualEdge<CostFlow> {
    remaining_capacity: CostFlow,
    cost: CostFlow,
    is_residual: bool,
}

impl<CostFlow> WeightedEdge for ResidualEdge<CostFlow>
where
    CostFlow: Copy
        + Sum
        + Div<Output = CostFlow>
        + From<u8>
        + PartialOrd
        + AddAssign<CostFlow>
        + Add<Output = CostFlow>
        + Default,
{
    type WeightType = CostFlow;

    fn get_weight(&self) -> Self::WeightType {
        self.cost
    }
}

impl<Backend> Graph<Backend>
where
    Backend: GraphBase<Direction = Directed> + Clone,
    Backend::Vertex: Clone,
    <Backend::Vertex as WithID>::IDType: Eq + Hash + PartialOrd + Copy,
    Backend::Edge: Clone,
{
    /// TODO
    pub fn successive_shortest_path<CFB, FlowMutFn, MaxFlowFn, CostFn, BalanceFn>(
        &mut self,
        balance: BalanceFn,
        flow_mut: FlowMutFn,
        max_flow: MaxFlowFn,
        cost: CostFn,
    ) -> Result<(), GraphError<<Backend::Vertex as WithID>::IDType>>
    where
        // Wtf, can I do this better?
        CFB: Default
            + Copy
            + PartialEq
            + PartialOrd
            + Sub<Output = CFB>
            + Add<Output = CFB>
            + Mul<CFB, Output = CFB>
            + Sum
            + Neg<Output = CFB>
            + Div<Output = CFB>
            + From<u8>
            + AddAssign<CFB>
            + Neg<Output = CFB>,
        BalanceFn: Fn(&Backend::Vertex) -> &CFB,
        FlowMutFn: Fn(&mut Backend::Edge) -> &mut CFB,
        MaxFlowFn: Fn(&Backend::Edge) -> &CFB,
        CostFn: Fn(&Backend::Edge) -> &CFB,
    {
        // TODO: Is this needed here? -> Should at least terminate the algorithm early, so we don't waste compute.
        // Check that the sum of all balances == 0
        let balance_sum: CFB = self.get_all_vertices().map(|v| *balance(v)).sum();
        if balance_sum != CFB::default() {
            return Err(GraphError::AlgorithmError(
                "Balance requirements cannot be fulfilled. Sum is not equal to zero.".to_string(),
            ));
        }

        // Initialize a new in between graph with minimal cost
        let mut graph = ListGraph::<
            BalanceVertex<<Backend::Vertex as WithID>::IDType, CFB>,
            CostFlowEdge<CFB>,
            Backend::Direction,
        >::from_vertices_and_edges(
            self.get_all_vertices()
                .map(|v| BalanceVertex {
                    id: v.get_id(),
                    balance: *balance(v),
                    balance_curr: CFB::default(),
                })
                .collect(),
            self.get_all_edges()
                .map(|(from, to, edge)| {
                    (
                        from,
                        to,
                        CostFlowEdge {
                            cost: *cost(edge),
                            flow: if cost(edge) >= &CFB::default() {
                                CFB::default()
                            } else {
                                *max_flow(edge)
                            },
                            max_flow: *max_flow(edge),
                        },
                    )
                })
                .collect(),
        )?;

        // Set all current balances
        // For each v: balance_curr = sum(outgoing flow) - sum(incoming flow)
        let balance_vec: Vec<_> = graph
            .get_all_vertices()
            .map(|v| {
                let outgoing_flow: CFB = graph
                    .get_adjacent_vertices_with_edges(v.get_id())
                    .map(|(_, edge)| edge.flow)
                    .sum();

                let incoming_flow: CFB = graph
                    .get_all_edges()
                    .filter(|(_, to, _)| to == &v.get_id())
                    .map(|(_, _, edge)| edge.flow)
                    .sum();

                (v.get_id(), outgoing_flow - incoming_flow)
            })
            .collect();

        for (id, balance) in balance_vec {
            graph.get_vertex_by_id_mut(id).unwrap().balance_curr = balance;
        }

        // Create the residual graph
        // Residual graph
        let res_edges: Vec<_> = graph
            .get_all_edges()
            .flat_map(|(from, to, edge)| {
                [
                    (
                        from,
                        to,
                        ResidualEdge {
                            remaining_capacity: edge.max_flow - edge.flow,
                            cost: edge.cost,
                            is_residual: false,
                        },
                    ),
                    (
                        to,
                        from,
                        ResidualEdge {
                            remaining_capacity: edge.flow,
                            cost: -(edge.cost),
                            is_residual: true,
                        },
                    ),
                ]
            })
            .collect();

        let mut residual_graph = ListGraph::<_, _, Backend::Direction>::from_vertices_and_edges(
            // Take vertices from original graph (without super source and target)
            graph.get_all_vertices().cloned().collect(),
            res_edges,
        )?;

        // Main Loop
        loop {
            // Find pseudo sources and targets
            let mut sources = vec![];
            let mut targets = vec![];
            for v in residual_graph.get_all_vertices() {
                // If the current balance is smaller than the expected balance, we have to "push flow out".
                // Therefore we treat it as a pseudo-source
                if v.balance_curr < v.balance {
                    sources.push(v.get_id());
                } else if v.balance_curr > v.balance {
                    // Vice versa
                    targets.push(v.get_id());
                }
            }

            // All balances are satisfied -> done
            if sources.is_empty() && targets.is_empty() {
                break;
            }

            // If only one set is empty -> balance satisfaction not possible
            if sources.is_empty() || targets.is_empty() {
                return Err(GraphError::AlgorithmError(
                    "Balance requirements cannot be satisfied. One of pseudo sources or targets is empty.".to_string(),
                ));
            }

            // Find a reachable pair (s, t)
            let target_set: FxHashSet<_> = targets.iter().cloned().collect();
            let reachable_pair = sources.iter().find_map(|&source| {
                residual_graph
                    .bfs_iter_with_filter(source, |e| e.remaining_capacity > CFB::default())
                    .ok()?
                    .find(|vertex| target_set.contains(&vertex.id))
                    .map(|target| (source, target.id))
            });

            let (source, target) = match reachable_pair {
                Some(pair) => pair,
                None => {
                    return Err(GraphError::AlgorithmError(
                        "No reachable source-target pair found".to_string(),
                    ));
                }
            };
            let source_v = residual_graph
                .get_vertex_by_id(source)
                .expect("Source must exist");
            let target_v = residual_graph
                .get_vertex_by_id(target)
                .expect("Target must exist");

            // Run bellman ford to find the shortest path from s to t
            let shortest_paths =
                match residual_graph.bellman_ford_with_edge_filter(source, |(_, _, edge)| {
                    // Only run bellman ford on edges with residual capacity != 0
                    edge.remaining_capacity > CFB::default()
                }) {
                    BellmanFordResult::SPT(paths) => paths,
                    BellmanFordResult::NegativeCycle(_) => {
                        return Err(GraphError::AlgorithmError(
                            "Bellman Ford detected negative cycles".to_string(),
                        ))
                    }
                };

            let shortest_path = shortest_paths.get_path(target);

            // Find the value to use for updating the flow along the path
            // Either the smallest capacity along the way OR the exact value to satisfy either source or target
            let gamma = shortest_path
                .windows(2)
                .map(|window| {
                    residual_graph
                        .get_edge(window[0], window[1])
                        .expect("Edge must exist")
                        .remaining_capacity
                })
                .chain([
                    source_v.balance - source_v.balance_curr,
                    target_v.balance_curr - target_v.balance,
                ])
                .min_by(|this, other| {
                    this.partial_cmp(other)
                        .expect("Graph capacities must be comparable")
                })
                .expect("A min gamma value must exist along the path");

            // Update the flow AND the current balances
            shortest_path.windows(2).for_each(|window| {
                let from = window[0];
                let to = window[1];

                // Update the forward edge
                let forward_edge = residual_graph
                    .get_edge_mut(from, to)
                    .expect("Edge must exist");
                // We subtract here, because now there is less flow to push in this direction
                forward_edge.remaining_capacity = forward_edge.remaining_capacity - gamma;

                // Update the corresponding backward edge
                let backward_edge = residual_graph
                    .get_edge_mut(to, from)
                    .expect("Backward edge must exist");
                // We add here, because now there is more flow to push in this direction
                backward_edge.remaining_capacity += gamma;

                // Update the balances
                let from_v = residual_graph
                    .get_vertex_by_id_mut(from)
                    .expect("\"from\" vertex must exist");
                from_v.balance_curr += gamma;

                let to_v = residual_graph
                    .get_vertex_by_id_mut(to)
                    .expect("\"to\" vertex must exist");
                to_v.balance_curr = to_v.balance_curr - gamma;
            })
        }

        // Apply flows found in residual graph to the main graph
        for (from, to, edge) in residual_graph
            .get_all_edges()
            .filter(|(_from, _to, edge)| !edge.is_residual)
        {
            let edge_to_modify = self
                .get_edge_mut(from, to)
                .expect("Edge must also exist in original graph");

            // As the residual graph contains the remaining potential,
            // we subtract from the max flow
            *flow_mut(edge_to_modify) = *max_flow(edge_to_modify) - edge.remaining_capacity;
        }

        Ok(())
    }
}
