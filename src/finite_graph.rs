use crate::*;
use std::collections::HashMap;
use std::hash::Hash;
use graph::Graph;
use edge::Edge;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Id(usize);

impl Id {
  fn next(&mut self) {
    self.0 += 1;
  }
}

#[derive(Clone)]
pub struct FiniteGraph<V, E> {
  id: Id,
  vertices_map: HashMap<Id, V>,
  edges_map: HashMap<Id, E>,
  neighbors_map: HashMap<Id, Vec<(Id, Id)>>
}

impl<V, E> FiniteGraph<V, E> {
  pub fn new() -> FiniteGraph<V, E> {
    FiniteGraph {
      id: Id(0),
      vertices_map: HashMap::new(),
      edges_map: HashMap::new(),
      neighbors_map: HashMap::new()
    }
  }

  pub fn with_capacity(vertex_capacity: usize, edge_capacity: usize) -> FiniteGraph<V, E> {
    FiniteGraph {
      id: Id(0),
      vertices_map: HashMap::with_capacity(vertex_capacity),
      edges_map: HashMap::with_capacity(edge_capacity),
      neighbors_map: HashMap::with_capacity(vertex_capacity)
    }
  }

  pub fn len(&self) -> (usize, usize) {
    (self.vertices_map.len(), self.edges_map.len())
  }

  pub fn capacity(&self) -> (usize, usize) {
    (self.vertices_map.capacity(), self.edges_map.capacity())
  }

  pub fn vertices(&self) -> impl Iterator<Item = &V> {
    self.vertices_map.values()
  }

  pub fn vertices_mut(&mut self) -> impl Iterator<Item = &mut V> {
    self.vertices_map.values_mut()
  }

  pub fn edges(&self) -> impl Iterator<Item = &E> {
    self.edges_map.values()
  }

  pub fn edges_mut(&mut self) -> impl Iterator<Item = &mut E> {
    self.edges_map.values_mut()
  }

  pub fn get(&self, vertex: Id) -> Option<&V> {
    self.vertices_map.get(&vertex)
  }

  pub fn get_mut(&mut self, vertex: Id) -> Option<&mut V> {
    self.vertices_map.get_mut(&vertex)
  }

  pub fn get_edge(&self, edge: Id) -> Option<&E> {
    self.edges_map.get(&edge)
  }

  pub fn get_edge_mut(&mut self, edge: Id) -> Option<&mut E> {
    self.edges_map.get_mut(&edge)
  }

  pub fn insert_vertex(&mut self, data: V) -> Id {
    let id = self.id;

    self.id.next();
    self.vertices_map.insert(id, data);

    id
  }

  pub fn remove_vertex(&mut self, vertex: Id) -> Option<V> {
    let result = self.vertices_map.remove_entry(&vertex)
      .map(|(_, data)| data);

    let neighbors = self.neighbors_map.remove_entry(&vertex)
      .map(|(_, neighbors)| neighbors)
      .unwrap_or_else(|| vec![]);

    for (neighbor, _) in neighbors {
      if let Some(neighbors) = self.neighbors_map.get_mut(&neighbor) {
        let index = neighbors.iter().position(|(x, _)| x == &vertex);

        if let Some(index) = index {
          neighbors.remove(index);
        }
      }
    }

    result
  }

  pub fn insert_edge(&mut self, start: Id, end: Id, data: E) -> Option<Id> {
    if !self.vertices_map.contains_key(&start) || !self.vertices_map.contains_key(&end) {
      return None;
    }

    let id = self.id;

    self.id.next();
    self.edges_map.insert(id, data);

    if let Some(neighbors) = self.neighbors_map.get_mut(&start) {
      neighbors.push((end, id));
    } else {
      self.neighbors_map.insert(start, vec![(end, id)]);
    }

    Some(id)
  }

  pub fn insert_bi_edge(&mut self, vertex: Id, other: Id, data: E) -> Option<(Id, Id)>
  where E: Clone {
    let edge1 = self.insert_edge(vertex, other, data.clone());
    let mut edge2 = None;

    if let Some(_) = &edge1 {
      edge2 = self.insert_edge(other, vertex, data);
    }

    edge1.and_then(|e1| edge2.map(|e2| (e1, e2)))
  }
}

impl<V, E> Graph<Id> for FiniteGraph<V, E> {
  type NeighborsIterator = Vec<Id>;

  fn neighbors(&self, vertex: &Id) -> Vec<Id> {
    self.neighbors_map.get(&vertex)
    .map(|neighbors| {
      neighbors.iter()
      .map(|(v, _)| *v)
      .collect()
    })
    .unwrap_or_else(|| vec![])
  }
}

impl<V, E: Edge> EdgedGraph<Id, E> for FiniteGraph<V, E> {
  type EdgesIterator = Vec<E>;

  fn edges(&self, vertex: &Id, other: &Id) -> Vec<E> {
    self.neighbors_map.get(&vertex)
    .map(|neighbors| {
      neighbors.iter()
      .filter(|&(v, _)| v == other)
      .filter_map(|&(_, e)| self.get_edge(e).cloned())
      .collect()
    })
    .unwrap_or_else(|| vec![])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  type Position = (i32, i32);

  #[test]
  fn insert_vertices_and_edges() {
    let mut graph = FiniteGraph::<Position, ()>::new();

    let a = graph.insert_vertex((0, 0));
    let b = graph.insert_vertex((0, 1));
    let c = graph.insert_vertex((1, 1));
    let d = graph.insert_vertex((1, 0));

    graph.insert_edge(a, b, ());
    graph.insert_edge(a, c, ());
    graph.insert_edge(a, d, ());
    graph.insert_edge(b, c, ());
    graph.insert_edge(d, c, ());

    assert_eq!(graph.vertices().count(), 4);
    assert_eq!(graph.neighbors(&a).len(), 3);
    assert_eq!(graph.neighbors(&b).len(), 1);
    assert_eq!(graph.neighbors(&c).len(), 0);
    assert_eq!(graph.neighbors(&d).len(), 1);
  }
}
