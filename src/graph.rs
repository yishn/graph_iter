use crate::vertex::Vertex;
use crate::vertex_container::{DfsContainer, BfsContainer, DijkstraContainer};
use crate::vertex_iter::VertexIter;

pub trait Graph<V: Vertex> {
  fn get_neighbors(&self, vertex: V) -> Vec<V>;

  fn bfs<'a>(&'a self, start: V) -> VertexIter<'a, Self, V, DfsContainer<V>>
  where Self: Sized {
    VertexIter::new(self, start)
  }

  fn dfs<'a>(&'a self, start: V) -> VertexIter<'a, Self, V, BfsContainer<V>>
  where Self: Sized {
    VertexIter::new(self, start)
  }
}
