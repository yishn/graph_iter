use crate::*;
use std::cmp::Reverse;
use std::collections::{VecDeque, HashMap, BinaryHeap};
use vertex::Vertex;

#[derive(Clone)]
pub struct DfsContainer<V>(Vec<V>);

#[derive(Clone)]
pub struct BfsContainer<V>(VecDeque<V>);

#[derive(Clone)]
pub struct DijkstraContainer<V, E>(
  BinaryHeap<Reverse<(E, usize)>>,
  HashMap<usize, V>
);

pub trait VertexContainer<V> {
  fn new() -> Self;
  fn pop(&mut self) -> Option<V>;
  fn push(&mut self, vertex: V);
}

impl<V> VertexContainer<V> for DfsContainer<V> {
  fn new() -> DfsContainer<V> {
    DfsContainer(Vec::new())
  }

  fn pop(&mut self) -> Option<V> {
    self.0.pop()
  }

  fn push(&mut self, vertex: V) {
    self.0.push(vertex);
  }
}

impl<V> VertexContainer<V> for BfsContainer<V> {
  fn new() -> BfsContainer<V> {
    BfsContainer(VecDeque::new())
  }

  fn pop(&mut self) -> Option<V> {
    self.0.pop_front()
  }

  fn push(&mut self, vertex: V) {
    self.0.push_back(vertex);
  }
}

impl<V: Vertex, E: Ord> VertexContainer<(V, E)> for DijkstraContainer<V, E> {
  fn new() -> DijkstraContainer<V, E> {
    DijkstraContainer(BinaryHeap::new(), HashMap::new())
  }

  fn pop(&mut self) -> Option<(V, E)> {
    self.0.pop().map(|Reverse((edge, id))| {
      let vertex = self.1.remove(&id).unwrap();
      (vertex, edge)
    })
  }

  fn push(&mut self, (vertex, edge): (V, E)) {
    let id = self.1.len();

    self.0.push(Reverse((edge, id)));
    self.1.insert(id, vertex);
  }
}
