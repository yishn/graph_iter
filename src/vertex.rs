use std::hash::Hash;

pub trait Vertex: Hash + Eq + Clone {}
impl<T: Hash + Eq + Clone> Vertex for T {}
