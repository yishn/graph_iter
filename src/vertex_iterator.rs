use crate::*;
use std::collections::HashMap;
use std::iter;
use std::marker::PhantomData;
use graph::EdgedGraph;
use vertex::Vertex;
use edge::{Edge, WeightedEdge};
use vertex_container::{VertexContainer, DijkstraContainer};

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
pub struct DefaultVertexIter<'a, G, V, E, C>
where G: Graph<V, E>, V: Vertex, E: Edge, C: VertexContainer<V> {
  graph: &'a G,
  start: V,
  queue: C,
  predecessor_map: HashMap<V, Option<V>>,
  phantom: PhantomData<E>
}

impl<'a, G, V, E, C> DefaultVertexIter<'a, G, V, E, C>
where G: Graph<V, E>, V: Vertex, E: Edge, C: VertexContainer<V> {
  pub(crate) fn new(graph: &G, start: V) -> DefaultVertexIter<'_, G, V, E, C> where C: Sized {
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
where G: Graph<V, E>, V: Vertex, E: Edge, C: VertexContainer<V> {
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
      for neighbor in self.graph.get_neighbors(&vertex) {
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

#[derive(Clone)]
pub struct DijkstraVertexIter<'a, G, V, E>
where G: EdgedGraph<V, E>, V: Vertex, E: WeightedEdge {
  graph: &'a G,
  start: V,
  queue: DijkstraContainer<V, E>,
  predecessor_map: HashMap<V, Option<V>>,
  min_edge_map: HashMap<V, E>
}

impl<'a, G, V, E> DijkstraVertexIter<'a, G, V, E>
where G: EdgedGraph<V, E>, V: Vertex, E: WeightedEdge {
  pub(crate) fn new(graph: &G, start: V) -> DijkstraVertexIter<'_, G, V, E> {
    let mut container = DijkstraContainer::new();
    container.push((start.clone(), E::default()));

    DijkstraVertexIter {
      graph,
      start: start.clone(),
      queue: container,
      predecessor_map: iter::once((start.clone(), None)).collect(),
      min_edge_map: iter::once((start, E::default())).collect()
    }
  }
}

impl<'a, G, V, E> VertexIterator<V> for DijkstraVertexIter<'a, G, V, E>
where G: EdgedGraph<V, E>, V: Vertex, E: WeightedEdge {
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
    let vertex_edge = self.queue.pop();

    vertex_edge.map(|(vertex, edge)| {
      for (neighbor, outgoing_edge) in self.graph.get_neighbors_with_edges(&vertex) {
        let new_edge = edge.clone() + outgoing_edge;
        let mut edge_shorter = false;

        if let Some(min_edge) = self.min_edge_map.get_mut(&neighbor) {
          if &new_edge < min_edge {
            *min_edge = new_edge.clone();
            edge_shorter = true;
          }
        } else {
          self.min_edge_map.insert(neighbor.clone(), new_edge.clone());
          edge_shorter = true;
        }

        if edge_shorter {
          self.queue.push((neighbor.clone(), new_edge));
          self.predecessor_map.insert(neighbor, Some(vertex.clone()));
        }
      }

      vertex
    })
  }
}
