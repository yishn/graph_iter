use crate::*;
use vertex::Vertex;
use edge::{Edge, WeightedEdge};
use vertex_container::{DfsContainer, BfsContainer};
use vertex_traverser::{DefaultVertexTrav, DijkstraVertexTrav};

/// Represents a directed, potentially infinite, graph.
///
/// `Graph<V>` is a trait and is parameterized over `V`, the type of your vertices.
/// `V` needs to implement [`Hash`], [`Eq`], and [`Clone`] traits. Vertex traversals
/// need to clone vertices fairly often, so consider wrapping your vertices in an
/// [`Rc`] pointer if cloning vertices takes a lot of effort.
///
/// Creating a graph involves two steps: Creating a `struct` to hold the graph data
/// and then implementing `Graph<V>` for that `struct`, i.e. implementing the required
/// trait function `neighbors` that will return a [`Vec`] of adjacent vertices of the
/// given vertex as function argument.
///
/// [`Hash`]: https://doc.rust-lang.org/core/hash/trait.Hash.html
/// [`Eq`]: https://doc.rust-lang.org/core/cmp/trait.Eq.html
/// [`Clone`]: https://doc.rust-lang.org/core/clone/trait.Clone.html
/// [`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
/// [`Vec`]: https://doc.rust-lang.org/alloc/vec/struct.Vec.html
///
/// # Example
///
/// ```
/// use graph_iter::{Graph, VertexTraverser};
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
///   type NeighborsIterator = Vec<Position>;
///
///   fn neighbors(&self, &(x, y): &Position) -> Vec<Position> {
///     [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
///     .filter(|&v| !self.blocked.contains(v))
///     .cloned()
///     .collect()
///   }
/// }
///
/// let graph = LatticeGraph {
///   blocked: vec![(0, 1)]
/// };
///
/// let path = graph.bfs(&(0, 0)).construct_path(&(0, 5)).unwrap();
///
/// assert_eq!(path[0], (0, 0));
/// assert_eq!(path.len(), 8);
/// assert_eq!(path[7], (0, 5));
/// ```
pub trait Graph<V: Vertex> {
  type NeighborsIterator: IntoIterator<Item = V>;

  /// Generates a list of adjacent vertices that can be reached from `vertex` by traveling along an edge.
  fn neighbors(&self, vertex: &V) -> Self::NeighborsIterator;

  /// Returns a [`VertexTraverser`](./trait.VertexTraverser.html) that iterates the graph vertices
  /// in a breadth-first manner.
  fn bfs(&self, start: &V) -> DefaultVertexTrav<'_, Self, V, BfsContainer<V>>
  where Self: Sized {
    DefaultVertexTrav::new(self, start.clone())
  }

  /// Returns a [`VertexTraverser`](./trait.VertexTraverser.html) that iterates the graph vertices
  /// in a depth-first manner.
  fn dfs(&self, start: &V) -> DefaultVertexTrav<'_, Self, V, DfsContainer<V>>
  where Self: Sized {
    DefaultVertexTrav::new(self, start.clone())
  }
}

/// Represents a directed, potentially infinite, multigraph, where edges contain certain data.
///
/// `EdgedGraph<V, E>` is a trait and is parameterized over `V`, the type of your vertices,
/// and `E`, the type of your edges. `E` needs to implement the [`Clone`] trait. Vertex
/// traversals need to clone edges fairly often, so consider wrapping your edges in an
/// [`Rc`] pointer if cloning vertices takes a lot of effort.
///
/// This trait requires your `struct` to implement [`Graph<V>`] as well.
///
/// [`Clone`]: https://doc.rust-lang.org/core/clone/trait.Clone.html
/// [`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
/// [`Graph<V>`]: ./trait.Graph.html
///
/// # Example
///
/// ```
/// use graph_iter::{Graph, EdgedGraph, VertexTraverser};
///
/// type Position = (i32, i32);
///
/// /// Implements a fully connected graph on the given vertices
/// /// that are points on an integer-based 2d grid.
/// struct FullyConnectedGraph {
///   vertices: Vec<Position>
/// }
///
/// impl Graph<Position> for FullyConnectedGraph {
///   type NeighborsIterator = Vec<Position>;
///
///   fn neighbors(&self, vertex: &Position) -> Vec<Position> {
///     self.vertices.iter()
///     .filter(|&v| v != vertex)
///     .cloned()
///     .collect()
///   }
/// }
///
/// impl EdgedGraph<Position, u32> for FullyConnectedGraph {
///   type EdgesIterator = Vec<u32>;
///
///   fn edges(&self, &(x1, y1): &Position, &(x2, y2): &Position) -> Vec<u32> {
///     vec![(x2 - x1).abs() as u32 + (y2 - y1).pow(2) as u32]
///   }
/// }
///
/// let graph = FullyConnectedGraph {
///   vertices: vec![(0, 0), (0, 10), (2, 5), (4, 7), (10, 0), (10, 10)]
/// };
///
/// let path = graph.dijkstra(&(0, 0)).construct_path(&(10, 10)).unwrap();
///
/// assert_eq!(path, [(0, 0), (2, 5), (4, 7), (10, 10)]);
/// ```
pub trait EdgedGraph<V: Vertex, E: Edge>: Graph<V> {
  type EdgesIterator: IntoIterator<Item = E>;

  /// Generates a list of edges that connect `vertex` with `other`.
  fn edges(&self, vertex: &V, other: &V) -> Self::EdgesIterator;

  /// Returns a [`VertexTraverser`](./trait.VertexTraverser.html) that iterates the graph vertices
  /// in a smallest-weight-sum-first manner. This function requires your edge type `E` to implement
  /// the [`WeightedEdge`](./trait.WeightedEdge.html) trait, i.e. additionally implement [`Ord`],
  /// [`Add`], and [`Default`].
  ///
  /// [`Ord`]: https://doc.rust-lang.org/core/cmp/trait.Ord.html
  /// [`Add`]: https://doc.rust-lang.org/core/ops/arith/trait.Add.html
  /// [`Default`]: https://doc.rust-lang.org/core/default/trait.Default.html
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

#[cfg(test)]
mod tests {
  use super::*;

  type Position = (i32, i32);

  struct LatticeGraph {
    blocked: Vec<Position>
  }

  impl Graph<Position> for LatticeGraph {
    type NeighborsIterator = Vec<Position>;

    fn neighbors(&self, &(x, y): &Position) -> Vec<Position> {
      [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
      .cloned()
      .filter(|v| !self.blocked.contains(v))
      .collect()
    }
  }

  struct FullyConnectedGraph {
    vertices: Vec<Position>
  }

  impl Graph<Position> for FullyConnectedGraph {
    type NeighborsIterator = Vec<Position>;

    fn neighbors(&self, vertex: &Position) -> Vec<Position> {
      self.vertices.iter()
      .filter(|&v| v != vertex)
      .cloned()
      .collect()
    }
  }

  impl EdgedGraph<Position, u32> for FullyConnectedGraph {
    type EdgesIterator = Vec<u32>;

    fn edges(&self, &(x1, y1): &Position, &(x2, y2): &Position) -> Vec<u32> {
      vec![(x2 - x1).abs() as u32 + (y2 - y1).pow(2) as u32]
    }
  }

  #[test]
  fn test_simple_lattice_graph() {
    let graph = LatticeGraph {
      blocked: vec![(0, 1)]
    };

    let mut bfs_traverser = graph.bfs(&(0, 0));
    let path = bfs_traverser.construct_path(&(0, 5)).unwrap();

    assert_eq!(path[0], (0, 0));
    assert_eq!(path.len(), 8);
    assert_eq!(path[7], (0, 5));
  }

  #[test]
  fn test_dijkstra_algorithm() {
    let graph = FullyConnectedGraph {
      vertices: vec![(0, 0), (0, 10), (2, 5), (4, 7), (10, 0), (10, 10)]
    };

    let mut dijkstra_traverser = graph.dijkstra(&(0, 0));
    let path = dijkstra_traverser.construct_path(&(10, 10)).unwrap();

    assert_eq!(path, [(0, 0), (2, 5), (4, 7), (10, 10)]);
  }
}
