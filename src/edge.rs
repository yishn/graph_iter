pub trait Edge: Clone {}
impl<T: Clone> Edge for T {}

pub trait WeightedEdge: Edge + Ord {}
impl<T: Edge + Ord> WeightedEdge for T {}
