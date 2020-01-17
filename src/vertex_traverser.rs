use crate::*;
use std::collections::HashMap;
use std::iter;
use graph::EdgedGraph;
use vertex::Vertex;
use edge::WeightedEdge;
use vertex_container::{VertexContainer, DfsContainer, BfsContainer, AstarContainer};
use graph_adapters::{Iter, PredecessorIter, PrePostIter, PostIter};

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
pub struct BfsVertexTrav<'a, G, V> {
  graph: &'a G,
  start: V,
  queue: BfsContainer<V>,
  predecessor_map: HashMap<V, Option<V>>
}

impl<'a, G: Graph<V>, V: Vertex> BfsVertexTrav<'a, G, V> {
  pub(crate) fn new(graph: &G, start: V) -> BfsVertexTrav<'_, G, V> {
    let mut container = BfsContainer::new();
    container.push(start.clone());

    BfsVertexTrav {
      graph,
      start: start.clone(),
      queue: container,
      predecessor_map: iter::once((start, None)).collect()
    }
  }
}

impl<'a, G: Graph<V>, V: Vertex> VertexTraverser<V> for BfsVertexTrav<'a, G, V> {
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
          continue;
        }

        self.queue.push(neighbor.clone());
        self.predecessor_map.insert(neighbor.clone(), Some(vertex.clone()));
      }

      vertex
    })
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrePostItem<V> {
  PreorderItem(V),
  PostorderItem(V)
}

pub(crate) enum DfsInnerIterEvent<V> {
  PreorderItem(V),
  PostorderItem(V),
  CycleEdge(V, V),
}

#[derive(Clone)]
pub struct DfsVertexTrav<'a, G, V> {
  graph: &'a G,
  start: V,
  queue: DfsContainer<(V, Option<V>)>,
  predecessor_finished_map: HashMap<V, (Option<V>, bool)>
}

impl<'a, G: Graph<V>, V: Vertex> DfsVertexTrav<'a, G, V> {
  pub(crate) fn new(graph: &G, start: V) -> DfsVertexTrav<'_, G, V> {
    let mut container = DfsContainer::new();
    container.push((start.clone(), None));

    DfsVertexTrav {
      graph,
      start: start.clone(),
      queue: container,
      predecessor_finished_map: HashMap::new()
    }
  }

  pub(crate) fn next_inner(&mut self) -> Option<DfsInnerIterEvent<V>> {
    let vertex = loop {
      let item = self.queue.peek();
      let predecessor_finished_map = &mut self.predecessor_finished_map;

      match (item, item.and_then(|(v, _)| predecessor_finished_map.get_mut(v))) {
        (_, Some((_, finished))) => {
          let item = self.queue.pop();

          if *finished {
            continue;
          } else {
            *finished = true;
            return item.map(|(v, _)| DfsInnerIterEvent::PostorderItem(v));
          }
        },
        (Some((v, p)), None) => {
          predecessor_finished_map.insert(v.clone(), (p.clone(), false));
          break v.clone();
        },
        (None, None) => return None
      }
    };

    for neighbor in self.graph.neighbors(&vertex) {
      self.queue.push((neighbor.clone(), Some(vertex.clone())));
    }

    Some(DfsInnerIterEvent::PreorderItem(vertex))
  }

  pub fn pre_post_iter(&mut self) -> PrePostIter<'_, 'a, G, V> {
    PrePostIter::new(self)
  }

  pub fn post_iter(&mut self) -> PostIter<'_, 'a, G, V> {
    PostIter::new(self)
  }
}

impl<'a, G: Graph<V>, V: Vertex> VertexTraverser<V> for DfsVertexTrav<'a, G, V> {
  fn first(&self) -> V {
    self.start.clone()
  }

  fn predecessor(&self, vertex: &V) -> Option<V> {
    self.predecessor_finished_map.get(vertex)
    .and_then(|(predecessor, _)| predecessor.clone())
  }

  fn next(&mut self) -> Option<V> {
    loop {
      match self.next_inner() {
        Some(DfsInnerIterEvent::PreorderItem(v)) => return Some(v),
        Some(_) => continue,
        None => return None
      }
    }
  }
}

#[derive(Clone)]
pub struct AstarVertexTrav<'a, G, V, E, F> {
  graph: &'a G,
  start: V,
  queue: AstarContainer<(V, E), E>,
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
    let mut container = AstarContainer::new();
    container.push((start.clone(), E::default()), E::default());

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

            self.queue.push((neighbor.clone(), new_edge), score);
            self.predecessor_map.insert(neighbor, Some(vertex.clone()));
          }
        }
      }

      vertex
    })
  }
}
