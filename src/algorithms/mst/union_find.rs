// Union-Find-Struct for detecting circles in a graph

use std::{fmt::Debug, hash::Hash};

use rustc_hash::FxHashMap;
use thiserror::Error;

#[derive(Debug)]
pub struct UnionFind<VId>
where
    VId: Eq + Hash + Copy,
{
    sets: FxHashMap<VId, VId>,
}

impl<VId> UnionFind<VId>
where
    VId: Eq + Hash + Copy + Debug,
{
    pub fn new() -> Self {
        UnionFind {
            sets: FxHashMap::default(),
        }
    }

    /// Adds a new Set with x as parent
    /// MakeSet(x)
    pub fn make_set(&mut self, x: VId) -> Result<(), UnionFindError<VId>> {
        match self.sets.insert(x, x) {
            Some(_) => Err(UnionFindError::DuplicateVertex(x)),
            None => Ok(()),
        }
    }

    /// Returns the parent of x
    /// Also applies path compression while searching (adding all nodes on the way directly to the parent)
    pub fn find(&mut self, x: VId) -> Result<VId, UnionFindError<VId>> {
        // Remember nodes we visit along the way
        let mut visited = vec![];

        // Find the parent
        let parent = {
            let mut current = &x;
            let mut parent = self.sets.get(&x).ok_or(UnionFindError::VertexNotFound(x))?;

            // Walk up the chain until we find the parent
            while current != parent {
                visited.push(*current);

                current = parent;
                parent = self
                    .sets
                    .get(parent)
                    .ok_or(UnionFindError::VertexNotFound(x))?;
            }

            *parent
        };

        // Path compression
        for visited in visited.into_iter() {
            self.sets.entry(visited).and_modify(|entry| *entry = parent);
        }

        Ok(parent)
    }

    /// The disjunct sets x and y are merged, the new parent is the parent of y
    pub fn union(&mut self, x: VId, y: VId) -> Result<(), UnionFindError<VId>> {
        let parent_x = self.find(x)?;
        let parent_y = self.find(y)?;

        if parent_x == parent_y {
            return Err(UnionFindError::NotDisjunct(x, y, parent_x));
        }

        self.sets
            .entry(parent_x)
            .and_modify(|entry| *entry = parent_y);

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum UnionFindError<VId> {
    #[error("Vertex with ID {0} not found")]
    VertexNotFound(VId),

    #[error("Vertex with ID {0} already exists")]
    DuplicateVertex(VId),

    #[error("Sets not disjunct. {0} and {1} both have {2} as parent.")]
    NotDisjunct(VId, VId, VId),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_make_set() {
        let mut union_find = UnionFind::<u32>::new();

        // Test adding two different nodes
        assert!(union_find.make_set(1).is_ok());
        assert!(union_find.make_set(2).is_ok());

        // Test adding the same node again
        let result = union_find.make_set(1);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UnionFindError::DuplicateVertex(1)
        ));
    }

    #[fixture]
    fn test_struct() -> UnionFind<u32> {
        let mut union_find = UnionFind::<u32>::new();

        for i in 1u32..=9 {
            union_find.make_set(i).unwrap_or_else(|e| {
                panic!("Failed to create initial union find struct setup: {:?}", e)
            });
        }

        union_find
    }

    #[rstest]
    fn test_find(test_struct: UnionFind<u32>) {
        let mut union_find = test_struct;

        // Test finding existing nodes
        assert_eq!(union_find.find(1).unwrap(), 1);
        assert_eq!(union_find.find(9).unwrap(), 9);

        // Test finding a non existing node
        let result = union_find.find(0);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UnionFindError::VertexNotFound(0)
        ));
    }

    #[rstest]
    fn test_union(test_struct: UnionFind<u32>) {
        let mut test_struct = test_struct;

        // Test that union works
        assert!(test_struct.union(1, 2).is_ok());
        assert!(test_struct.union(1, 3).is_ok());

        // Test that union fails if the sets are not disjunct
        let result = test_struct.union(2, 3);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UnionFindError::NotDisjunct(2, 3, 3)
        ));
    }

    #[rstest]
    /// Important! Reference for these test values can be found in Fig. 4 of the Kruskal Algorithm explanation  
    /// https://www.hoever-downloads.fh-aachen.de/MathAlg/Unterlagen/geschuetzt/1-2_a_MST.pdf
    fn test_union_and_find(test_struct: UnionFind<u32>) {
        let mut union_find = test_struct;

        // Test that union works
        assert!(union_find.union(1, 2).is_ok());
        assert!(union_find.union(1, 3).is_ok());
        assert!(union_find.union(2, 4).is_ok());
        assert!(union_find.union(2, 5).is_ok());
        assert!(union_find.union(6, 7).is_ok());

        // Test union failing case
        let result = union_find.union(3, 2);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UnionFindError::NotDisjunct(3, 2, 5)
        ));

        // Test that find still returns correct values for all nodes
        assert_eq!(union_find.find(1).unwrap(), 5);
        assert_eq!(union_find.find(2).unwrap(), 5);
        assert_eq!(union_find.find(3).unwrap(), 5);
        assert_eq!(union_find.find(4).unwrap(), 5);
        assert_eq!(union_find.find(5).unwrap(), 5);
        assert_eq!(union_find.find(6).unwrap(), 7);
        assert_eq!(union_find.find(7).unwrap(), 7);
        assert_eq!(union_find.find(8).unwrap(), 8);
        assert_eq!(union_find.find(9).unwrap(), 9);
    }
}
