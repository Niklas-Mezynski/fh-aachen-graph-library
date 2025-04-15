use std::{fmt::Debug, hash::Hash};

use crate::{
    graph::{WeightedEdge, WithID},
    Graph, GraphError,
};

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Debug + Copy,
    Vertex: WithID<VId> + Clone,
    Edge: WeightedEdge + Clone,
{
    /// Creates an MST using the Kruskal's algorithm.
    ///
    /// Returns the MST as a new graph
    pub fn mst_kruskal(&self) -> Result<Graph<VId, Vertex, Edge>, GraphError<VId>> {
        let mut mst_graph = Graph::<VId, Vertex, Edge>::new(self.is_directed());

        // Get all edges and sort them ascending
        // Pop each edge in edges (lowest first):
        //  if adding e to MST would not create a circle:
        //      Add e to the MST

        Ok(mst_graph)
    }
}
