use std::{fmt::Debug, hash::Hash};

use crate::{graph::WithID, Graph, GraphError};

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Debug + Copy,
    Vertex: WithID<VId>,
    Edge: Clone,
{
    /// Creates an MST using the Prim algorithm.
    ///
    /// Returns the MST as a new graph
    pub fn mst_prim(&self) -> Result<Graph<VId, Vertex, Edge>, GraphError<VId>> {
        todo!()
    }
}
