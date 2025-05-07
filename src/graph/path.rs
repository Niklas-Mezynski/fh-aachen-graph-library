use super::WeightedEdge;

#[derive(Debug, Clone, PartialEq)]
pub struct Path<VId, Edge> {
    edges: Vec<(VId, VId, Edge)>,
}

impl<VId, Edge> Path<VId, Edge>
where
    Edge: WeightedEdge,
{
    pub fn total_cost(&self) -> Edge::WeightType {
        self.edges.iter().map(|(_, _, e)| e.get_weight()).sum()
    }

    pub fn nodes(&self) -> Vec<VId>
    where
        VId: Copy,
    {
        let mut nodes = Vec::new();
        if let Some((from, _, _)) = self.edges.first() {
            nodes.push(*from);
        }
        for (_, to, _) in &self.edges {
            nodes.push(*to);
        }
        nodes
    }

    pub fn push(&mut self, from: VId, to: VId, edge: Edge) {
        self.edges.push((from, to, edge));
    }

    pub fn edges(&self) -> impl Iterator<Item = &(VId, VId, Edge)> {
        self.edges.iter()
    }

    pub fn vertices(&self) -> Box<dyn Iterator<Item = &VId> + '_> {
        if self.edges.is_empty() {
            Box::new(std::iter::empty())
        } else {
            let first = self.edges.first().map(|(from, _, _)| from).into_iter();
            let rest = self.edges.iter().map(|(_, to, _)| to);
            Box::new(first.chain(rest))
        }
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

impl<VId, Edge> Default for Path<VId, Edge> {
    fn default() -> Self {
        Path { edges: Vec::new() }
    }
}

impl<VId: std::fmt::Debug, Edge: std::fmt::Debug> std::fmt::Display for Path<VId, Edge> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, (from, to, edge)) in self.edges.iter().enumerate() {
            writeln!(f, "{}: {:?} -> {:?} via {:?}", i + 1, from, to, edge)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::WeightedEdge;
    use rstest::rstest;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct MockEdge {
        weight: u32,
    }

    impl WeightedEdge for MockEdge {
        type WeightType = u32;
        fn get_weight(&self) -> Self::WeightType {
            self.weight
        }
    }

    #[rstest]
    #[case(
    vec![(1, 2, MockEdge { weight: 10 }), (2, 3, MockEdge { weight: 20 })],
    30,
    vec![1, 2, 3]
)]
    #[case(
    vec![(5, 6, MockEdge { weight: 5 })],
    5,
    vec![5, 6]
)]
    fn test_path_total_cost_and_nodes(
        #[case] edges: Vec<(u32, u32, MockEdge)>,
        #[case] expected_cost: u32,
        #[case] expected_nodes: Vec<u32>,
    ) {
        let path = Path { edges };
        assert_eq!(path.total_cost(), expected_cost);
        assert_eq!(path.nodes(), expected_nodes);
    }

    #[test]
    fn test_path_display() {
        let path = Path {
            edges: vec![
                (1, 2, MockEdge { weight: 10 }),
                (2, 3, MockEdge { weight: 20 }),
            ],
        };
        let output = format!("{}", path);
        assert!(output.contains("1: 1 -> 2 via MockEdge { weight: 10 }"));
        assert!(output.contains("2: 2 -> 3 via MockEdge { weight: 20 }"));
    }

    #[test]
    fn test_vertices_iter_empty() {
        let path: Path<u32, MockEdge> = Path::default();
        let vertices: Vec<_> = path.vertices().cloned().collect();
        assert!(vertices.is_empty());
    }

    #[test]
    fn test_vertices_iter_single_edge() {
        let path = Path {
            edges: vec![(1, 2, MockEdge { weight: 42 })],
        };
        let vertices: Vec<_> = path.vertices().cloned().collect();
        assert_eq!(vertices, vec![1, 2]);
    }

    #[test]
    fn test_vertices_iter_multiple_edges() {
        let path = Path {
            edges: vec![
                (1, 2, MockEdge { weight: 10 }),
                (2, 3, MockEdge { weight: 20 }),
                (3, 4, MockEdge { weight: 30 }),
            ],
        };
        let vertices: Vec<_> = path.vertices().cloned().collect();
        assert_eq!(vertices, vec![1, 2, 3, 4]);
    }
}
