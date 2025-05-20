use std::hash::Hash;

use rustc_hash::FxHashMap;

/// A structure that holds information about the shortest path in a graph
///
/// - `costs` is a `HashMap` that maps from vertex id to path costs
/// - `predecessor` is a `HashMap` that maps `VertexID` to the predecessor `VertexID` that can be used to reconstruct the path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleSourceShortestPaths<VId: Hash + Eq, Cost> {
    start: VId,
    costs: FxHashMap<VId, Cost>,
    predecessors: FxHashMap<VId, VId>,
}

impl<VId, Cost> SingleSourceShortestPaths<VId, Cost>
where
    VId: Hash + Eq + Copy,
    Cost: Copy,
{
    pub fn new(start: VId, costs: FxHashMap<VId, Cost>, predecessors: FxHashMap<VId, VId>) -> Self {
        Self {
            start,
            costs,
            predecessors,
        }
    }

    pub fn start(&self) -> VId {
        self.start
    }

    /// Gets the cost from the start vertex to `target` vertex
    pub fn get_cost(&self, target: VId) -> Option<Cost> {
        self.costs.get(&target).copied()
    }

    /// Reconstruct the (shortest) path that is taken to get from the
    /// start vertex to `target`
    pub fn get_path(&self, target: VId) -> Vec<VId> {
        let mut path = vec![];
        let mut current = target;

        // If the target is not reachable, return an empty path
        if !self.costs.contains_key(&target) {
            return path;
        }

        // Walk backwards from target to start using predecessors
        while current != self.start {
            path.push(current);
            match self.predecessors.get(&current) {
                Some(&pred) => current = pred,
                None => return vec![], // No path exists
            }
        }
        path.push(self.start);
        path.reverse();
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rustc_hash::FxHashMap;

    #[rstest]
    fn test_get_cost_existing() {
        let mut costs = FxHashMap::default();
        costs.insert(1, 0);
        costs.insert(2, 5);
        let predecessors = FxHashMap::default();
        let sp = SingleSourceShortestPaths::new(1, costs, predecessors);
        assert_eq!(sp.get_cost(1), Some(0));
        assert_eq!(sp.get_cost(2), Some(5));
    }

    #[rstest]
    fn test_get_cost_non_existing() {
        let costs = FxHashMap::default();
        let predecessors = FxHashMap::default();
        let sp: SingleSourceShortestPaths<i32, f32> =
            SingleSourceShortestPaths::new(1, costs, predecessors);
        assert_eq!(sp.get_cost(2), None);
    }

    #[rstest]
    fn test_get_path_simple() {
        // 1 -> 2 -> 3
        let mut costs = FxHashMap::default();
        costs.insert(1, 0);
        costs.insert(2, 1);
        costs.insert(3, 2);

        let mut predecessors = FxHashMap::default();
        predecessors.insert(2, 1);
        predecessors.insert(3, 2);

        let sp = SingleSourceShortestPaths::new(1, costs, predecessors);
        assert_eq!(sp.get_path(3), vec![1, 2, 3]);
        assert_eq!(sp.get_path(2), vec![1, 2]);
        assert_eq!(sp.get_path(1), vec![1]);
    }

    #[rstest]
    fn test_get_path_unreachable() {
        let mut costs = FxHashMap::default();
        costs.insert(1, 0);

        let predecessors = FxHashMap::default();

        let sp = SingleSourceShortestPaths::new(1, costs, predecessors);
        // 2 is not reachable
        assert_eq!(sp.get_path(2), Vec::<i32>::new());
    }

    #[rstest]
    fn test_get_path_no_predecessor() {
        // 1 -> 2, but 3 is in costs without a predecessor
        let mut costs = FxHashMap::default();
        costs.insert(1, 0);
        costs.insert(2, 1);
        costs.insert(3, 2);

        let mut predecessors = FxHashMap::default();
        predecessors.insert(2, 1);
        // 3 has no predecessor

        let sp = SingleSourceShortestPaths::new(1, costs, predecessors);
        // Should return an empty vec as no path from 1 to 3 exists
        assert_eq!(sp.get_path(3), vec![]);
    }

    #[rstest]
    fn test_start() {
        let costs = FxHashMap::default();
        let predecessors = FxHashMap::default();
        let sp: SingleSourceShortestPaths<i32, f32> =
            SingleSourceShortestPaths::new(42, costs, predecessors);
        assert_eq!(sp.start(), 42);
    }
}
