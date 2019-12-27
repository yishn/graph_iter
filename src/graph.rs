use crate::*;
use vertex::Vertex;
use edge::{Edge, WeightedEdge};
use vertex_container::{DfsContainer, BfsContainer};
use vertex_traverser::{DefaultVertexTrav, DijkstraVertexTrav};

/// Represents a directed, potentially infinite, graph.
pub trait Graph<V: Vertex> {
  /// Generates a list of vertices that can be reached from `vertex` by traveling along an edge.
  fn get_neighbors(&self, vertex: &V) -> Vec<V>;

  /// Returns a [`VertexTraverser`](./trait.VertexTraverser.html) that iterates the graph vertices
  /// in a breadth-first manner.
  fn bfs(&self, start: &V) -> DefaultVertexTrav<'_, Self, V, DfsContainer<V>>
  where Self: Sized {
    DefaultVertexTrav::new(self, start.clone())
  }

  /// Returns a [`VertexTraverser`](./trait.VertexTraverser.html) that iterates the graph vertices
  /// in a depth-first manner.
  fn dfs(&self, start: &V) -> DefaultVertexTrav<'_, Self, V, BfsContainer<V>>
  where Self: Sized {
    DefaultVertexTrav::new(self, start.clone())
  }
}

/// Represents a directed, potentially infinite, multigraph, where edges contain certain data.
pub trait EdgedGraph<V: Vertex, E: Edge>: Graph<V> {
  /// Generates a list of edges that connect `vertex` with `other`.
  fn get_edges(&self, vertex: &V, other: &V) -> Vec<E>;

  /// Returns a [`VertexTraverser`](./trait.VertexTraverser.html) that iterates the graph vertices
  /// in a smallest-weight-sum-first manner.
  ///
  /// Keep in mind that the [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm)
  /// only supports non-negative weights. In terms of our edge type `E` this means the following
  /// conditions should hold
  ///
  /// ```
  /// e1 + e2 >= std::cmp::max(e1, e2)
  /// e1 >= E::default()
  /// ```
  ///
  /// for all `E` types `e1` and `e2`.
  fn dijkstra(&self, start: &V) -> DijkstraVertexTrav<'_, Self, V, E>
  where E: WeightedEdge, Self: Sized {
    DijkstraVertexTrav::new(self, start.clone())
  }
}
