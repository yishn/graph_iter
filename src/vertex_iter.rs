use std::collections::{VecDeque, HashMap};
use std::iter;
use crate::graph::Graph;
use crate::vertex::Vertex;

pub(crate) enum VertexIterType {
  Bfs,
  Dfs
}

pub struct VertexIter<'a, G: Graph<V>, V: Vertex> {
  graph: &'a G,
  start: V,
  queue: VecDeque<V>,
  predecessor_map: HashMap<V, Option<V>>,
  iter_type: VertexIterType
}

impl<'a, G: Graph<V>, V: Vertex> VertexIter<'a, G, V> {
  pub(crate) fn new(graph: &'a G, start: V, iter_type: VertexIterType) -> VertexIter<'a, G, V> {
    VertexIter {
      graph,
      start: start.clone(),
      queue: iter::once(start.clone()).collect(),
      predecessor_map: iter::once((start, None)).collect(),
      iter_type
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

impl<'a, G: Graph<V>, V: Vertex> Iterator for VertexIter<'a, G, V> {
  type Item = V;

  fn next(&mut self) -> Option<Self::Item> {
    let vertex = match self.iter_type {
      VertexIterType::Bfs => self.queue.pop_front(),
      VertexIterType::Dfs => self.queue.pop_back()
    };

    vertex.map(|vertex| {
      for neighbor in self.graph.get_neighbors(vertex.clone()) {
        if self.predecessor_map.contains_key(&neighbor) {
          continue;
        }

        self.queue.push_back(neighbor.clone());
        self.predecessor_map.insert(neighbor, Some(vertex.clone()));
      }

      vertex
    })
  }
}
