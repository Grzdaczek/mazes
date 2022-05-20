use crate::matrix::AdjacencyMatrix;
use core::panic;

#[derive(Debug, Clone, PartialEq)]
enum MatrixCell<T> {
    Empty,
    Edge(T),
}

impl<T> Default for MatrixCell<T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<T> MatrixCell<T> {
    pub fn is_empty(&self) -> bool {
        match self {
            MatrixCell::Empty => true,
            MatrixCell::Edge(_) => false,
        }
    }

    #[inline]
    pub fn is_edge(&self) -> bool {
        !self.is_empty()
    }

    pub fn unwrap(self) -> T {
        match self {
            MatrixCell::Empty => panic!(),
            MatrixCell::Edge(e) => e,
        }
    }

    pub fn unwrap_ref(&self) -> &T {
        match self {
            MatrixCell::Empty => panic!(),
            MatrixCell::Edge(e) => e,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graph<N, E> {
    edges: AdjacencyMatrix<MatrixCell<E>>,
    nodes: Vec<N>,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            edges: AdjacencyMatrix::new(),
            nodes: Vec::new(),
        }
    }

    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Self {
            edges: AdjacencyMatrix::with_capacity(edges),
            nodes: Vec::with_capacity(nodes),
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges
            .iter()
            .filter(|(_, _, e)| match e {
                MatrixCell::Empty => false,
                MatrixCell::Edge(_) => true,
            })
            .count()
    }

    pub fn set_node(&mut self, index: usize, value: N) {
        self.nodes[index] = value;
    }

    pub fn add_node(&mut self, value: N) {
        self.edges.push_default();
        self.nodes.push(value);
    }

    pub fn node(&self, index: usize) -> Option<NodeRef<N, E>> {
        match self.node_count() > index {
            true => Some(NodeRef {
                graph: self,
                index,
                value: &self.nodes[index],
            }),
            false => None,
        }
    }

    pub fn node_mut(&mut self, index: usize) -> Option<&mut N> {
        match self.node_count() > index {
            true => Some(&mut self.nodes[index]),
            false => None,
        }
    }

    pub fn remove_node(&mut self, index: usize) {
        self.edges.remove(index);
        self.nodes.remove(index);
    }

    pub fn set_edge(&mut self, index_a: usize, index_b: usize, value: E) {
        self.edges.set(index_a, index_b, MatrixCell::Edge(value));
    }

    pub fn remove_edge(&mut self, index_a: usize, index_b: usize) {
        self.edges.set(index_a, index_b, MatrixCell::Empty);
    }

    pub fn edge(&self, index_a: usize, index_b: usize) -> Option<EdgeRef<N, E>> {
        if self.node_count() > index_a && self.node_count() > index_b {
            match self.edges.get(index_a, index_b) {
                MatrixCell::Empty => None,
                MatrixCell::Edge(e) => Some(EdgeRef {
                    graph: self,
                    value: e,
                    index_a,
                    index_b,
                }),
            }
        } else {
            None
        }
    }

    pub fn edge_mut(&mut self, _index_a: usize, _index_b: usize) -> Option<&mut E> {
        todo!()
    }

    pub fn iter_nodes(&self) -> NodeIterator<N, E> {
        NodeIterator::new(self)
    }

    pub fn iter_edges(&self) -> EdgeIterator<N, E> {
        EdgeIterator::new(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeRef<'a, N, E> {
    graph: &'a Graph<N, E>,
    value: &'a N,
    index: usize,
}

impl<'a, N, E> NodeRef<'a, N, E> {
    pub fn value(&self) -> &N {
        self.value
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn iter_edges(&'a self) -> AdjEdgeIterator<'a, N, E> {
        AdjEdgeIterator::new(self.graph, self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeRef<'a, N, E> {
    graph: &'a Graph<N, E>,
    value: &'a E,
    index_a: usize,
    index_b: usize,
}

impl<'a, N, E> EdgeRef<'a, N, E> {
    pub fn value(&self) -> &E {
        self.value
    }

    pub fn a(self) -> NodeRef<'a, N, E> {
        self.graph.node(self.index_a).unwrap()
    }

    pub fn b(self) -> NodeRef<'a, N, E> {
        self.graph.node(self.index_b).unwrap()
    }
}

#[derive(Debug)]
pub struct NodeIterator<'a, N, E> {
    graph: &'a Graph<N, E>,
    current: usize,
}

impl<'a, N, E> NodeIterator<'a, N, E> {
    pub fn new(graph: &'a Graph<N, E>) -> Self {
        Self { graph, current: 0 }
    }
}

impl<'a, N, E> Iterator for NodeIterator<'a, N, E> {
    type Item = NodeRef<'a, N, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.graph.node(self.current);
        self.current += 1;

        item
    }
}

pub struct EdgeIterator<'a, N, E> {
    inner: Box<(dyn Iterator<Item = EdgeRef<'a, N, E>> + 'a)>,
}

impl<'a, N, E> EdgeIterator<'a, N, E> {
    pub fn new(graph: &'a Graph<N, E>) -> Self {
        let iter = graph
            .edges
            .iter()
            .filter(|(_, _, cell)| cell.is_edge())
            .map(|(index_a, index_b, value)| EdgeRef {
                graph,
                value: value.unwrap_ref(),
                index_a,
                index_b,
            });

        Self {
            inner: Box::new(iter),
        }
    }
}

impl<'a, N, E> Iterator for EdgeIterator<'a, N, E> {
    type Item = EdgeRef<'a, N, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct AdjEdgeIterator<'a, N, E> {
    inner: Box<(dyn Iterator<Item = EdgeRef<'a, N, E>> + 'a)>,
}

impl<'a, N, E> AdjEdgeIterator<'a, N, E> {
    pub fn new(graph: &'a Graph<N, E>, node: &'a NodeRef<N, E>) -> Self {
        let iter = graph
            .edges
            .iter()
            .filter(|(_, _, cell)| cell.is_edge())
            .filter(|(x, y, _)| *x == node.index || *y == node.index)
            .map(|(index_a, index_b, value)| EdgeRef {
                graph,
                value: value.unwrap_ref(),
                index_a,
                index_b,
            });

        Self {
            inner: Box::new(iter),
        }
    }
}

impl<'a, N, E> Iterator for AdjEdgeIterator<'a, N, E> {
    type Item = EdgeRef<'a, N, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let g = Graph::<u8, u8>::new();
        assert_eq!(g.node_count(), 0);
        assert_eq!(g.edge_count(), 0);
    }

    fn test_set() -> Graph<String, String> {
        let mut g = Graph::<String, String>::new();
        g.add_node("A".to_owned());
        g.add_node("B".to_owned());
        g.add_node("C".to_owned());

        g.set_edge(0, 1, "AB".to_owned());
        g.set_edge(1, 0, "BA".to_owned());
        g.set_edge(0, 2, "AC".to_owned());

        g
    }

    #[test]
    fn can_get_added_node() {
        let g = test_set();

        let a = g.node(0).unwrap();
        let b = g.node(1).unwrap();
        let c = g.node(2).unwrap();

        assert_eq!(a.value(), "A");
        assert_eq!(b.value(), "B");
        assert_eq!(c.value(), "C");
    }

    #[test]
    fn can_get_added_edge() {
        let g = test_set();

        assert_eq!(g.edge(0, 1).unwrap().value(), &"AB");
        assert_eq!(g.edge(1, 0).unwrap().value(), &"BA");
        assert_eq!(g.edge(0, 2).unwrap().value(), &"AC");
        assert_eq!(g.edge(2, 0), None);
        assert_eq!(g.edge(1, 2), None);
        assert_eq!(g.edge(2, 1), None);
    }

    #[test]
    fn can_remove_added_node() {
        let mut g = test_set();

        g.remove_node(1);
        assert_eq!(g.node(0).unwrap().value(), &"A");
        assert_eq!(g.node(1).unwrap().value(), &"C");
        assert_eq!(g.node(2), None);

        assert_eq!(g.edge(0, 1).unwrap().value(), &"AC");
        assert_eq!(g.edge(1, 0), None);
        assert_eq!(g.edge(0, 2), None);
    }

    #[test]
    fn can_remove_added_edge() {
        let mut g = test_set();

        g.remove_edge(0, 1);
        g.remove_edge(1, 0);
        g.remove_edge(0, 2);

        assert_eq!(g.edge(0, 1), None);
        assert_eq!(g.edge(1, 0), None);
        assert_eq!(g.edge(0, 2), None);
    }

    #[test]
    fn can_iter_over_nodes() {
        let g = test_set();
        let test = vec!["A", "B", "C"];

        assert_eq!(g.iter_nodes().count(), 3);

        g.iter_nodes()
            .zip(test.iter().enumerate())
            .for_each(|(a, (i, &b))| {
                assert_eq!(a.value(), b);
                assert_eq!(a.index(), i);
            })
    }

    #[test]
    fn can_iter_over_edges() {
        let g = test_set();
        let test = vec!["BA", "AB", "AC"];

        assert_eq!(g.iter_edges().count(), 3);

        g.iter_edges().zip(test.iter()).for_each(|(a, &b)| {
            assert_eq!(a.value(), b);
        })
    }

    #[test]
    fn can_iter_over_adj_edges() {
        let g = test_set();
        let node = g.node(1).unwrap();
        let mut iter = node.iter_edges();

        assert_eq!(iter.next().unwrap().value(), "BA");
        assert_eq!(iter.next().unwrap().value(), "AB");
        assert_eq!(iter.next(), None);
    }
}
