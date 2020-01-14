use crate::*;
use std::marker::PhantomData;
use graph::*;
use vertex_traverser::VertexTraverser;

pub struct Iter<'a, V, T>(&'a mut T, PhantomData<&'a V>);

impl<'a, V: Vertex, T: VertexTraverser<V>> Iter<'a, V, T> {
  pub(crate) fn new(traverser: &'a mut T) -> Iter<'a, V, T> {
    Iter(traverser, PhantomData)
  }
}

impl<'a, V: Vertex, T: VertexTraverser<V>> Iterator for Iter<'a, V, T> {
  type Item = V;

  fn next(&mut self) -> Option<V> {
    self.0.next()
  }
}

pub struct PredecessorIter<'a, V, T>(&'a T, Option<V>);

impl<'a, V: Vertex, T: VertexTraverser<V>> PredecessorIter<'a, V, T> {
  pub(crate) fn new(traverser: &'a T, vertex: V) -> PredecessorIter<'a, V, T> {
    PredecessorIter(traverser, Some(vertex))
  }
}

impl<'a, V: Vertex, T: VertexTraverser<V>> Iterator for PredecessorIter<'a, V, T> {
  type Item = V;

  fn next(&mut self) -> Option<V> {
    self.1.clone().map(|vertex| {
      let predecessor = self.0.predecessor(&vertex);
      self.1 = predecessor;

      vertex
    })
  }
}

#[derive(Clone)]
pub struct Reversed<'a, T> {
  graph: &'a T
}

impl<'a, T> Reversed<'a, T> {
  pub(crate) fn new(graph: &'a T) -> Reversed<'a, T> {
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
