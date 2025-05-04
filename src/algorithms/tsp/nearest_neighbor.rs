use std::hash::Hash;

use rustc_hash::FxHashSet;

use crate::{
    graph::{GraphBase, Path, WeightedEdge, WithID},
    Graph,
};

use super::TspResult;

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Copy + Eq + Hash,
    Backend::Edge: WeightedEdge + Clone,
{
    pub fn tsp_nearest_neighbor(
        &self,
        start_vertex_id: Option<<Backend::Vertex as WithID>::IDType>,
    ) -> TspResult<Backend> {
        let n = self.vertex_count();
        let mut visited = FxHashSet::default();
        let mut path = Path::default();

        // Get random start vertex
        let (start_v, _) = match self.get_initial_vertex(start_vertex_id) {
            Some(v) => v,
            None => return Ok(Path::default()),
        };

        // Mark start vertex
        visited.insert(start_v);

        // While not all vertices have been visited...
        let mut current_v = start_v;
        let mut i = 1;
        while i < n {
            // Find next cheapest vertex (that ahs not been visited yet)
            let (next_v, edge) = self
                .get_adjacent_vertices_with_edges(current_v)
                .filter(|(to, _edge)| !visited.contains(&to.get_id()))
                .min_by(|(_to, edge), (_to_other, edge_other)| {
                    edge.get_weight()
                        .partial_cmp(&edge_other.get_weight())
                        .expect("Graph weights must not contain NaN values")
                })
                .expect("There must be another vertex to visit. TSP expects a complete graph");

            let next_v = next_v.get_id();

            // Add to path, mark as visited
            path.edges.push((current_v, next_v, edge.to_owned()));
            visited.insert(next_v);
            current_v = next_v;

            i += 1;
        }

        // Complete the cycle (back to start)
        path.edges.push((
            current_v,
            start_v,
            self.get_edge(current_v, start_v)
                .expect(
                    "There must be and edge back to the start vertex. TSP expects a complete graph",
                )
                .to_owned(),
        ));

        Ok(path)
    }
}
