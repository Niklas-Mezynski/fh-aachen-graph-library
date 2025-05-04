use std::hash::Hash;

use rustc_hash::FxHashSet;

use crate::{
    graph::{GraphBase, ListGraphBackend, Path, WeightedEdge, WithID},
    Graph,
};

use super::TspResult;

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Copy + Eq + Hash,
    Backend::Vertex: Clone,
    Backend::Edge: WeightedEdge + Clone,
    ListGraphBackend<Backend::Vertex, Backend::Edge, Backend::Direction>:
        GraphBase<Vertex = Backend::Vertex, Edge = Backend::Edge, Direction = Backend::Direction>,
{
    pub fn tsp_double_tree(&self) -> TspResult<Backend> {
        let mut path = Path::default();

        // Get random start vertex
        let mut vertices = self.get_all_vertices().map(|v| v.get_id());
        let start_v = match vertices.next() {
            Some(v) => v,
            None => return Ok(Path::default()),
        };

        // Generate MST
        let mst = self.mst_prim::<ListGraphBackend<_, _, _>>(Some(start_v))?;

        let mut prev_v = start_v;
        for current_v in mst.dfs_iter(start_v)?.skip(1).map(|v| v.get_id()) {
            path.edges.push((
                prev_v,
                current_v,
                self.get_edge(prev_v, current_v)
                    .expect("Edge must exist as TSP works on complete graphs")
                    .to_owned(),
            ));

            prev_v = current_v;
        }

        // Return to start_v
        path.edges.push((
            prev_v,
            start_v,
            self.get_edge(prev_v, start_v)
                .expect("Edge must exist as TSP works on complete graphs")
                .to_owned(),
        ));

        Ok(path)
    }
}
