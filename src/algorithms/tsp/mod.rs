use crate::{
    graph::{GraphBase, Path, WithID},
    GraphError,
};

pub mod brute_force;
pub mod nearest_neighbor;

pub type TspResult<Backend> = Result<
    Path<<<Backend as GraphBase>::Vertex as WithID>::IDType, <Backend as GraphBase>::Edge>,
    GraphError<<<Backend as GraphBase>::Vertex as WithID>::IDType>,
>;
