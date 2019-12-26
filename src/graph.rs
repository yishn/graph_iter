use crate::vertex::Vertex;
use crate::vertex_iter::{VertexIter, VertexIterType};

pub trait Graph<V: Vertex> {
  fn get_neighbors(&self, vertex: V) -> Vec<V>;

  fn bfs<'a>(&'a self, start: V) -> VertexIter<'a, Self, V> where Self: Sized {
    VertexIter::new(self, start, VertexIterType::Bfs)
  }

  fn dfs<'a>(&'a self, start: V) -> VertexIter<'a, Self, V> where Self: Sized {
    VertexIter::new(self, start, VertexIterType::Dfs)
  }
}
