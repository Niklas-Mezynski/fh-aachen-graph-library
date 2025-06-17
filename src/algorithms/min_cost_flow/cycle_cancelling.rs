use std::{
    hash::Hash,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

use crate::{
    algorithms::shortest_path::bellman_ford::BellmanFordResult,
    graph::{GraphBase, WeightedEdge, WithID},
    Directed, Graph, GraphError, ListGraph,
};

#[derive(Debug, Clone)]
pub struct ResidualEdge<Flow, Cost> {
    remaining_capacity: Flow,
    cost: Cost,
    is_residual: bool,
    is_helper: bool,
}

impl<Flow, Cost> WeightedEdge for ResidualEdge<Flow, Cost>
where
    Cost: Copy
        + Sum
        + Div<Output = Cost>
        + From<u8>
        + PartialOrd
        + AddAssign<Cost>
        + Add<Output = Cost>
        + Default,
{
    type WeightType = Cost;

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
    /// TODO...
    pub fn cycle_cancelling<
        Flow,
        Cost,
        Balance,
        FlowFn,
        FlowMutFn,
        MaxFlowFn,
        CostFn,
        BalanceFn,
        EdgeBuilderFn,
    >(
        &mut self,
        balance: BalanceFn,
        flow: FlowFn,
        flow_mut: FlowMutFn,
        max_flow: MaxFlowFn,
        cost: CostFn,
        [super_source, super_target]: [Backend::Vertex; 2],
        cost_edge_builder: EdgeBuilderFn,
    ) -> Result<(), GraphError<<Backend::Vertex as WithID>::IDType>>
    where
        // Wtf, can I do this better?
        Flow: Default + Copy + PartialEq + PartialOrd + Sub<Output = Flow> + Add<Output = Flow>,
        Cost: Default
            + Copy
            + PartialEq
            + PartialOrd
            + Sub<Output = Cost>
            + Add<Output = Cost>
            + Mul<Flow, Output = Cost>
            + Sum
            + Neg<Output = Cost>
            + Div<Output = Cost>
            + From<u8>
            + AddAssign<Cost>,
        Balance: Default + Copy + PartialEq + PartialOrd + Sum + Neg<Output = Balance>,
        BalanceFn: Fn(&Backend::Vertex) -> &Balance,
        FlowFn: Fn(&Backend::Edge) -> &Flow,
        FlowMutFn: Fn(&mut Backend::Edge) -> &mut Flow,
        MaxFlowFn: Fn(&Backend::Edge) -> &Flow,
        CostFn: Fn(&Backend::Edge) -> &Cost,
        EdgeBuilderFn: Fn(Balance) -> Backend::Edge,
    {
        // Check that the sum of all balances == 0
        let balance_sum: Balance = self.get_all_vertices().map(|v| *balance(v)).sum();
        if balance_sum != Balance::default() {
            return Err(GraphError::AlgorithmError(
                "Balance requirements cannot be fulfilled. Sum is not equal to zero.".to_string(),
            ));
        }

        // Add super source and target so that we can run a max flow algorithm to try to fulfil the balances
        let mut super_graph = self.clone();

        let sources = super_graph
            .get_all_vertices()
            .filter(|v| balance(v) > &Balance::default())
            .map(|v| (v.get_id(), *balance(v)))
            .collect::<Vec<_>>();

        let targets = super_graph
            .get_all_vertices()
            .filter(|v| balance(v) < &Balance::default())
            .map(|v| (v.get_id(), *balance(v)))
            .collect::<Vec<_>>();

        let super_source_id = super_source.get_id();
        let super_target_id = super_target.get_id();
        super_graph.push_vertex(super_source.clone())?;
        super_graph.push_vertex(super_target)?;

        sources
            .into_iter()
            .try_for_each(|(source_id, source_balance)| {
                super_graph.push_edge(
                    super_source_id,
                    source_id,
                    cost_edge_builder(source_balance),
                )
            })?;

        targets
            .into_iter()
            .try_for_each(|(target_id, target_balance)| {
                super_graph.push_edge(
                    target_id,
                    super_target_id,
                    cost_edge_builder(-target_balance),
                )
            })?;

        // Run Edmonds-Karp-Algorithm algorithm
        super_graph
            .edmonds_karp(super_source_id, super_target_id, &flow_mut, &max_flow)
            .map_err(|_e| {
                GraphError::AlgorithmError("Error running Edmonds-Karp-Algorithm".to_string())
            })?;

        // Test that all balance requirements are fulfilled
        // -> Validate that super source's outgoing capacities are fully utilized
        super_graph
            .get_adjacent_vertices_with_edges(super_source_id)
            .try_for_each(|(_to, edge)| {
                if flow(edge) != max_flow(edge) {
                    return Err(GraphError::AlgorithmError("".to_string()));
                }
                Ok(())
            })?;

        // Residual graph
        let res_edges: Vec<_> = super_graph
            .get_all_edges()
            .filter(|(from, to, _edge)| from != &super_source_id && to != &super_target_id)
            // All edges in the "main direction" have their max flow potential in the beginning
            .flat_map(|(from, to, edge)| {
                [
                    (
                        from,
                        to,
                        ResidualEdge {
                            remaining_capacity: *max_flow(edge) - *flow(edge),
                            cost: *cost(edge),
                            is_residual: false,
                            is_helper: false,
                        },
                    ),
                    (
                        to,
                        from,
                        ResidualEdge {
                            remaining_capacity: *flow(edge),
                            cost: -(*cost(edge)),
                            is_residual: true,
                            is_helper: false,
                        },
                    ),
                ]
            })
            .collect();

        let mut residual_graph = ListGraph::<_, _, Backend::Direction>::from_vertices_and_edges(
            // Take vertices from original graph (without super source and target)
            self.get_all_vertices().cloned().collect(),
            res_edges,
        )?;

        // Add a new "super source" which connects to all other vertices. This way we can detect negative cycles anywhere in the graph
        let all_vertices = residual_graph
            .get_all_vertices()
            .map(|v| v.get_id())
            .collect::<Vec<_>>();
        residual_graph.push_vertex(super_source)?;
        all_vertices.into_iter().try_for_each(|v| {
            residual_graph.push_edge(
                super_source_id,
                v,
                ResidualEdge {
                    cost: Cost::default(),
                    remaining_capacity: Flow::default(),
                    is_residual: true,
                    is_helper: true,
                },
            )
        })?;

        // --- Now we try to optimize the cost ---
        // We execute the Moore-Bellman-Ford-Algorithm in order to check for a negative cycle in this graph
        while let BellmanFordResult::NegativeCycle(negative_cycle) = residual_graph
            .bellman_ford_with_edge_filter(super_source_id, |(_, _, edge)| {
                // Only run bellman ford on edges with residual capacity != 0
                edge.remaining_capacity > Flow::default() || edge.is_helper
            })
        {
            // Find the smallest residual capacity among the cycle
            let min = negative_cycle
                .windows(2)
                .map(|window| {
                    residual_graph
                        .get_edge(window[0], window[1])
                        .expect("Edge must exist")
                        .remaining_capacity
                })
                .min_by(|this, other| {
                    this.partial_cmp(other)
                        .expect("Graph capacities must not contain NaN values")
                })
                .expect("Negative cycle exist");

            // Update all flows by the current value
            negative_cycle.windows(2).for_each(|window| {
                let from = window[0];
                let to = window[1];

                // Update the forward edge
                let forward_edge = residual_graph
                    .get_edge_mut(from, to)
                    .expect("Edge must exist");
                // We subtract here, because now there is less flow to push in this direction
                forward_edge.remaining_capacity = forward_edge.remaining_capacity - min;

                // Update the corresponding backward edge
                let backward_edge = residual_graph
                    .get_edge_mut(to, from)
                    .expect("Backward edge must exist");
                // We add here, because now there is more flow to push in this direction
                backward_edge.remaining_capacity = backward_edge.remaining_capacity + min;
            })
        }

        // Apply flows found in residual graph to the main graph
        for (from, to, edge) in residual_graph
            .get_all_edges()
            .filter(|(_from, _to, edge)| !edge.is_residual && !edge.is_helper)
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
