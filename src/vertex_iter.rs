use std::collections::HashMap;
use std::iter;
use crate::graph::Graph;
use crate::vertex::Vertex;
use crate::vertex_container::VertexContainer;

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
