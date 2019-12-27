use crate::*;
use std::rc::Rc;
use vertex::Vertex;
use edge::{Edge, WeightedEdge};
use vertex_container::{DfsContainer, BfsContainer};
use vertex_traverser::{DefaultVertexTrav, DijkstraVertexTrav};

/// Represents a directed, potentially infinite, graph.
///
/// # Example
///
/// ```
/// use graph_iter::Graph;
///
/// type Position = (i32, i32);
///
/// /// Implements a simple 2d integer-based grid where certain
/// /// positions are blocked off from traversing.
/// struct LatticeGraph {
///   blocked: Vec<Position>
/// }
///
/// impl Graph<Position> for LatticeGraph {
///   fn get_neighbors(&self, &(x, y): &Position) -> Vec<Position> {
///     [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
///     .cloned()
///     .filter(|v| !self.blocked.contains(v))
///     .collect()
///   }
/// }
/// ```
pub trait Graph<V: Vertex> {
  /// Generates a list of vertices that can be reached from `vertex` by traveling along an edge.
  fn get_neighbors(&self, vertex: &V) -> Vec<V>;

  /// Returns a [`VertexTraverser`](./trait.VertexTraverser.html) that iterates the graph vertices
  /// in a breadth-first manner.
  fn bfs(&self, start: &V) -> DefaultVertexTrav<'_, Self, V, DfsContainer<Rc<V>>>
  where Self: Sized {
    DefaultVertexTrav::new(self, start.clone())
  }

  /// Returns a [`VertexTraverser`](./trait.VertexTraverser.html) that iterates the graph vertices
  /// in a depth-first manner.
  fn dfs(&self, start: &V) -> DefaultVertexTrav<'_, Self, V, BfsContainer<Rc<V>>>
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
  /// ```text
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
