use crate::matrix::AdjacencyMatrix;
use std::fmt::Debug;

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

    pub fn unwrap_ref(&self) -> &T {
        match self {
            MatrixCell::Empty => panic!(),
            MatrixCell::Edge(e) => e,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Graph<N, E> {
    edges: AdjacencyMatrix<MatrixCell<E>>,
    nodes: Vec<N>,
    directed: bool,
}

impl<N, E> Graph<N, E> {
    pub fn new_directed() -> Self {
        Self {
            edges: AdjacencyMatrix::new(),
            nodes: Vec::new(),
            directed: true,
        }
    }

    pub fn new_undirected() -> Self {
        Self {
            edges: AdjacencyMatrix::new(),
            nodes: Vec::new(),
            directed: false,
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        let count = self
            .edges
            .iter()
            .filter(|(_, _, e)| match e {
                MatrixCell::Empty => false,
                MatrixCell::Edge(_) => true,
            })
            .count();

        count
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

    pub fn remove_node(&mut self, index: usize) {
        self.edges.remove(index);
        self.nodes.remove(index);
    }

    pub fn set_edge(&mut self, index_a: usize, index_b: usize, value: E) {
        let (a, b) = if !self.directed && index_a < index_b {
            (index_b, index_a)
        } else {
            (index_a, index_b)
        };

        self.edges.set(a, b, MatrixCell::Edge(value));
    }

    pub fn remove_edge(&mut self, index_a: usize, index_b: usize) {
        let (a, b) = if !self.directed && index_a < index_b {
            (index_b, index_a)
        } else {
            (index_a, index_b)
        };

        self.edges.set(a, b, MatrixCell::Empty);
    }

    pub fn edge(&self, index_a: usize, index_b: usize) -> Option<EdgeRef<N, E>> {
        let (a, b) = if !self.directed && index_a < index_b {
            (index_b, index_a)
        } else {
            (index_a, index_b)
        };

        if self.node_count() > a && self.node_count() > b {
            match self.edges.get(a, b) {
                MatrixCell::Empty => None,
                MatrixCell::Edge(e) => Some(EdgeRef {
                    graph: self,
                    value: e,
                    index_a: a,
                    index_b: b,
                }),
            }
        } else {
            None
        }
    }

    pub fn nodes(&self) -> NodeIterator<N, E> {
        NodeIterator::new(self)
    }

    pub fn edges(&self) -> EdgeIterator<N, E> {
        EdgeIterator::new(self)
    }
}

impl<N, E> Debug for Graph<N, E>
where
    N: Debug,
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Nodes:")?;
        f.debug_map()
            .entries(self.nodes().map(|n| (n.index, n.value)))
            .finish()?;
        f.write_str("\n")?;
        f.write_str("Edges:")?;
        f.debug_list()
            .entries(self.edges().map(|e| (e.index_a, e.index_b, e.value)))
            .finish()?;
        f.write_str("\n")?;

        Ok(())
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

    pub fn a(&self) -> NodeRef<'a, N, E> {
        self.graph.node(self.index_a).unwrap()
    }

    pub fn b(&self) -> NodeRef<'a, N, E> {
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

    fn directed_test_graph() -> Graph<String, String> {
        let mut g = Graph::<String, String>::new_directed();
        g.add_node("A".to_owned());
        g.add_node("B".to_owned());
        g.add_node("C".to_owned());

        g.set_edge(0, 1, "AB".to_owned());
        g.set_edge(1, 0, "BA".to_owned());
        g.set_edge(0, 2, "AC".to_owned());

        g
    }

    fn undirected_test_graph() -> Graph<String, String> {
        let mut g = Graph::<String, String>::new_undirected();
        g.add_node("A".to_owned());
        g.add_node("B".to_owned());
        g.add_node("C".to_owned());

        g.set_edge(0, 1, "AB".to_owned());
        g.set_edge(0, 2, "AC".to_owned());

        g
    }

    #[test]
    fn can_create() {
        let a = Graph::<u8, u8>::new_directed();
        assert_eq!(a.node_count(), 0);
        assert_eq!(a.edge_count(), 0);

        let b = Graph::<u8, u8>::new_undirected();
        assert_eq!(b.node_count(), 0);
        assert_eq!(b.edge_count(), 0);
    }

    #[test]
    fn can_count_edges() {
        let a = directed_test_graph();
        assert_eq!(a.edge_count(), 3);

        let b = undirected_test_graph();
        assert_eq!(b.edge_count(), 2);
    }

    #[test]
    fn can_count_nodes() {
        let a = directed_test_graph();
        assert_eq!(a.node_count(), 3);

        let b = undirected_test_graph();
        assert_eq!(b.node_count(), 3);
    }

    #[test]
    fn can_get_added_node() {
        let g = directed_test_graph();

        let a = g.node(0).unwrap();
        let b = g.node(1).unwrap();
        let c = g.node(2).unwrap();

        assert_eq!(a.value(), "A");
        assert_eq!(b.value(), "B");
        assert_eq!(c.value(), "C");
    }

    #[test]
    fn can_get_added_edge() {
        let a = directed_test_graph();
        assert_eq!(a.edge(0, 1).unwrap().value(), &"AB");
        assert_eq!(a.edge(1, 0).unwrap().value(), &"BA");
        assert_eq!(a.edge(0, 2).unwrap().value(), &"AC");
        assert_eq!(a.edge(2, 0), None);
        assert_eq!(a.edge(1, 2), None);
        assert_eq!(a.edge(2, 1), None);

        let b = undirected_test_graph();
        assert_eq!(b.edge(0, 1).unwrap().value(), &"AB");
        assert_eq!(b.edge(1, 0).unwrap().value(), &"AB");
        assert_eq!(b.edge(0, 2).unwrap().value(), &"AC");
        assert_eq!(b.edge(2, 0).unwrap().value(), &"AC");
        assert_eq!(b.edge(1, 2), None);
        assert_eq!(b.edge(2, 1), None);
    }

    #[test]
    fn can_remove_added_node() {
        let mut g = directed_test_graph();

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
        let mut g = directed_test_graph();
        g.remove_edge(0, 1);
        g.remove_edge(1, 0);
        g.remove_edge(0, 2);
        assert_eq!(g.edge(0, 1), None);
        assert_eq!(g.edge(1, 0), None);
        assert_eq!(g.edge(0, 2), None);

        let mut g = undirected_test_graph();
        g.remove_edge(0, 1);
        g.remove_edge(0, 2);
        assert_eq!(g.edge(0, 1), None);
        assert_eq!(g.edge(1, 0), None);
        assert_eq!(g.edge(0, 2), None);
    }

    #[test]
    fn can_iter_over_nodes() {
        let g = directed_test_graph();
        let test = vec!["A", "B", "C"];

        assert_eq!(g.nodes().count(), 3);

        g.nodes()
            .zip(test.iter().enumerate())
            .for_each(|(a, (i, &b))| {
                assert_eq!(a.value(), b);
                assert_eq!(a.index(), i);
            })
    }

    #[test]
    fn can_iter_over_edges() {
        let g = directed_test_graph();
        let test = vec!["BA", "AB", "AC"];

        assert_eq!(g.edges().count(), 3);

        g.edges().zip(test.iter()).for_each(|(a, &b)| {
            assert_eq!(a.value(), b);
        })
    }

    #[test]
    fn can_iter_over_adj_edges() {
        {
            let g = directed_test_graph();
            let node = g.node(1).unwrap();
            let mut iter = node.iter_edges();

            assert_eq!(iter.next().unwrap().value(), "BA");
            assert_eq!(iter.next().unwrap().value(), "AB");
            assert_eq!(iter.next(), None);
        }

        {
            let g = undirected_test_graph();
            let node = g.node(1).unwrap();
            let mut iter = node.iter_edges();

            assert_eq!(iter.next().unwrap().value(), "AB");
            assert_eq!(iter.next(), None);
        }
    }
}
