use crate::*;
use graph::*;

#[derive(Clone)]
pub struct Reversed<'a, T> {
  graph: &'a T
}

impl<'a, T> Reversed<'a, T> {
  pub fn new(graph: &'a T) -> Reversed<'a, T> {
    Reversed {
      graph
    }
  }
}

impl<'a, V: Vertex, T: ReversableGraph<V>> Graph<V> for Reversed<'a, T> {
  type NeighborsIterator = T::ReverseNeighborsIterator;

  fn neighbors(&self, vertex: &V) -> Self::NeighborsIterator {
    self.graph.reverse_neighbors(vertex)
  }
}

impl<'a, V: Vertex, T: ReversableGraph<V>> ReversableGraph<V> for Reversed<'a, T> {
  type ReverseNeighborsIterator = T::NeighborsIterator;

  fn reverse_neighbors(&self, vertex: &V) -> Self::ReverseNeighborsIterator {
    self.graph.neighbors(vertex)
  }
}

impl<'a, V: Vertex, E: Edge, T> EdgedGraph<V, E> for Reversed<'a, T>
where T: ReversableGraph<V> + EdgedGraph<V, E> {
  type EdgesIterator = T::EdgesIterator;

  fn edges(&self, vertex: &V, other: &V) -> Self::EdgesIterator {
    self.graph.edges(other, vertex)
  }
}