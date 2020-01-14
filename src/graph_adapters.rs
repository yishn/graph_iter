use crate::*;
use graph::*;
use edge::Edge;
use finite_graph::{Id, FiniteGraph};

pub struct NeighborsIter<I: Iterator<Item = (Id, Id)>> {
  iter: Option<I>
}

pub struct EdgesIter<'a, V, E: Edge, I: Iterator<Item = (Id, Id)>> {
  graph: &'a FiniteGraph<V, E>,
  iter: Option<I>,
  other: Id
}

impl<'a, V, E: Edge, I: Iterator<Item = (Id, Id)>> Iterator for EdgesIter<'a, V, E, I> {
  type Item = E;

  fn next(&mut self) -> Option<E> {
    let graph = self.graph;
    let other = self.other;

    self.iter.as_mut().and_then(|iter| {
      iter
      .find(|(v, _)| v == &other)
      .and_then(|(_, e)| {
        graph.get_edge(e).cloned()
      })
    })
  }
}

impl<I: Iterator<Item = (Id, Id)>> Iterator for NeighborsIter<I> {
  type Item = Id;

  fn next(&mut self) -> Option<Id> {
    self.iter.as_mut().and_then(|iter| {
      iter.next().map(|(v, _)| v)
    })
  }
}

pub struct GraphIter<'a, V, E> {
  graph: &'a FiniteGraph<V, E>
}

impl<'a, V, E> GraphIter<'a, V, E> {
  pub fn new(graph: &'a FiniteGraph<V, E>) -> GraphIter<'a, V, E> {
    GraphIter {
      graph
    }
  }
}

impl<'a, V, E> Graph<Id> for GraphIter<'a, V, E> {
  type NeighborsIterator = NeighborsIter<std::iter::Copied<std::slice::Iter<'a, (Id, Id)>>>;

  fn neighbors(&self, vertex: &Id) -> Self::NeighborsIterator {
    NeighborsIter {
      iter: self.graph.neighbors_map.get(vertex)
        .map(|neighbors| neighbors.iter().copied())
    }
  }
}

impl<'a, V, E> ReversableGraph<Id> for GraphIter<'a, V, E> {
  type ReverseNeighborsIterator = Self::NeighborsIterator;

  fn reverse_neighbors(&self, vertex: &Id) -> Self::NeighborsIterator {
    NeighborsIter {
      iter: self.graph.reverse_neighbors_map.get(vertex)
        .map(|neighbors| neighbors.iter().copied())
    }
  }
}

impl<'a, V, E: Edge> EdgedGraph<Id, E> for GraphIter<'a, V, E> {
  type EdgesIterator = EdgesIter<'a, V, E, std::iter::Copied<std::slice::Iter<'a, (Id, Id)>>>;

  fn edges(&self, vertex: &Id, other: &Id) -> Self::EdgesIterator {
    EdgesIter {
      graph: self.graph,
      iter: self.graph.neighbors_map.get(vertex)
        .map(|neighbors| neighbors.iter().copied()),
      other: *other
    }
  }
}

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
