use crate::*;
use vertex::Vertex;
use edge::{Edge, WeightedEdge};
use vertex_container::{DfsContainer, BfsContainer};
use vertex_iterator::{DefaultVertexIter, DijkstraVertexIter};

/// Represents a directed, potentially infinite, graph.
pub trait Graph<V: Vertex> {
  /// Generates a list of vertices that can be reached from `vertex` by traveling along an edge.
  fn get_neighbors(&self, vertex: &V) -> Vec<V>;

  /// Returns a [`VertexIterator`](./trait.VertexIterator.html) that iterates the graph vertices
  /// in a breadth-first manner.
  fn bfs(&self, start: &V) -> DefaultVertexIter<'_, Self, V, DfsContainer<V>>
  where Self: Sized {
    DefaultVertexIter::new(self, start.clone())
  }

  /// Returns a [`VertexIterator`](./trait.VertexIterator.html) that iterates the graph vertices
  /// in a depth-first manner.
  fn dfs(&self, start: &V) -> DefaultVertexIter<'_, Self, V, BfsContainer<V>>
  where Self: Sized {
    DefaultVertexIter::new(self, start.clone())
  }
}

/// Represents a directed, potentially infinite, multigraph, where edges contain certain data.
pub trait EdgedGraph<V: Vertex, E: Edge>: Graph<V> {
  /// Generates a list of edges that connect `vertex` with `other`.
  fn get_edges(&self, vertex: &V, other: &V) -> Vec<E>;

  /// Returns a [`VertexIterator`](./trait.VertexIterator.html) that iterates the graph vertices
  /// in a smallest-weight-sum-first manner.
  fn dijkstra(&self, start: &V) -> DijkstraVertexIter<'_, Self, V, E>
  where E: WeightedEdge, Self: Sized {
    DijkstraVertexIter::new(self, start.clone())
  }
}
