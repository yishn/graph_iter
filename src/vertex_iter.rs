use std::cmp::Reverse;
use std::collections::{VecDeque, HashMap, BinaryHeap};
use std::iter;
use crate::graph::Graph;
use crate::vertex::Vertex;

pub type DfsContainer<V> = Vec<V>;
pub type BfsContainer<V> = VecDeque<V>;
pub type DijkstraContainer<V> = BinaryHeap<Reverse<V>>;

pub trait VertexContainer<V> {
  fn new() -> Self;
  fn pop(&mut self) -> Option<V>;
  fn push(&mut self, vertex: V);
}

impl<V> VertexContainer<V> for DfsContainer<V> {
  fn new() -> DfsContainer<V> {
    Vec::new()
  }

  fn pop(&mut self) -> Option<V> {
    self.pop()
  }

  fn push(&mut self, vertex: V) {
    self.push(vertex);
  }
}

impl<V> VertexContainer<V> for BfsContainer<V> {
  fn new() -> BfsContainer<V> {
    VecDeque::new()
  }

  fn pop(&mut self) -> Option<V> {
    self.pop_front()
  }

  fn push(&mut self, vertex: V) {
    self.push_back(vertex);
  }
}

impl<V: Ord> VertexContainer<V> for DijkstraContainer<V> {
  fn new() -> DijkstraContainer<V> {
    BinaryHeap::new()
  }

  fn pop(&mut self) -> Option<V> {
    self.pop().map(|x| x.0)
  }

  fn push(&mut self, vertex: V) {
    self.push(Reverse(vertex));
  }
}

pub struct VertexIter<'a, G: Graph<V>, V: Vertex, C: VertexContainer<V>> {
  graph: &'a G,
  start: V,
  queue: C,
  predecessor_map: HashMap<V, Option<V>>,
}

impl<'a, G: Graph<V>, V: Vertex, C: VertexContainer<V>> VertexIter<'a, G, V, C> {
  pub(crate) fn new(graph: &'a G, start: V) -> VertexIter<'a, G, V, C> where C: Sized {
    let mut container = C::new();
    container.push(start.clone());

    VertexIter {
      graph,
      start: start.clone(),
      queue: container,
      predecessor_map: iter::once((start, None)).collect()
    }
  }

  pub fn construct_path(mut self, target: V) -> Option<Vec<V>> {
    if !self.predecessor_map.contains_key(&target) {
      self.find(|v| v == &target);
    }

    let mut path = vec![target];

    while let Some(Some(previous)) = self.predecessor_map.remove(path.last().unwrap()) {
      path.push(previous);
    }

    path.reverse();

    if path[0] == self.start {
      Some(path)
    } else {
      None
    }
  }
}

impl<'a, G: Graph<V>, V: Vertex, C: VertexContainer<V>> Iterator for VertexIter<'a, G, V, C> {
  type Item = V;

  fn next(&mut self) -> Option<Self::Item> {
    let vertex = self.queue.pop();

    vertex.map(|vertex| {
      for neighbor in self.graph.get_neighbors(vertex.clone()) {
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
