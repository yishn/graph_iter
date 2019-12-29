use crate::*;
use std::collections::HashMap;
use std::hash::Hash;
use derive_more::AddAssign;
use graph::Graph;
use edge::Edge;

#[derive(Clone, Copy, Hash, Eq, PartialEq, AddAssign)]
pub struct Id(usize);

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

  pub fn vertices(&self) -> impl Iterator<Item = &V> {
    self.vertices_map.values()
  }

  pub fn edges(&self) -> impl Iterator<Item = &E> {
    self.edges_map.values()
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

    self.id += Id(1);
    self.vertices_map.insert(id, data);

    id
  }

  pub fn remove_vertex(&mut self, vertex: Id) {
    self.vertices_map.remove(&vertex);

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
  }

  pub fn insert_edge(&mut self, start: Id, end: Id, data: E) -> Option<Id> {
    if !self.vertices_map.contains_key(&start) || !self.vertices_map.contains_key(&end) {
      return None;
    }

    let id = self.id;
    self.id += Id(1);

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
  fn neighbors(&self, vertex: &Id) -> Vec<Id> {
    self.neighbors_map.get(&vertex)
    .map(|neighbors| {
      neighbors.iter()
      .map(|(neighbor, _)| *neighbor)
      .collect()
    })
    .unwrap_or_else(|| vec![])
  }
}

impl<V, E: Edge> EdgedGraph<Id, E> for FiniteGraph<V, E> {
  fn edges(&self, vertex: &Id, other: &Id) -> Vec<E> {
    self.neighbors_map.get(vertex)
    .map(|neighbors| {
      neighbors.iter()
      .filter(|(neighbor, _)| neighbor == other)
      .filter_map(|(_, edge)| self.get_edge(*edge).cloned())
      .collect()
    })
    .unwrap_or_else(|| vec![])
  }
}
