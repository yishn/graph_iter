use crate::*;
use std::collections::HashMap;
use std::iter;
use std::marker::PhantomData;
use graph::EdgedGraph;
use vertex::Vertex;
use edge::Edge;
use vertex_container::VertexContainer;

pub struct Iter<'a, V: Vertex, I: VertexIterator<V>>(&'a mut I, PhantomData<V>);

impl<'a, V: Vertex, I: VertexIterator<V>> Iterator for Iter<'a, V, I> {
  type Item = V;

  fn next(&mut self) -> Option<V> {
    self.0.next()
  }
}

/// An interface for dealing with vertex iterators over a graph.
pub trait VertexIterator<V: Vertex> {
  /// Returns start vertex.
  fn get_start(&self) -> V;

  /// Returns the predecessor vertex of the given vertex
  /// or `None` if `vertex` is the start vertex or is not reachable.
  fn get_predecessor(&mut self, vertex: &V) -> Option<V>;

  /// Advances the iterator and returns the next value.
  fn next(&mut self) -> Option<V>;

  /// Returns an actual iterator.
  fn iter(&mut self) -> Iter<'_, V, Self> where Self: Sized {
    Iter(self, PhantomData)
  }

  /// Returns a path from start vertex to `target` or `None` if
  /// there's no such path.
  fn construct_path(&mut self, target: &V) -> Option<Vec<V>> {
    let mut path = vec![target.clone()];

    while let Some(previous) = self.get_predecessor(path.last().unwrap()) {
      path.push(previous.clone());
    }

    path.reverse();

    if path[0] == self.get_start() {
      Some(path)
    } else {
      None
    }
  }
}

#[derive(Clone)]
pub struct DefaultVertexIter<'a, G: EdgedGraph<V, E>, V: Vertex, E: Edge, C: VertexContainer<V>> {
  graph: &'a G,
  start: V,
  queue: C,
  predecessor_map: HashMap<V, Option<V>>,
  phantom: PhantomData<E>
}

impl<'a, G, V, E, C> DefaultVertexIter<'a, G, V, E, C>
where G: EdgedGraph<V, E>, V: Vertex, E: Edge, C: VertexContainer<V> {
  pub(crate) fn new(graph: &'a G, start: V) -> DefaultVertexIter<'a, G, V, E, C> where C: Sized {
    let mut container = C::new();
    container.push(start.clone());

    DefaultVertexIter {
      graph,
      start: start.clone(),
      queue: container,
      predecessor_map: iter::once((start, None)).collect(),
      phantom: PhantomData
    }
  }
}

impl<'a, G, V, E, C> VertexIterator<V> for DefaultVertexIter<'a, G, V, E, C>
where G: EdgedGraph<V, E>, V: Vertex, E: Edge, C: VertexContainer<V> {
  fn get_start(&self) -> V {
    self.start.clone()
  }

  fn get_predecessor(&mut self, vertex: &V) -> Option<V> {
    if !self.predecessor_map.contains_key(vertex) {
      self.iter().find(|v| v == vertex);
    }

    self.predecessor_map.get(vertex)
    .and_then(|x| x.clone())
  }

  fn next(&mut self) -> Option<V> {
    let vertex = self.queue.pop();

    vertex.map(|vertex| {
      for (neighbor, _) in self.graph.get_neighbors_with_edges(vertex.clone()) {
        if self.predecessor_map.contains_key(&neighbor) {
          continue;
        }

        self.queue.push(neighbor.clone());
        self.predecessor_map.insert(neighbor, Some(vertex.clone()));
      }

      vertex
    })
  }
}
