//! [`Dynamic Bounding Volume Tree`](../../dbvt/struct.DynamicBoundingVolumeTree.html) accelerated
//! broad phase collision detection algorithm

use std::cmp::Ordering;
use std::fmt::Debug;

use dbvt::{DiscreteVisitor, DynamicBoundingVolumeTree, TreeValue};
use prelude::*;

/// [`Dynamic Bounding Volume Tree`](../../dbvt/struct.DynamicBoundingVolumeTree.html) accelerated
/// broad phase collision detection algorithm
pub struct DbvtBroadPhase;

impl DbvtBroadPhase {
    /// Create a new DbvtBroadPhase
    pub fn new() -> Self {
        Self {}
    }

    /// Find all collider pairs between the shapes in the tree. Will only process the shapes that
    /// are marked as dirty in the given dirty list.
    ///
    /// ## Returns
    ///
    /// A list of tuples of indices into the values list in the tree, sorted by increasing left
    /// index.
    pub fn find_collider_pairs<T>(
        &self,
        tree: &DynamicBoundingVolumeTree<T>,
        dirty: &[bool],
    ) -> Vec<(usize, usize)>
    where
        T: TreeValue,
        T::Bound: Discrete<T::Bound>
            + Clone
            + Debug
            + Contains<T::Bound>
            + SurfaceArea
            + Union<T::Bound, Output = T::Bound>,
    {
        let mut potentials = Vec::default();
        // do intersection tests against tree for each value that is dirty
        for &(shape_node_index, ref shape) in tree.values() {
            // We get the node index from the values list in the tree, and immediately get the value
            // index of said node, so unwrapping will be safe for all cases where the tree isn't
            // corrupt. The tree being corrupt is a programming error, which should be a panic.
            let shape_value_index = tree.value_index(shape_node_index).unwrap();
            if dirty[shape_value_index] {
                for (hit_value_index, _) in
                    tree.query_for_indices(&mut DiscreteVisitor::<T::Bound, T>::new(shape.bound()))
                {
                    let insert = match shape_value_index.cmp(&hit_value_index) {
                        Ordering::Equal => None,
                        Ordering::Less => Some((shape_value_index, hit_value_index)),
                        Ordering::Greater => Some((hit_value_index, shape_value_index)),
                    }.and_then(|pair| {
                        match potentials.binary_search(&pair) {
                            Err(pos) => Some((pos, pair)),
                            Ok(_) => None,
                        }
                    });
                    match insert {
                        Some((pos, pair)) => potentials.insert(pos, pair),
                        None => (),
                    }
                }
            }
        }
        potentials
    }
}
