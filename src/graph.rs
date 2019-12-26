use crate::vertex::Vertex;
use crate::vertex_iter::{VertexIter, DfsContainer, BfsContainer, DijkstraContainer};

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

  fn dijkstra<'a>(&'a self, start: V) -> VertexIter<'a, Self, V, DijkstraContainer<V>>
  where Self: Sized, V: Ord {
    VertexIter::new(self, start)
  }
}
