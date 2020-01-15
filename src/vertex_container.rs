use std::cmp::Reverse;
use std::collections::{VecDeque, HashMap, BinaryHeap};

pub trait VertexContainer<V> {
  fn new() -> Self;
  fn pop(&mut self) -> Option<V>;
  fn peek(&self) -> Option<&V>;
  fn push(&mut self, value: V);
}

#[derive(Clone)]
pub struct DfsContainer<V>(Vec<V>);

impl<V> VertexContainer<V> for DfsContainer<V> {
  fn new() -> DfsContainer<V> {
    DfsContainer(Vec::new())
  }

  fn pop(&mut self) -> Option<V> {
    self.0.pop()
  }

  fn peek(&self) -> Option<&V> {
    self.0.last()
  }

  fn push(&mut self, value: V) {
    self.0.push(value);
  }
}

#[derive(Clone)]
pub struct BfsContainer<V>(VecDeque<V>);

impl<V> VertexContainer<V> for BfsContainer<V> {
  fn new() -> BfsContainer<V> {
    BfsContainer(VecDeque::new())
  }

  fn pop(&mut self) -> Option<V> {
    self.0.pop_front()
  }

  fn peek(&self) -> Option<&V> {
    self.0.front()
  }

  fn push(&mut self, value: V) {
    self.0.push_back(value);
  }
}

#[derive(Clone)]
pub struct AstarContainer<V, C> {
  id: usize,
  binary_heap: BinaryHeap<Reverse<(C, usize)>>,
  id_map: HashMap<usize, V>
}

impl<V, C: Ord> AstarContainer<V, C> {
  pub fn new() -> AstarContainer<V, C> {
    AstarContainer {
      id: 0,
      binary_heap: BinaryHeap::new(),
      id_map: HashMap::new()
    }
  }

  pub fn pop(&mut self) -> Option<(V, C)> {
    self.binary_heap.pop().map(|Reverse((cost, id))| {
      let value = self.id_map.remove(&id).unwrap();
      (value, cost)
    })
  }

  pub fn peek(&self) -> Option<(&V, &C)> {
    self.binary_heap.peek().and_then(|Reverse((cost, id))| {
      self.id_map.get(id).map(|value| {
        (value, cost)
      })
    })
  }

  pub fn push(&mut self, value: V, cost: C) {
    let id = self.id;
    self.id += 1;

    self.binary_heap.push(Reverse((cost, id)));
    self.id_map.insert(id, value);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Hash, Eq, PartialEq, Debug)]
  struct AstarKey(usize);

  #[test]
  fn astar_container() {
    let mut container = AstarContainer::new();

    container.push(AstarKey(1), 5);
    container.push(AstarKey(2), 8);
    container.push(AstarKey(3), 2);
    container.push(AstarKey(4), 1);
    container.push(AstarKey(5), 10);

    assert_eq!(container.pop(), Some((AstarKey(4), 1)));
    assert_eq!(container.pop(), Some((AstarKey(3), 2)));
    assert_eq!(container.pop(), Some((AstarKey(1), 5)));

    container.push(AstarKey(1), 3);
    container.push(AstarKey(6), 9);

    assert_eq!(container.pop(), Some((AstarKey(1), 3)));
    assert_eq!(container.pop(), Some((AstarKey(2), 8)));
    assert_eq!(container.pop(), Some((AstarKey(6), 9)));
    assert_eq!(container.pop(), Some((AstarKey(5), 10)));
    assert_eq!(container.pop(), None);
  }
}
