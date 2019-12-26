use crate::vertex::Vertex;
use crate::vertex_container::{DfsContainer, BfsContainer, DijkstraContainer};
use crate::vertex_iterator::DefaultVertexIter;

pub trait Graph<V: Vertex> {
  /// Generates a list of vertices that can be reached from `vertex`.
  fn get_neighbors(&self, vertex: V) -> Vec<V>;

  /// Returns a [`VertexIterator`](./trait.VertexIterator.html) that iterates the graph vertices
  /// in a breadth-first manner.
  fn bfs<'a>(&'a self, start: V) -> DefaultVertexIter<'a, Self, V, DfsContainer<V>>
  where Self: Sized {
    DefaultVertexIter::new(self, start)
  }

  /// Returns a [`VertexIterator`](./trait.VertexIterator.html) that iterates the graph vertices
  /// in a depth-first manner.
  fn dfs<'a>(&'a self, start: V) -> DefaultVertexIter<'a, Self, V, BfsContainer<V>>
  where Self: Sized {
    DefaultVertexIter::new(self, start)
  }
}
