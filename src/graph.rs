use crate::vertex::Vertex;
use crate::vertex_container::{DfsContainer, BfsContainer, DijkstraContainer};
use crate::vertex_iterator::DefaultVertexIter;

pub trait Graph<V: Vertex> {
  fn get_neighbors(&self, vertex: V) -> Vec<V>;

  fn bfs<'a>(&'a self, start: V) -> DefaultVertexIter<'a, Self, V, DfsContainer<V>>
  where Self: Sized {
    DefaultVertexIter::new(self, start)
  }

  fn dfs<'a>(&'a self, start: V) -> DefaultVertexIter<'a, Self, V, BfsContainer<V>>
  where Self: Sized {
    DefaultVertexIter::new(self, start)
  }
}
