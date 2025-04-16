use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::{
    graph::{WeightedEdge, WithID},
    Graph, GraphError,
};

use super::union_find::{UnionFind, UnionFindError};

impl<VId, Vertex, Edge> Graph<VId, Vertex, Edge>
where
    VId: Eq + Hash + Debug + Copy + Display + 'static,
    Vertex: WithID<VId> + Clone,
    Edge: WeightedEdge + Clone,
{
    /// Creates an MST using the Kruskal's algorithm.
    ///
    /// Returns the MST as a new graph
    pub fn mst_kruskal(&self) -> Result<Graph<VId, Vertex, Edge>, GraphError<VId>> {
        let is_directed = self.is_directed();
        let mut mst_graph = Graph::<VId, Vertex, Edge>::new(is_directed);

        // Get all edges and sort them
        let mut edges = self
            .get_all_edges()
            .into_iter()
            .map(|(v1, v2, e)| (v1, v2, e.get_weight(), e))
            .collect::<Vec<_>>();

        // Sort descending to pop lowest elements first
        edges.sort_by(|(_, _, weight1, _), (_, _, weight2, _)| {
            weight2
                .partial_cmp(weight1)
                .expect("Graph weights must not contain NaN values")
        });

        // Put all vertices in a Union-Find struct
        let mut union_find = UnionFind::new();
        for v in self.get_all_vertices().into_iter() {
            union_find.make_set(v.get_id())?;
            mst_graph.push_vertex(v.clone())?;
        }

        // Pop each edge in edges (lowest first):
        while let Some((from, to, weight, edge)) = edges.pop() {
            //  if adding e to MST would not create a circle:
            match union_find.union(from, to) {
                Ok(_) => {
                    // Add e to the MST
                    match is_directed {
                        true => mst_graph.push_edge(*from, *to, edge.to_owned())?,
                        false => mst_graph.push_undirected_edge(*from, *to, edge.to_owned())?,
                    }
                }
                Err(err) => match err {
                    UnionFindError::NotDisjunct(_, _, _) => {}
                    _ => return Err(GraphError::from(err)),
                },
            }
        }

        Ok(mst_graph)
    }
}
