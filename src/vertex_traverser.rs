use crate::*;
use std::collections::HashMap;
use std::iter;
use std::marker::PhantomData;
use graph::EdgedGraph;
use vertex::Vertex;
use edge::WeightedEdge;
use vertex_container::{VertexContainer, DijkstraContainer};

pub struct Iter<'a, V: Vertex, T: VertexTraverser<V>>(&'a mut T, PhantomData<V>);

impl<'a, V: Vertex, T: VertexTraverser<V>> Iterator for Iter<'a, V, T> {
  type Item = V;

  fn next(&mut self) -> Option<V> {
    self.0.next()
  }
}

/// An interface for dealing with vertex traversers over a graph.
pub trait VertexTraverser<V: Vertex> {
  /// Returns start vertex.
  fn first(&self) -> V;

  /// Returns the predecessor vertex of the given vertex
  /// or `None` if `vertex` is the start vertex or is not reachable.
  fn predecessor(&mut self, vertex: &V) -> Option<V>;

  /// Advances the traverser and returns the next value.
  fn next(&mut self) -> Option<V>;

  /// Returns an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
  /// that let's you iterate over the traverser.
  fn iter(&mut self) -> Iter<'_, V, Self> where Self: Sized {
    Iter(self, PhantomData)
  }

  /// Returns a path from start vertex to `target` or `None` if
  /// there's no such path.
  fn construct_path(&mut self, target: &V) -> Option<Vec<V>> {
    let mut path = vec![target.clone()];

    while let Some(previous) = self.predecessor(path.last().unwrap()) {
      path.push(previous);
    }

    path.reverse();

    if path[0] == self.first() {
      Some(path)
    } else {
      None
    }
  }
}

#[derive(Clone)]
pub struct DefaultVertexTrav<'a, G, V, C>
where G: Graph<V>, V: Vertex, C: VertexContainer<V> {
  graph: &'a G,
  start: V,
  queue: C,
  predecessor_map: HashMap<V, Option<V>>
}

impl<'a, G, V, C> DefaultVertexTrav<'a, G, V, C>
where G: Graph<V>, V: Vertex, C: VertexContainer<V> {
  pub(crate) fn new(graph: &G, start: V) -> DefaultVertexTrav<'_, G, V, C> where C: Sized {
    let mut container = C::new();
    container.push(start.clone());

    DefaultVertexTrav {
      graph,
      start: start.clone(),
      queue: container,
      predecessor_map: iter::once((start, None)).collect()
    }
  }
}

impl<'a, G, V, C> VertexTraverser<V> for DefaultVertexTrav<'a, G, V, C>
where G: Graph<V>, V: Vertex, C: VertexContainer<V> {
  fn first(&self) -> V {
    self.start.clone()
  }

  fn predecessor(&mut self, vertex: &V) -> Option<V> {
    if !self.predecessor_map.contains_key(vertex) {
      self.iter().find(|v| v == vertex);
    }

    self.predecessor_map.get(vertex)
    .and_then(|predecessor| predecessor.clone())
  }

  fn next(&mut self) -> Option<V> {
    let vertex = self.queue.pop();

    vertex.map(|vertex| {
      for neighbor in self.graph.neighbors(&vertex) {
        if self.predecessor_map.contains_key(&neighbor) {
          continue;
        }

        self.queue.push(neighbor.clone());
        self.predecessor_map.insert(neighbor.clone(), Some(vertex.clone()));
      }

      vertex
    })
  }
}

#[derive(Clone)]
pub struct AstarVertexTrav<'a, G, V, E, F> {
  graph: &'a G,
  start: V,
  queue: DijkstraContainer<(V, E), E>,
  predecessor_map: HashMap<V, Option<V>>,
  min_edge_map: HashMap<V, E>,
  estimator: Option<F>
}

impl<'a, G, V, E, F> AstarVertexTrav<'a, G, V, E, F>
where
  G: EdgedGraph<V, E>,
  V: Vertex,
  E: WeightedEdge,
  F: Fn(&V) -> E
{
  pub(crate) fn new(graph: &G, start: V) -> AstarVertexTrav<'_, G, V, E, F> {
    let mut container = DijkstraContainer::new();
    container.push(((start.clone(), E::default()), E::default()));

    AstarVertexTrav {
      graph,
      start: start.clone(),
      queue: container,
      predecessor_map: iter::once((start.clone(), None)).collect(),
      min_edge_map: iter::once((start, E::default())).collect(),
      estimator: None
    }
  }

  pub(crate) fn with_estimator(graph: &G, start: V, estimator: F) -> AstarVertexTrav<'_, G, V, E, F> {
    let mut result = AstarVertexTrav::new(graph, start);
    result.estimator = Some(estimator);

    result
  }
}

impl<'a, G, V, E, F> VertexTraverser<V> for AstarVertexTrav<'a, G, V, E, F>
where
  G: EdgedGraph<V, E>,
  V: Vertex,
  E: WeightedEdge,
  F: Fn(&V) -> E
{
  fn first(&self) -> V {
    self.start.clone()
  }

  fn predecessor(&mut self, vertex: &V) -> Option<V> {
    if !self.predecessor_map.contains_key(vertex) {
      self.iter().find(|v| v == vertex);
    }

    self.predecessor_map.get(vertex)
    .and_then(|predecessor| predecessor.clone())
  }

  fn next(&mut self) -> Option<V> {
    let vertex_edge = self.queue.pop();

    vertex_edge.map(|((vertex, edge), _)| {
      for neighbor in self.graph.neighbors(&vertex) {
        let outgoing_edge = self.graph
          .edges(&vertex, &neighbor)
          .into_iter()
          .min();

        if let Some(outgoing_edge) = outgoing_edge {
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
            let mut score = new_edge.clone();

            if let Some(estimator) = self.estimator.as_ref() {
              score = score + estimator(&neighbor);
            }

            self.queue.push(((neighbor.clone(), new_edge), score));
            self.predecessor_map.insert(neighbor, Some(vertex.clone()));
          }
        }
      }

      vertex
    })
  }
}
