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
  neighbors_map: HashMap<Id, Vec<(Id, Id)>>,
  reverse_neighbors_map: HashMap<Id, Vec<(Id, Id)>>
}

impl<V, E> FiniteGraph<V, E> {
  /// Constructs a new, empty `FiniteGraph<V, E>`.
  pub fn new() -> FiniteGraph<V, E> {
    FiniteGraph {
      id: Id(0),
      vertices_map: HashMap::new(),
      edges_map: HashMap::new(),
      neighbors_map: HashMap::new(),
      reverse_neighbors_map: HashMap::new()
    }
  }

  /// Constructs a new, empty `FiniteGraph<V, E>` with the specified vertex/edge capacities.
  pub fn with_capacity(vertex_capacity: usize, edge_capacity: usize) -> FiniteGraph<V, E> {
    FiniteGraph {
      id: Id(0),
      vertices_map: HashMap::with_capacity(vertex_capacity),
      edges_map: HashMap::with_capacity(edge_capacity),
      neighbors_map: HashMap::with_capacity(vertex_capacity),
      reverse_neighbors_map: HashMap::with_capacity(vertex_capacity)
    }
  }

  /// Returns the number of vertices and the number of edges.
  pub fn len(&self) -> (usize, usize) {
    (self.vertices_map.len(), self.edges_map.len())
  }

  /// Returns the number of vertices and the number of edges the collection can
  /// hold without reallocating.
  pub fn capacity(&self) -> (usize, usize) {
    (self.vertices_map.capacity(), self.edges_map.capacity())
  }

  /// An iterator visiting all vertices in arbitrary order.
  pub fn all_vertices(&self) -> impl Iterator<Item = (Id, &V)> {
    self.vertices_map.iter().map(|(id, v)| (*id, v))
  }

  /// An iterator visiting all vertices mutably in arbitrary order.
  pub fn all_vertices_mut(&mut self) -> impl Iterator<Item = (Id, &mut V)> {
    self.vertices_map.iter_mut().map(|(id, v)| (*id, v))
  }

  /// An iterator visiting all edges in arbitrary order.
  pub fn all_edges(&self) -> impl Iterator<Item = (Id, &E)> {
    self.edges_map.iter().map(|(id, (e, _, _))| (*id, e))
  }

  /// An iterator visiting all edges mutably in arbitrary order.
  pub fn all_edges_mut(&mut self) -> impl Iterator<Item = (Id, &mut E)> {
    self.edges_map.iter_mut().map(|(id, (e, _, _))| (*id, e))
  }

  /// Returns a reference to the value corresponding to the vertex.
  pub fn get_vertex(&self, vertex: Id) -> Option<&V> {
    self.vertices_map.get(&vertex)
  }

  /// Returns a mutable reference to the value corresponding to the vertex.
  pub fn get_vertex_mut(&mut self, vertex: Id) -> Option<&mut V> {
    self.vertices_map.get_mut(&vertex)
  }

  /// Returns a reference to the value corresponding to the edge.
  pub fn get_edge(&self, edge: Id) -> Option<&E> {
    self.edges_map.get(&edge).map(|(e, _, _)| e)
  }

  /// Returns a mutable reference to the value corresponding to the edge.
  pub fn get_edge_mut(&mut self, edge: Id) -> Option<&mut E> {
    self.edges_map.get_mut(&edge).map(|(e, _, _)| e)
  }

  /// Returns `true` if the graph contains a value for the specified vertex.
  pub fn contains_vertex(&self, vertex: Id) -> bool {
    self.vertices_map.contains_key(&vertex)
  }

  /// Returns `true` if the graph contains a value for the specified edge.
  pub fn contains_edge(&self, edge: Id) -> bool {
    self.edges_map.contains_key(&edge)
  }

  pub fn insert_vertex(&mut self, value: V) -> Id {
    let id = self.id.next();
    self.vertices_map.insert(id, value);

    id
  }

  pub fn remove_vertex(&mut self, vertex: Id) -> Option<V> {
    let result = self.vertices_map.remove(&vertex);
    let neighbors = self.neighbors_map.remove(&vertex).unwrap_or_else(|| vec![]);
    let reverse_neighbors = self.reverse_neighbors_map.remove(&vertex).unwrap_or_else(|| vec![]);

    let to_delete = neighbors.into_iter()
      .map(|(_, e)| e)
      .chain(
        reverse_neighbors.into_iter()
        .map(|(_, e)| e)
      );

    for edge in to_delete {
      self.remove_edge(edge);
    }

    result
  }

  fn insert_edge_id(&mut self, from: Id, to: Id, edge: Id) -> Option<Id> {
    if let Some(neighbors) = self.neighbors_map.get_mut(&from) {
      neighbors.push((to, edge));
    } else {
      self.neighbors_map.insert(from, vec![(to, edge)]);
    }

    if let Some(reverse_neighbors) = self.reverse_neighbors_map.get_mut(&to) {
      reverse_neighbors.push((from, edge));
    } else {
      self.reverse_neighbors_map.insert(to, vec![(from, edge)]);
    }

    Some(edge)
  }

  pub fn insert_edge(&mut self, from: Id, to: Id, value: E) -> Option<Id> {
    if !self.vertices_map.contains_key(&from) || !self.vertices_map.contains_key(&to) {
      return None;
    }

    let id = self.id.next();
    self.edges_map.insert(id, (value, from, to));

    self.insert_edge_id(from, to, id)
  }

  pub fn insert_bi_edge(&mut self, from: Id, to: Id, data: E) -> Option<Id> {
    let edge = self.insert_edge(from, to, data);

    if let &Some(id) = &edge {
      self.insert_edge_id(to, from, id);
    }

    edge
  }

  pub fn remove_edge(&mut self, edge: Id) -> Option<E> {
    self.edges_map.remove(&edge).map(|(data, vertex, other)| {
      for vertex in &[vertex, other] {
        for map in &mut [&mut self.neighbors_map, &mut self.reverse_neighbors_map] {
          map.get_mut(&vertex).map(|neighbors| {
            neighbors.iter().position(|(_, e)| e == &edge).map(|index| {
              neighbors.remove(index);
            });
          });
        }
      }

      data
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

impl<V, E> ReversableGraph<Id> for FiniteGraph<V, E> {
  type ReverseNeighborsIterator = Vec<Id>;

  fn reverse_neighbors(&self, vertex: &Id) -> Vec<Id> {
    self.reverse_neighbors_map.get(&vertex)
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
  fn insert_and_remove_vertices_and_edges() {
    let mut graph = FiniteGraph::<Position, usize>::new();

    let a = graph.insert_vertex((0, 0));
    let b = graph.insert_vertex((0, 1));
    let c = graph.insert_vertex((1, 1));
    let d = graph.insert_vertex((1, 0));

    let e1 = graph.insert_edge(a, b, 1).unwrap();
    let e2 = graph.insert_edge(a, c, 2).unwrap();
    let e3 = graph.insert_edge(a, d, 3).unwrap();
    let e4 = graph.insert_edge(b, c, 4).unwrap();
    let e5 = graph.insert_edge(d, c, 5).unwrap();

    assert_eq!(graph.len(), (4, 5));
    assert_eq!(graph.all_vertices().count(), 4);
    assert_eq!(graph.all_edges().count(), 5);

    assert_eq!(graph.get_vertex(a).unwrap(), &(0, 0));
    assert_eq!(graph.get_vertex(b).unwrap(), &(0, 1));
    assert_eq!(graph.get_vertex(c).unwrap(), &(1, 1));
    assert_eq!(graph.get_vertex(d).unwrap(), &(1, 0));

    assert_eq!(graph.get_edge(e1), Some(&1));
    assert_eq!(graph.get_edge(e2), Some(&2));
    assert_eq!(graph.get_edge(e3), Some(&3));
    assert_eq!(graph.get_edge(e4), Some(&4));
    assert_eq!(graph.get_edge(e5), Some(&5));

    assert_eq!(graph.neighbors(&a), vec![b, c, d]);
    assert_eq!(graph.neighbors(&b), vec![c]);
    assert_eq!(graph.neighbors(&c), vec![]);
    assert_eq!(graph.neighbors(&d), vec![c]);

    let edge_data = graph.remove_edge(e3);

    assert_eq!(edge_data, Some(3));
    assert_eq!(graph.get_edge(e3), None);
    assert_eq!(graph.all_vertices().count(), 4);
    assert_eq!(graph.all_edges().count(), 4);
    assert_eq!(graph.neighbors(&a), vec![b, c]);
    assert_eq!(graph.neighbors(&d), vec![c]);

    let vertex_data = graph.remove_vertex(b);

    assert_eq!(vertex_data, Some((0, 1)));
    assert_eq!(graph.get_vertex(b), None);
    assert_eq!(graph.all_vertices().count(), 3);
    assert_eq!(graph.all_edges().count(), 2);
    assert_eq!(graph.neighbors(&a), vec![c]);
  }

  #[test]
  fn insert_and_remove_bi_edges() {
    let mut graph = FiniteGraph::<Position, usize>::new();

    let a = graph.insert_vertex((0, 0));
    let b = graph.insert_vertex((0, 1));
    let c = graph.insert_vertex((1, 1));
    let d = graph.insert_vertex((1, 0));

    let e1 = graph.insert_bi_edge(a, b, 1).unwrap();
    graph.insert_bi_edge(b, c, 2).unwrap();
    graph.insert_bi_edge(c, d, 3).unwrap();
    graph.insert_bi_edge(d, a, 4).unwrap();

    assert_eq!(graph.all_edges().count(), 4);
    assert_eq!(graph.edges(&a, &b), vec![1]);
    assert_eq!(graph.edges(&b, &a), vec![1]);
    assert_eq!(graph.edges(&a, &d), vec![4]);
    assert_eq!(graph.edges(&d, &a), vec![4]);
    assert_eq!(graph.edges(&b, &d), vec![]);
    assert_eq!(graph.edges(&d, &b), vec![]);

    graph.remove_edge(e1);

    assert_eq!(graph.edges(&a, &b), vec![]);
    assert_eq!(graph.edges(&b, &a), vec![]);
  }
}
