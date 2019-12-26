use crate::*;
use vertex::Vertex;
use edge::{Edge, WeightedEdge};
use vertex_container::{DfsContainer, BfsContainer};
use vertex_iterator::{DefaultVertexIter, DijkstraVertexIter};

/// Represents a directed graph.
pub trait Graph<V: Vertex, E: Edge = ()> {
  /// Generates a list of vertices that can be reached from `vertex` by traveling along an edge.
  fn get_neighbors(&self, vertex: V) -> Vec<V>;

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
}

/// Represents a directed multigraph, where edges contains certain data.
///
/// This trait will automatically implement [`Graph`](./trait.Graph.html), so
/// all functions in [`Graph`](./trait.Graph.html) will be available here as well.
pub trait EdgedGraph<V: Vertex, E: Edge> {
  /// Generates a list of vertices and their edges that are adjacent to `vertex`.
  fn get_neighbors_with_edges(&self, vertex: V) -> Vec<(V, E)>;

  /// Returns a [`VertexIterator`](./trait.VertexIterator.html) that iterates the graph vertices
  /// in a smallest-weight-sum-first manner.
  fn dijkstra(&self, start: V) -> DijkstraVertexIter<'_, Self, V, E>
  where E: WeightedEdge, Self: Sized {
    DijkstraVertexIter::new(self, start)
  }
}

impl<V: Vertex, E: Edge, T: EdgedGraph<V, E>> Graph<V, E> for T {
  fn get_neighbors(&self, vertex: V) -> Vec<V> {
    self.get_neighbors_with_edges(vertex).into_iter()
    .map(|(v, _)| v)
    .collect()
  }
}
