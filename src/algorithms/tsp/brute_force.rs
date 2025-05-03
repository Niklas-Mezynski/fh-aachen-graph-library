use std::hash::Hash;

use crate::{
    graph::{GraphBase, Path, WeightedEdge, WithID},
    Graph, GraphError,
};

use super::TspResult;

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: Copy,
    Backend::Edge: WeightedEdge + Clone,
{
    pub fn tsp_brute_force(&self) -> TspResult<Backend> {
        let mut vertices = self.get_all_vertices().map(|v| v.get_id());

        let start_v = match vertices.next() {
            Some(v) => v,
            None => return Ok(Path::default()),
        };

        let mut vertices = vertices.collect::<Vec<_>>();

        let mut cheapest_cost = None;
        let mut cheapest_path = None;
        heap_permutations(&mut vertices, |permutation| {
            let mut path_vertices = vec![start_v];
            let mut total_cost = <Backend::Edge as WeightedEdge>::WeightType::default();
            let mut current_v = start_v;
            let mut valid = true;

            for &next_v in permutation {
                match self.get_edge(current_v, next_v) {
                    Some(edge) => {
                        total_cost += edge.get_weight();
                        path_vertices.push(next_v);
                        current_v = next_v;
                    }
                    None => {
                        valid = false;
                        break;
                    }
                }
            }

            // Return to start
            if valid {
                if let Some(edge) = self.get_edge(current_v, start_v) {
                    total_cost += edge.get_weight();

                    if cheapest_cost.is_none() || &total_cost < cheapest_cost.as_ref().unwrap() {
                        cheapest_cost = Some(total_cost);

                        path_vertices.push(start_v);
                        cheapest_path = Some(path_vertices);
                    }
                }
            }
        });

        Ok(cheapest_path
            .map(|path_vertices| {
                let mut current_v = *path_vertices.first().unwrap();
                let mut path = Path::default();
                for next_v in path_vertices.into_iter().skip(1) {
                    path.edges.push((
                        current_v,
                        next_v,
                        self.get_edge(current_v, next_v).unwrap().clone(),
                    ));
                    current_v = next_v;
                }
                path
            })
            .unwrap_or_default())
    }
}

fn heap_permutations<T, F: FnMut(&[T])>(a: &mut [T], mut f: F) {
    let n = a.len();
    let mut c = vec![0; n];
    f(a); // erste Permutation

    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                a.swap(0, i);
            } else {
                a.swap(c[i], i);
            }
            f(a);
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
}
