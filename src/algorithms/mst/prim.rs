use std::{fmt::Debug, hash::Hash};

use rustc_hash::FxHashSet;

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
    /// Creates an MST using the Prim algorithm.
    ///
    /// Returns the MST as a new graph
    pub fn mst_prim(&self) -> Result<Graph<VId, Vertex, Edge>, GraphError<VId>> {
        let mut mst_graph = Graph::<VId, Vertex, Edge>::new(self.is_directed());

        // Schritt 1: Wähle einen Knoten 𝑣0 ∊ 𝑉
        let mut vertices_iter = self.get_all_vertices().into_iter();
        let v0 = match vertices_iter.next() {
            Some(v) => v,
            // If the graph has no vertices -> abort
            None => return Ok(mst_graph),
        };

        mst_graph.push_vertex(v0.clone())?;

        let mut remaining_vertices = vertices_iter.map(|v| v.get_id()).collect::<FxHashSet<_>>();

        let mut edges = self
            .get_adjacent_vertices_with_edges(&v0.get_id())?
            .into_iter()
            .map(|(to, edge)| (v0, to, edge))
            .collect::<Vec<_>>();

        // Schritt 2: Solange 𝑇 noch nicht alle Knoten aus 𝐺 enthält, wiederhole die folgende Prozedur
        // Wenn 𝑇 alle Knoten aus 𝐺 enthält, hat 𝑇 eine Kantenmenge von |V| -1 (Terminiert
        // Algorithmus).
        while !remaining_vertices.is_empty() {
            //   Schritt (a): Wählen Sie die billigste Kante aus, die von einem schon besuchten Knote
            //     zu einem noch nicht besuchten Knoten geht.
            edges.retain(|(_, v_to, _)| remaining_vertices.contains(&v_to.get_id()));
            let (cheapest_v_from, cheapest_v_to, cheapest_edge) = *edges
                .iter()
                .min_by(|(_, _, e1), (_, _, e2)| {
                    e1.get_weight()
                        .partial_cmp(&e2.get_weight())
                        .expect("TODO:")
                })
                .expect("TODO:");

            let cheapest_v_to_id = cheapest_v_to.get_id();
            // Aus dem remaining vertices set löschen
            remaining_vertices.remove(&cheapest_v_to_id);

            // Schritt (b): Fügen Sie die Kante und den nun erreichbaren Knoten in den Baum T ein
            // Nach und nach entsteht dann ein MST (der Baum wächst).
            mst_graph.push_vertex(cheapest_v_to.clone())?;
            mst_graph.push_undirected_edge(
                cheapest_v_from.get_id(),
                cheapest_v_to_id,
                cheapest_edge.clone(),
            )?;

            // Die neuen (nun erreichbaren Kanten) hinzufügen
            edges.append(
                &mut self
                    .get_adjacent_vertices_with_edges(&cheapest_v_to_id)?
                    .into_iter()
                    .map(|(to, edge)| (cheapest_v_to, to, edge))
                    .collect(),
            );
        }

        Ok(mst_graph)
    }
}
