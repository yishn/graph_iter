use crate::*;
use std::collections::HashMap;
use std::iter;
use graph::EdgedGraph;
use vertex::Vertex;
use edge::WeightedEdge;
use vertex_container::{VertexContainer, DijkstraContainer};
use graph_adapters::{Iter, PredecessorIter};

/// An interface for dealing with vertex traversers over a graph.
pub trait VertexTraverser<V: Vertex> where Self: Sized {
  /// Returns start vertex.
  fn first(&self) -> V;

  /// Advances the traverser and returns the next value.
  fn next(&mut self) -> Option<V>;

  /// Returns the predecessor vertex of the given vertex
  /// or `None` if `vertex` is the start vertex or has not been reached yet.
  fn predecessor(&self, vertex: &V) -> Option<V>;

  /// Returns an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
  /// that lets you iterate over the traverser.
  fn iter(&mut self) -> Iter<'_, V, Self> {
    Iter::new(self)
  }

  /// Returns an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
  /// that lets you iterate over predecessors beginning at a given `vertex`.
  fn predecessor_iter(&self, vertex: &V) -> PredecessorIter<'_, V, Self> {
    PredecessorIter::new(self, vertex.clone())
  }

  /// Traverses through the graph until we reach `target` and returns a path from start vertex
  /// to `target`, or `None` if the `target` vertex cannot be reached.
  fn construct_path(&mut self, target: &V) -> Option<Vec<V>> {
    if self.predecessor(target).is_none() {
      self.iter().find(|v| v == target);
    }

    let mut path = self.predecessor_iter(target).collect::<Vec<_>>();
    path.reverse();

    if path.len() > 1 || target == &self.first() {
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
  predecessor_map: HashMap<V, Option<V>>,
  cycle_detected: bool
}

impl<'a, G, V, C> DefaultVertexTrav<'a, G, V, C>
where G: Graph<V>, V: Vertex, C: VertexContainer<V> {
  pub(crate) fn new(graph: &G, start: V) -> DefaultVertexTrav<'_, G, V, C> {
    let mut container = C::new();
    container.push(start.clone());

    DefaultVertexTrav {
      graph,
      start: start.clone(),
      queue: container,
      predecessor_map: iter::once((start, None)).collect(),
      cycle_detected: false
    }
  }

  /// Returns `true` if we can reach a cycle by traversing the graph starting
  /// with the start vertex.
  pub fn is_cyclic(mut self) -> bool {
    while !self.cycle_detected {
      if let None = self.next() {
        break;
      }
    }

    self.cycle_detected
  }
}

impl<'a, G, V, C> VertexTraverser<V> for DefaultVertexTrav<'a, G, V, C>
where G: Graph<V>, V: Vertex, C: VertexContainer<V> {
  fn first(&self) -> V {
    self.start.clone()
  }

  fn predecessor(&self, vertex: &V) -> Option<V> {
    self.predecessor_map.get(vertex)
    .and_then(|predecessor| predecessor.clone())
  }

  fn next(&mut self) -> Option<V> {
    let vertex = self.queue.pop();

    vertex.map(|vertex| {
      for neighbor in self.graph.neighbors(&vertex) {
        if self.predecessor_map.contains_key(&neighbor) {
          self.cycle_detected = true;
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

  fn predecessor(&self, vertex: &V) -> Option<V> {
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
