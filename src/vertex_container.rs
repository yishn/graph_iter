use std::cmp::Reverse;
use std::collections::{VecDeque, HashMap, BinaryHeap};

#[derive(Clone)]
pub struct DfsContainer<V>(Vec<V>);

#[derive(Clone)]
pub struct BfsContainer<V>(VecDeque<V>);

#[derive(Clone)]
pub struct DijkstraContainer<V, E> {
  id: usize,
  binary_heap: BinaryHeap<Reverse<(E, usize)>>,
  id_map: HashMap<usize, V>
}

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

impl<V, E: Ord> VertexContainer<(V, E)> for DijkstraContainer<V, E> {
  fn new() -> DijkstraContainer<V, E> {
    DijkstraContainer {
      id: 0,
      binary_heap: BinaryHeap::new(),
      id_map: HashMap::new()
    }
  }

  fn pop(&mut self) -> Option<(V, E)> {
    self.binary_heap.pop().map(|Reverse((edge, id))| {
      let vertex = self.id_map.remove(&id).unwrap();
      (vertex, edge)
    })
  }

  fn push(&mut self, (vertex, edge): (V, E)) {
    let id = self.id;
    self.id += 1;

    self.binary_heap.push(Reverse((edge, id)));
    self.id_map.insert(id, vertex);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Hash, Eq, PartialEq, Debug)]
  struct DijkstraKey(usize);

  #[test]
  fn dijkstra_container() {
    let mut container = DijkstraContainer::new();

    container.push((DijkstraKey(1), 5));
    container.push((DijkstraKey(2), 8));
    container.push((DijkstraKey(3), 2));
    container.push((DijkstraKey(4), 1));
    container.push((DijkstraKey(5), 10));

    assert_eq!(container.pop(), Some((DijkstraKey(4), 1)));
    assert_eq!(container.pop(), Some((DijkstraKey(3), 2)));
    assert_eq!(container.pop(), Some((DijkstraKey(1), 5)));

    container.push((DijkstraKey(1), 3));
    container.push((DijkstraKey(6), 9));

    assert_eq!(container.pop(), Some((DijkstraKey(1), 3)));
    assert_eq!(container.pop(), Some((DijkstraKey(2), 8)));
    assert_eq!(container.pop(), Some((DijkstraKey(6), 9)));
    assert_eq!(container.pop(), Some((DijkstraKey(5), 10)));
    assert_eq!(container.pop(), None);
  }
}
