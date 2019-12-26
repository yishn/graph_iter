use std::ops::Add;

/// A trait alias for all types that can represent an edge.
pub trait Edge: Clone {}
impl<T: Clone> Edge for T {}

/// A trait alias for a weighted edge.
pub trait WeightedEdge: Edge + Ord + Default + Add<Output = Self> {}
impl<T: Edge + Ord + Default + Add<Output = Self>> WeightedEdge for T {}
