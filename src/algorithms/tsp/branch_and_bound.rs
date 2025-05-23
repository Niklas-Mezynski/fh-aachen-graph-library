use std::{fmt::Debug, hash::Hash, ops::Add};

use crate::{
    graph::{GraphBase, ListGraphBackend, Path, WeightedEdge, WithID},
    Graph,
};

use super::TspResult;

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Copy + Eq + Hash + Debug,
    Backend::Vertex: Clone,
    Backend::Edge: WeightedEdge + Clone,
    <Backend::Edge as WeightedEdge>::WeightType:
        Add<Output = <Backend::Edge as WeightedEdge>::WeightType> + Copy,
    ListGraphBackend<Backend::Vertex, Backend::Edge, Backend::Direction>:
        GraphBase<Vertex = Backend::Vertex, Edge = Backend::Edge, Direction = Backend::Direction>,
{
    /// Finds a path with the optimal TSP solution using a branch and bound brute force approach.
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
    pub fn tsp_branch_and_bound(
        &self,
        start_vertex_id: Option<<Backend::Vertex as WithID>::IDType>,
    ) -> TspResult<Backend> {
        let (start_v, remaining_vertices) = match self.get_initial_vertex(start_vertex_id) {
            Some(v) => v,
            None => return Ok(Path::default()),
        };

        // Einen ersten "besten" Pfad mit dem Nearest Neighbor Algorithmus berechnen
        let best_path = self.tsp_double_tree(Some(start_v))?;

        let mut current_best_path = best_path.vertices().cloned().collect::<Vec<_>>();
        let mut current_best_cost = best_path.total_cost();

        let mut initial_path = vec![start_v];
        let initial_cost = <Backend::Edge as WeightedEdge>::WeightType::default();
        let mut remaining = remaining_vertices.collect::<Vec<_>>();

        self.branch_and_bound(
            start_v,
            &mut initial_path,
            initial_cost,
            &mut remaining,
            (&mut current_best_cost, &mut current_best_path),
        );

        // Construct the Path object
        let mut path = Path::default();

        for window in current_best_path.windows(2) {
            let from_v = window[0];
            let to_v = window[1];
            let edge = self.get_edge(from_v, to_v).unwrap().clone();
            path.push(from_v, to_v, edge);
        }
        Ok(path)
    }

    /// Recursive function to go through the different permutations
    fn branch_and_bound(
        &self,
        current_v: <Backend::Vertex as WithID>::IDType,
        current_path: &mut Vec<<Backend::Vertex as WithID>::IDType>,
        current_cost: <Backend::Edge as WeightedEdge>::WeightType,
        remaining: &mut Vec<<Backend::Vertex as WithID>::IDType>,
        (current_best_cost, current_best_path): (
            &mut <Backend::Edge as WeightedEdge>::WeightType,
            &mut Vec<<Backend::Vertex as WithID>::IDType>,
        ),
    ) {
        if current_path.len() == self.vertex_count() {
            // Alle Knoten besucht, Tour schließen
            let edge_cost = self
                .get_edge(current_v, current_path[0])
                .unwrap()
                .get_weight();
            let total_cost = current_cost + edge_cost;

            // Prüfen ob diese neue Tour besser ist als das aktuelle Optimum
            if &total_cost < current_best_cost {
                // Startknoten zum Ende der Tour hinzufügen
                let mut path = current_path.to_owned();
                path.push(current_path[0]);
                *current_best_cost = total_cost;
                *current_best_path = path;
            }

            // Diese Permutation "abschließen"
            return;
        }

        // Für alle noch nicht besuchten Knoten
        // Wir iterieren durch alle Indizes des nicht besuchten Knoten
        let last_remaining_idx = remaining.len() - 1;
        for next_i in 0..=last_remaining_idx {
            let next = remaining[next_i];
            let edge_cost = self.get_edge(current_v, next).unwrap().get_weight();
            let new_cost = current_cost + edge_cost;

            // Prüfen ob es sich noch lohnt, diese Tour weiter zu erkunden
            // Verbesserte Prüfung:
            // Für alle Knoten in remaining, schaue ich mir die billigste und 2. billigste Kante zu Knoten an, an denen ich noch nicht war
            // Das Ergebnis * 0.5 und dann die Kantenkosten aufsummieren.
            // Wenn das + die aktuellen Kosten dann schon größer sind als current_best_cost, können wir abbrechen

            // 1. Für alle Knoten in remaining die billigste und 2. billigste Kante finden
            // Außerdem prüfen, dass noch mindestens 2 (+ der aktuelle = 3) Knoten zu verarbeiten sind
            let min_remaining_cost: <Backend::Edge as WeightedEdge>::WeightType =
                if remaining.len() > 2 {
                    remaining
                        .iter()
                        .filter(|v| v != &&next)
                        .map(|remaining_to| {
                            let weights = remaining
                                .iter()
                                .filter(move |remaining_from| remaining_to != *remaining_from)
                                .map(|remaining_from| {
                                    self.get_edge(*remaining_from, *remaining_to)
                                        .unwrap()
                                        .get_weight()
                                });

                            // Find the two cheapest
                            let mut cheapest: Option<(
                                usize,
                                <Backend::Edge as WeightedEdge>::WeightType,
                            )> = None;
                            let mut second_cheapest: Option<(
                                usize,
                                <Backend::Edge as WeightedEdge>::WeightType,
                            )> = None;
                            for (i, weight) in weights.enumerate() {
                                // Init cheapest
                                if cheapest.is_none() {
                                    cheapest = Some((i, weight));
                                    continue;
                                }

                                // Init second cheapest
                                if second_cheapest.is_none() && cheapest.is_some() {
                                    second_cheapest = Some((i, weight));
                                }

                                if weight < cheapest.unwrap().1 {
                                    second_cheapest = cheapest;
                                    cheapest = Some((i, weight));
                                }
                            }

                            // 2. Diese beiden Kantenkosten aufsummieren und das Ergebnis * 0.5
                            (cheapest.unwrap().1 + second_cheapest.unwrap().1) / 2u8.into()
                        })
                        .sum()
                } else {
                    // Wenn wir nicht mehr genügend Restkanten zum prüfen haben, dann rechnen wir "einfach" mit den aktuellen Pfadkosten
                    <Backend::Edge as WeightedEdge>::WeightType::default()
                };

            // 3. Wenn diese minimalen Restkosten + die aktuellen Kosten >=
            if &(new_cost + min_remaining_cost) >= current_best_cost {
                // Wenn bereits teurer -> Abbruch
                continue;
            }

            // Wir untersuchen nun den Knoten an Position i
            // Dazu swappen wir ihn an die letzte Position des Vecs, damit wir ihn per `.pop()` entfernen können.
            // `swap()` und `pop()` sind beide O(1)
            remaining.swap(next_i, last_remaining_idx);
            let next = remaining.pop().unwrap();

            // Rekursiv weiter erkunden
            current_path.push(next);
            self.branch_and_bound(
                next,
                current_path,
                new_cost,
                remaining,
                (current_best_cost, current_best_path),
            );

            // State vor rekursivem Aufruf wiederherstellen
            current_path.pop();
            remaining.push(next);
            remaining.swap(next_i, last_remaining_idx);
        }
    }
}
