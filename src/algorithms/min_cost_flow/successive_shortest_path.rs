use std::{
    hash::Hash,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

use rustc_hash::FxHashMap;

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
    is_helper: bool,
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
    #[allow(clippy::too_many_arguments)]
    pub fn successive_shortest_path<
        CFB,
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
        cost_edge_builder: EdgeBuilderFn,
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
        FlowFn: Fn(&Backend::Edge) -> &CFB,
        FlowMutFn: Fn(&mut Backend::Edge) -> &mut CFB,
        MaxFlowFn: Fn(&Backend::Edge) -> &CFB,
        CostFn: Fn(&Backend::Edge) -> &CFB,
        EdgeBuilderFn: Fn(CFB) -> Backend::Edge,
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
                            is_helper: false,
                        },
                    ),
                    (
                        to,
                        from,
                        ResidualEdge {
                            remaining_capacity: edge.flow,
                            cost: -(edge.cost),
                            is_residual: true,
                            is_helper: false,
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

        Ok(())
    }
}
