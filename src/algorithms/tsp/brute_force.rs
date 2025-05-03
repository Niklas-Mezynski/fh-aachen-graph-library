use std::hash::Hash;

use crate::{
    graph::{GraphBase, Path, WeightedEdge, WithID},
    Graph, GraphError,
};

type TspBruteForceResult<Backend> = Result<
    Path<<<Backend as GraphBase>::Vertex as WithID>::IDType, <Backend as GraphBase>::Edge>,
    GraphError<<<Backend as GraphBase>::Vertex as WithID>::IDType>,
>;

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Copy + Eq + Hash,
    Backend::Edge: WeightedEdge + Clone,
{
    pub fn tsp_brute_force(&self) -> TspBruteForceResult<Backend> {
        todo!()
    }
}
