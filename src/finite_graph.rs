use crate::*;
use std::collections::HashMap;
use std::hash::Hash;
use graph::Graph;
use edge::Edge;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Id(usize);

impl Id {
  fn next(&mut self) -> Id {
    self.0 += 1;
    *self
  }
}

#[derive(Clone)]
pub struct FiniteGraph<V, E> {
  id: Id,
  vertices_map: HashMap<Id, V>,
  edges_map: HashMap<Id, (E, Id, Id)>,
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
    .map(|(e, _, _)| e)
  }

  pub fn edges_mut(&mut self) -> impl Iterator<Item = &mut E> {
    self.edges_map.values_mut()
    .map(|(e, _, _)| e)
  }

  pub fn get(&self, vertex: Id) -> Option<&V> {
    self.vertices_map.get(&vertex)
  }

  pub fn get_mut(&mut self, vertex: Id) -> Option<&mut V> {
    self.vertices_map.get_mut(&vertex)
  }

  pub fn get_edge(&self, edge: Id) -> Option<&E> {
    self.edges_map.get(&edge).map(|(e, _, _)| e)
  }

  pub fn get_edge_mut(&mut self, edge: Id) -> Option<&mut E> {
    self.edges_map.get_mut(&edge).map(|(e, _, _)| e)
  }

  pub fn insert_vertex(&mut self, data: V) -> Id {
    let id = self.id.next();
    self.vertices_map.insert(id, data);

    id
  }

  pub fn remove_vertex(&mut self, vertex: Id) -> Option<V> {
    let result = self.vertices_map.remove(&vertex);
    let neighbors = self.neighbors_map.remove(&vertex).unwrap_or_else(|| vec![]);

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

  fn insert_edge_id(&mut self, start: Id, end: Id, edge: Id) -> Option<Id> {
    if let Some(neighbors) = self.neighbors_map.get_mut(&start) {
      neighbors.push((end, edge));
    } else {
      self.neighbors_map.insert(start, vec![(end, edge)]);
    }

    Some(edge)
  }

  pub fn insert_edge(&mut self, start: Id, end: Id, data: E) -> Option<Id> {
    if !self.vertices_map.contains_key(&start) || !self.vertices_map.contains_key(&end) {
      return None;
    }

    let id = self.id.next();
    self.edges_map.insert(id, (data, start, end));

    self.insert_edge_id(start, end, id)
  }

  pub fn insert_bi_edge(&mut self, vertex: Id, other: Id, data: E) -> Option<Id> {
    let edge = self.insert_edge(vertex, other, data);

    if let &Some(id) = &edge {
      self.insert_edge_id(other, vertex, id);
    }

    edge
  }

  pub fn remove_edge(&mut self, edge: Id) -> Option<E> {
    self.edges_map.remove(&edge).map(|(result, vertex, other)| {
      for vertex in &[vertex, other] {
        self.neighbors_map.get_mut(&vertex).map(|neighbors| {
          neighbors.iter().position(|(_, e)| e == &edge).map(|index| {
            neighbors.remove(index);
          });
        });
      }

      result
    })
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
