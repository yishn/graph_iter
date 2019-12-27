use crate::*;
use std::collections::HashMap;
use std::iter;
use std::marker::PhantomData;
use std::rc::Rc;
use graph::EdgedGraph;
use vertex::Vertex;
use edge::WeightedEdge;
use vertex_container::{VertexContainer, DijkstraContainer};

pub struct Iter<'a, V: Vertex, T: VertexTraverser<V>>(&'a mut T, PhantomData<V>);

impl<'a, V: Vertex, T: VertexTraverser<V>> Iterator for Iter<'a, V, T> {
  type Item = V;

  fn next(&mut self) -> Option<V> {
    self.0.next()
  }
}

/// An interface for dealing with vertex traversers over a graph.
pub trait VertexTraverser<V: Vertex> {
  /// Returns start vertex.
  fn first(&self) -> V;

  /// Returns the predecessor vertex of the given vertex
  /// or `None` if `vertex` is the start vertex or is not reachable.
  fn predecessor(&mut self, vertex: &V) -> Option<V>;

  /// Advances the traverser and returns the next value.
  fn next(&mut self) -> Option<V>;

  /// Returns an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
  /// that let's you iterate over the traverser.
  fn iter(&mut self) -> Iter<'_, V, Self> where Self: Sized {
    Iter(self, PhantomData)
  }

  /// Returns a path from start vertex to `target` or `None` if
  /// there's no such path.
  fn construct_path(&mut self, target: &V) -> Option<Vec<V>> {
    let mut path = vec![target.clone()];

    while let Some(previous) = self.predecessor(path.last().unwrap()) {
      path.push(previous);
    }

    path.reverse();

    if path[0] == self.first() {
      Some(path)
    } else {
      None
    }
  }
}

#[derive(Clone)]
pub struct DefaultVertexTrav<'a, G, V, C>
where G: Graph<V>, V: Vertex, C: VertexContainer<Rc<V>> {
  graph: &'a G,
  start: Rc<V>,
  queue: C,
  predecessor_map: HashMap<Rc<V>, Option<Rc<V>>>
}

impl<'a, G, V, C> DefaultVertexTrav<'a, G, V, C>
where G: Graph<V>, V: Vertex, C: VertexContainer<Rc<V>> {
  pub(crate) fn new(graph: &G, start: V) -> DefaultVertexTrav<'_, G, V, C> where C: Sized {
    let start = Rc::new(start);
    let mut container = C::new();
    container.push(Rc::clone(&start));

    DefaultVertexTrav {
      graph,
      start: Rc::clone(&start),
      queue: container,
      predecessor_map: iter::once((start, None)).collect()
    }
  }
}

impl<'a, G, V, C> VertexTraverser<V> for DefaultVertexTrav<'a, G, V, C>
where G: Graph<V>, V: Vertex, C: VertexContainer<Rc<V>> {
  fn first(&self) -> V {
    (*self.start).clone()
  }

  fn predecessor(&mut self, vertex: &V) -> Option<V> {
    if !self.predecessor_map.contains_key(vertex) {
      self.iter().find(|v| v == vertex);
    }

    self.predecessor_map.get(vertex)
    .and_then(|predecessor| {
      predecessor.clone().map(|v| (*v).clone())
    })
  }

  fn next(&mut self) -> Option<V> {
    let vertex = self.queue.pop();

    vertex.map(|vertex| {
      for neighbor in self.graph.neighbors(&vertex) {
        if self.predecessor_map.contains_key(&neighbor) {
          continue;
        }

        let neighbor = Rc::new(neighbor);

        self.queue.push(Rc::clone(&neighbor));
        self.predecessor_map.insert(Rc::clone(&neighbor), Some(Rc::clone(&vertex)));
      }

      (*vertex).clone()
    })
  }
}

#[derive(Clone)]
pub struct DijkstraVertexTrav<'a, G, V, E>
where G: EdgedGraph<V, E>, V: Vertex, E: WeightedEdge {
  graph: &'a G,
  start: Rc<V>,
  queue: DijkstraContainer<Rc<V>, Rc<E>>,
  predecessor_map: HashMap<Rc<V>, Option<Rc<V>>>,
  min_edge_map: HashMap<Rc<V>, Rc<E>>
}

impl<'a, G, V, E> DijkstraVertexTrav<'a, G, V, E>
where G: EdgedGraph<V, E>, V: Vertex, E: WeightedEdge {
  pub(crate) fn new(graph: &G, start: V) -> DijkstraVertexTrav<'_, G, V, E> {
    let start = Rc::new(start);
    let mut container = DijkstraContainer::new();
    container.push((Rc::clone(&start), Rc::new(E::default())));

    DijkstraVertexTrav {
      graph,
      start: Rc::clone(&start),
      queue: container,
      predecessor_map: iter::once((Rc::clone(&start), None)).collect(),
      min_edge_map: iter::once((start, Rc::new(E::default()))).collect()
    }
  }
}

impl<'a, G, V, E> VertexTraverser<V> for DijkstraVertexTrav<'a, G, V, E>
where G: EdgedGraph<V, E>, V: Vertex, E: WeightedEdge {
  fn first(&self) -> V {
    (*self.start).clone()
  }

  fn predecessor(&mut self, vertex: &V) -> Option<V> {
    if !self.predecessor_map.contains_key(vertex) {
      self.iter().find(|v| v == vertex);
    }

    self.predecessor_map.get(vertex)
    .and_then(|predecessor| {
      predecessor.clone().map(|v| (*v).clone())
    })
  }

  fn next(&mut self) -> Option<V> {
    let vertex_edge = self.queue.pop();

    vertex_edge.map(|(vertex, edge)| {
      for neighbor in self.graph.neighbors(&vertex) {
        let neighbor = Rc::new(neighbor);
        let outgoing_edge = self.graph
          .edges(&vertex, &neighbor)
          .into_iter()
          .min();

        if let Some(outgoing_edge) = outgoing_edge {
          let new_edge = Rc::new((*edge).clone() + outgoing_edge);
          let mut edge_shorter = false;

          if let Some(min_edge) = self.min_edge_map.get_mut(&neighbor) {
            if &new_edge < min_edge {
              *min_edge = Rc::clone(&new_edge);
              edge_shorter = true;
            }
          } else {
            self.min_edge_map.insert(Rc::clone(&neighbor), Rc::clone(&new_edge));
            edge_shorter = true;
          }

          if edge_shorter {
            self.queue.push((Rc::clone(&neighbor), new_edge));
            self.predecessor_map.insert(neighbor, Some(Rc::clone(&vertex)));
          }
        }
      }

      (*vertex).clone()
    })
  }
}
