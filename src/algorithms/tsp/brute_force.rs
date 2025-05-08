use std::ops::Add;

use crate::{
    graph::{GraphBase, Path, WeightedEdge, WithID},
    Graph, GraphError,
};

use super::TspResult;

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Copy + PartialEq,
    Backend::Edge: WeightedEdge + Clone,
    <Backend::Edge as WeightedEdge>::WeightType:
        Add<Output = <Backend::Edge as WeightedEdge>::WeightType> + Copy,
{
    /// Finds a path with the optimal TSP solution using a simple brute force approach.
    ///
    /// # Requirements
    /// - `self` must be a fully connected graph with weights assigned to all edges.
    ///
    /// # Parameters
    /// - `start_vertex_id`: Optional ID of the vertex to start the TSP from. If `None`, a default starting vertex is chosen.
    ///
    /// # Returns
    /// - Returns a `TspResult<Backend>` containing the optimal path found, or an empty path if the graph is empty.
    ///
    /// # Panics
    /// - May panic if the graph is not fully connected.
    pub fn tsp_brute_force(
        &self,
        start_vertex_id: Option<<Backend::Vertex as WithID>::IDType>,
    ) -> TspResult<Backend> {
        let (start_v, remaining_vertices) = match self.get_initial_vertex(start_vertex_id) {
            Some(v) => v,
            None => return Ok(Path::default()),
        };

        let mut best_path = None;
        let mut initial_path = vec![start_v];
        let initial_cost = <Backend::Edge as WeightedEdge>::WeightType::default();
        let mut remaining = remaining_vertices.collect::<Vec<_>>();

        self.brute_force(
            start_v,
            &mut initial_path,
            initial_cost,
            &mut remaining,
            &mut best_path,
        );

        match best_path {
            Some((_, best_path)) => {
                // Construct the Path object
                let mut path = Path::default();

                for window in best_path.windows(2) {
                    let from_v = window[0];
                    let to_v = window[1];
                    let edge = self.get_edge(from_v, to_v).unwrap().clone();
                    path.push(from_v, to_v, edge);
                }
                Ok(path)
            }
            None => Err(GraphError::AlgorithmError(
                "Could not solve TSP, no optimal solution was found".to_string(),
            )),
        }
    }

    #[allow(clippy::type_complexity)]
    /// Recursive function to go through the different permutations
    fn brute_force(
        &self,
        current_v: <Backend::Vertex as WithID>::IDType,
        current_path: &mut Vec<<Backend::Vertex as WithID>::IDType>,
        current_cost: <Backend::Edge as WeightedEdge>::WeightType,
        remaining: &mut Vec<<Backend::Vertex as WithID>::IDType>,
        current_best: &mut Option<(
            <Backend::Edge as WeightedEdge>::WeightType,
            Vec<<Backend::Vertex as WithID>::IDType>,
        )>,
    ) {
        if current_path.len() == self.vertex_count() {
            // Alle Knoten besucht, Tour schließen
            let edge_cost = self
                .get_edge(current_v, current_path[0])
                .unwrap()
                .get_weight();
            let total_cost = current_cost + edge_cost;

            match current_best {
                Some((best_cost, best_path)) if &total_cost < best_cost => {
                    // Startknoten zum Ende der Tour hinzufügen
                    let mut path = current_path.to_owned();
                    path.push(current_path[0]);
                    *best_cost = total_cost;
                    *best_path = path;
                }
                None => {
                    // Startknoten zum Ende der Tour hinzufügen
                    let mut path = current_path.to_owned();
                    path.push(current_path[0]);
                    *current_best = Some((total_cost, path));
                }
                _ => {}
            }

            // Diese Permutation "abschließen"
            return;
        }

        // Für alle noch nicht besuchten Knoten
        // Wir iterieren durch alle Indizes des nicht besuchten Knoten
        let last_remaining_idx = remaining.len() - 1;
        for next_i in 0..=last_remaining_idx {
            // Wir untersuchen nun den Knoten an Position i
            // Dazu swappen wir ihn an die letzte Position des Vecs, damit wir ihn per `.pop()` entfernen können.
            // `swap()` und `pop()` sind beide O(1)
            remaining.swap(next_i, last_remaining_idx);
            let next = remaining.pop().unwrap();

            let edge_cost = self.get_edge(current_v, next).unwrap().get_weight();
            let new_cost = current_cost + edge_cost;

            // Rekursiv weiter erkunden
            current_path.push(next);
            self.brute_force(next, current_path, new_cost, remaining, current_best);

            // State vor rekursivem Aufruf wiederherstellen
            current_path.pop();
            remaining.push(next);
            remaining.swap(next_i, last_remaining_idx);
        }
    }
}
