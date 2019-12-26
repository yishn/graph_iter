use crate::*;
use vertex::Vertex;
use edge::{Edge, WeightedEdge};
use vertex_container::{DfsContainer, BfsContainer, DijkstraContainer};
use vertex_iterator::{DefaultVertexIter, DijkstraVertexIter};

/// Represents a directed graph. All implementations will
/// implement [`EdgedGraph`](./trait.EdgedGraph.html) automatically,
/// so all provided functions for [`EdgedGraph`](./trait.EdgedGraph.html)
/// will be available for `Graph` as well.
pub trait Graph<V: Vertex>: EdgedGraph<V, ()> {
  /// Generates a list of vertices that can be reached from `vertex`.
  fn get_neighbors(&self, vertex: V) -> Vec<V>;
}

/// Represents a directed multigraph, where edges contains certain data.
pub trait EdgedGraph<V: Vertex, E: Edge> {
  /// Generates a list of vertices and their edges that can be reached from `vertex`.
  fn get_neighbors_with_edges(&self, vertex: V) -> Vec<(V, E)>;

  /// Returns a [`VertexIterator`](./trait.VertexIterator.html) that iterates the graph vertices
  /// in a breadth-first manner.
  fn bfs(&self, start: V) -> DefaultVertexIter<'_, Self, V, E, DfsContainer<V>>
  where Self: Sized {
    DefaultVertexIter::new(self, start)
  }

  /// Returns a [`VertexIterator`](./trait.VertexIterator.html) that iterates the graph vertices
  /// in a depth-first manner.
  fn dfs(&self, start: V) -> DefaultVertexIter<'_, Self, V, E, BfsContainer<V>>
  where Self: Sized {
    DefaultVertexIter::new(self, start)
  }

  /// Returns a [`VertexIterator`](./trait.VertexIterator.html) that iterates the graph vertices
  /// in a depth-first manner.
  fn dijkstra(&self, start: V) -> DijkstraVertexIter<'_, Self, V, E>
  where E: WeightedEdge, Self: Sized {
    DijkstraVertexIter::new(self, start)
  }
}

impl<V: Vertex, G: Graph<V>> EdgedGraph<V, ()> for G {
  fn get_neighbors_with_edges(&self, vertex: V) -> Vec<(V, ())> {
    self.get_neighbors(vertex).into_iter()
    .map(|v| (v, ()))
    .collect()
  }
}
