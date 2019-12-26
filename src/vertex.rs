use std::hash::Hash;

/// A trait alias for all types that can represent a vertex.
pub trait Vertex: Hash + Eq + Clone {}
impl<T: Hash + Eq + Clone> Vertex for T {}
