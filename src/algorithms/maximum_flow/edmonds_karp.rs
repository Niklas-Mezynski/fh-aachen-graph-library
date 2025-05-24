use std::{
    collections::VecDeque,
    hash::Hash,
    ops::{Add, Sub},
};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    graph::{GraphBase, WithID},
    Directed, Graph, GraphError,
};

#[derive(Debug, Clone)]
pub struct ResidualEdge<Flow> {
    flow: Flow,
    is_residual: bool,
}

impl<Backend> Graph<Backend>
where
    Backend: GraphBase<Direction = Directed>,
    Backend::Vertex: Clone,
    <Backend::Vertex as WithID>::IDType: Copy + Eq + Hash,
    Backend::Edge: Clone,
{
    /// Edmonds-Karp-Algorithm
    ///
    /// Returns ...
    pub fn edmonds_karp<ResBackend, Flow, FlowFn, MaxFlowFn>(
        &mut self,
        start: <Backend::Vertex as WithID>::IDType,
        target: <Backend::Vertex as WithID>::IDType,
        flow: FlowFn,
        max_flow: MaxFlowFn,
    ) -> Result<(), GraphError<<Backend::Vertex as WithID>::IDType>>
    where
        FlowFn: Fn(&mut Backend::Edge) -> &mut Flow,
        ResBackend:
            GraphBase<Vertex = Backend::Vertex, Edge = ResidualEdge<Flow>, Direction = Directed>,
        MaxFlowFn: Fn(&Backend::Edge) -> &Flow,
        Flow: Default + Copy + PartialEq + PartialOrd + Sub<Output = Flow> + Add<Output = Flow>,
    {
        if start == target {
            todo!()
        }
        // 1. Starte mit Fluss f(u, v) = 0 ∀(u, v) ∈ E
        // Set all flow values to 0
        // self.get_all_edges_mut()
        //     .for_each(|(_, _, edge)| *flow(edge) = Flow::default());

        // 2. Bestimme den Residualgraph Gf für f
        // Copy the original graph and perform all operations on this (residual) graph
        // Later we map back the edges to the original one
        let res_edges: Vec<_> = self
            .get_all_edges()
            // All edges in the "main direction" have their max flow potential in the beginning
            .map(|(from, to, edge)| {
                (
                    from,
                    to,
                    ResidualEdge {
                        flow: *max_flow(edge),
                        is_residual: false,
                    },
                )
            })
            .chain(
                // Also add all edges in the other direction, with 0 as their initial "potential"
                self.get_all_edges().map(|(from, to, _edge)| {
                    (
                        to,
                        from,
                        ResidualEdge {
                            flow: Flow::default(),
                            is_residual: true,
                        },
                    )
                }),
            )
            .collect();

        let mut res = Graph::<ResBackend>::from_vertices_and_edges(
            self.get_all_vertices().cloned().collect(),
            res_edges,
        )?;

        loop {
            // 3. Finde den kürzesten Weg (Anzahl der Kanten) von s zu t in Gf
            //    Wenn es keinen Weg gibt: Stoppe mit f
            let path = Self::find_shortest_path::<Flow, ResBackend>(&res, start, target);

            if let Some(path) = path {
                // 4. Verändere f entlang des identifizierten Wegs um die kleinste Kantenkapazität γ des Weges
                let min = path
                    .windows(2)
                    .map(|window| {
                        res.get_edge(window[0], window[1])
                            .expect("Edge must exist")
                            .flow
                    })
                    .min_by(|this, other| {
                        this.partial_cmp(other)
                            .expect("Graph capacities must not contain NaN values")
                    })
                    .expect("Path exist");

                // Update all flows by the current value
                path.windows(2).for_each(|window| {
                    let edge = res
                        .get_edge_mut(window[0], window[1])
                        .expect("Edge must exist");

                    if edge.is_residual {
                        // Subtract min from flow, in the residual graph we have to add
                        edge.flow = edge.flow + min;
                    } else {
                        // Vice versa
                        edge.flow = edge.flow - min;
                    }
                })
            } else {
                // No path found, we are done
                break;
            }

            // 5. Springe zurück zu Schritt 2
        }

        // Apply flows found in residual graph to the main graph
        for (from, to, edge) in res
            .get_all_edges()
            .filter(|(_from, _to, edge)| !edge.is_residual)
        {
            let edge_to_modify = self
                .get_edge_mut(from, to)
                .expect("Edge must also exist in original graph");

            *flow(edge_to_modify) = *max_flow(edge_to_modify) - edge.flow;
        }

        Ok(())
    }

    /// Find an shortest path (in terms of edge count) from start to target using BFS
    fn find_shortest_path<Flow, ResBackend>(
        res: &Graph<ResBackend>,
        start: <Backend::Vertex as WithID>::IDType,
        target: <Backend::Vertex as WithID>::IDType,
    ) -> Option<Vec<<Backend::Vertex as WithID>::IDType>>
    where
        ResBackend: GraphBase<
            Vertex = Backend::Vertex,
            Edge = ResidualEdge<Flow>,
            Direction = Backend::Direction,
        >,
        Flow: Default + Copy + PartialEq,
    {
        let mut visited = FxHashSet::default();
        let mut pred = FxHashMap::default();

        let mut queue = VecDeque::from([start]);
        visited.insert(start);
        pred.insert(start, start);

        // Modified bfs that also keeps track of paths (predecessors)
        'outer: while let Some(current) = queue.pop_front() {
            for (to, _edge) in res
                .get_adjacent_vertices_with_edges(current)
                .filter(|(_, e)| e.flow != Flow::default())
            {
                let to = to.get_id();
                if !visited.contains(&to) {
                    visited.insert(to);
                    queue.push_back(to);
                    pred.insert(to, current);
                }

                // Abort if we found our target vertex
                if to == target {
                    break 'outer;
                }
            }
        }

        // Reconstruct the path if target was reached
        if visited.contains(&target) {
            let mut path = Vec::new();
            let mut current = target;
            while current != start {
                path.push(current);
                current = pred[&current];
            }
            path.push(start);
            path.reverse();
            Some(path)
        } else {
            None
        }
    }
}
