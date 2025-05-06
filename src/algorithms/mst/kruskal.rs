use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::{
    graph::{GraphBase, WeightedEdge, WithID},
    Graph, GraphError,
};

use super::union_find::UnionFind;

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    Backend::Vertex: Clone,
    <Backend::Vertex as WithID>::IDType: Copy + Eq + Hash + Debug + Display + 'static,
    Backend::Edge: WeightedEdge + Clone,
{
    /// Creates an MST using the Kruskal's algorithm.
    ///
    /// Returns the MST as a new graph
    pub fn mst_kruskal<OutputBackend>(
        &self,
    ) -> Result<Graph<OutputBackend>, GraphError<<Backend::Vertex as WithID>::IDType>>
    where
        OutputBackend: GraphBase<
            Vertex = Backend::Vertex,
            Edge = Backend::Edge,
            Direction = Backend::Direction,
        >,
    {
        let mut mst_graph = Graph::<OutputBackend>::new();

        // Get all edges and sort them
        let mut edges = self
            .get_all_edges()
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
        for v in self.get_all_vertices() {
            union_find
                .make_set(v.get_id())
                .map_err(|e| GraphError::AlgorithmError(e.to_string()))?;
            mst_graph.push_vertex(v.clone())?;
        }

        let mut edge_count = 0;
        let target_edge_count = mst_graph.vertex_count() - 1;
        // Pop each edge in edges (lowest first):
        while let Some((from, to, _weight, edge)) = edges.pop() {
            //  if adding e to MST would not create a circle:
            let was_merged = union_find
                .union(&from, &to)
                .map_err(|e| GraphError::AlgorithmError(e.to_string()))?;

            if was_merged {
                mst_graph.push_edge(from, to, edge.to_owned())?;
                edge_count += 1;
            }

            // Early abort when n-1 edges have been visited
            if edge_count >= target_edge_count {
                break;
            }
        }

        Ok(mst_graph)
    }
}
