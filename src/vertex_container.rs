use std::cmp::Reverse;
use std::collections::{VecDeque, BinaryHeap};

pub struct DfsContainer<V>(Vec<V>);
pub struct BfsContainer<V>(VecDeque<V>);
pub struct DijkstraContainer<V>(BinaryHeap<Reverse<V>>);

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

impl<V: Ord> VertexContainer<V> for DijkstraContainer<V> {
  fn new() -> DijkstraContainer<V> {
    DijkstraContainer(BinaryHeap::new())
  }

  fn pop(&mut self) -> Option<V> {
    self.0.pop().map(|x| x.0)
  }

  fn push(&mut self, vertex: V) {
    self.0.push(Reverse(vertex));
  }
}
