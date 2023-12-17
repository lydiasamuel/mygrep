// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
use std::cell::RefCell;
use std::rc::Rc;

pub struct Graph<T, U> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge<U>>,
}

pub type NodeIndex = usize;

pub struct Node<T> {
    first_outgoing_edge: Option<EdgeIndex>,
    data: Rc<RefCell<T>>
}

pub type EdgeIndex = usize;

pub struct Edge<U> {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
    data: Rc<RefCell<U>>
}

impl<T, U> Graph<T, U> {
    pub fn new() -> Graph<T, U> {
        Graph { nodes: Vec::new(),
                edges: Vec::new(), }
    }

    pub fn add_node(&mut self, data: T) -> NodeIndex {
        let index = self.nodes.len();

        self.nodes.push(Node::new( 
            None, 
            data
        ));

        return index;
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, data: U) {
        let edge_index = self.edges.len();

        let node_data = &mut self.nodes[source];

        self.edges.push(Edge::new(
            target,
            node_data.first_outgoing_edge,
            data
        ));

        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn outgoing_edges(&self, source: NodeIndex) -> Result<Vec<EdgeIndex>, String> {
        let source = self.check_node_index(source)?;

        let mut edges: Vec<EdgeIndex> = Vec::new();

        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        
        match first_outgoing_edge {
            Some(edge) => {
                edges.push(edge);

                let mut prev = edge;
                while let Some(next) = self.edges[prev].next_outgoing_edge {
                    edges.push(next);
                    prev = next;
                } 
            },
            None => ()
        }

        return Ok(edges);
    }

    pub fn traverse(&self, edge: EdgeIndex) -> Result<NodeIndex, String> {
        let edge = self.check_edge_index(edge)?;

        return Ok(self.edges[edge].target);
    }

    fn check_node_index(&self, index: NodeIndex) -> Result<NodeIndex, String> {
        let end = self.nodes.len() - 1;

        if index > end {
            return Err(format!("Error - Invalid source node index: {}, must be between 0-{}", index.to_string(), end.to_string()));
        }

        return Ok(index);
    }

    fn check_edge_index(&self, index: EdgeIndex) -> Result<EdgeIndex, String> {
        let end = self.edges.len() - 1;

        if index > end {
            return Err(format!("Error - Invalid edge index: {}, must be between 0-{}", index.to_string(), end.to_string()));
        }

        return Ok(index);
    }

    pub fn get_node_data(&self, index: NodeIndex) -> Rc<RefCell<T>> {
        return self.nodes[index].data.clone();
    }

    pub fn get_edge_data(&self, index: EdgeIndex) -> Rc<RefCell<U>> {
        return self.edges[index].data.clone();
    }


}

impl<T> Node<T> {
    pub fn new(first_outgoing_edge: Option<EdgeIndex>, data: T) -> Node<T> {
        return Node {
            first_outgoing_edge,
            data: Rc::new(RefCell::new(data)),
        }
    }
}

impl<U> Edge<U> {
    pub fn new(target: NodeIndex, next_outgoing_edge: Option<EdgeIndex>, data: U) -> Edge<U> {
        return Edge {
            target,
            next_outgoing_edge,
            data: Rc::new(RefCell::new(data)),
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn given_basic_graph_when_outgoing_edges_called_for_node_should_return_all_outgoing_edges_for_node() {

        // N0 ---E0---> N1 ---E1---> 2
        // |                         ^
        // E2                        |
        // |                         |
        // v                         |
        // N3 ----------E3-----------+

        let mut graph: Graph<&str, &str> = Graph::new();

        let n0 = graph.add_node("n0");
        let n1 = graph.add_node("n1");
        let n2 = graph.add_node("n2");
        let n3 = graph.add_node("n3");

        graph.add_edge(n0, n1, "n0->n1"); // e0
        graph.add_edge(n1, n2, "n1->n2"); // e1
        graph.add_edge(n0, n3, "n0->n3"); // e2
        graph.add_edge(n3, n2, "n3->n2"); // e3

        let n0_outgoing_edges = graph.outgoing_edges(n0).unwrap();

        assert!(n0_outgoing_edges.len() == 2);
        assert!(*graph.get_edge_data(n0_outgoing_edges[0]).borrow() == "n0->n3");
        assert!(*graph.get_edge_data(n0_outgoing_edges[1]).borrow() == "n0->n1");

        let n1_outgoing_edges = graph.outgoing_edges(n1).unwrap();

        assert!(n1_outgoing_edges.len() == 1);
        assert!(*graph.get_edge_data(n1_outgoing_edges[0]).borrow() == "n1->n2");

        let n2_outgoing_edges = graph.outgoing_edges(n2).unwrap();

        assert!(n2_outgoing_edges.len() == 0);

        let n3_outgoing_edges = graph.outgoing_edges(n3).unwrap();

        assert!(n3_outgoing_edges.len() == 1);
        assert!(*graph.get_edge_data(n3_outgoing_edges[0]).borrow() == "n3->n2");

    }

    #[test]
    fn given_basic_graph_that_loops_when_traversing_through_from_start_should_loopback_to_the_start() {

        // N0 ---E0---> N1 ---E1---> N2
        // ^                         |
        // |                         |
        // |                         |
        // |                         |
        // +-----------E2------------+

        let mut graph: Graph<&str, &str> = Graph::new();

        let n0 = graph.add_node("n0");
        let n1 = graph.add_node("n1");
        let n2 = graph.add_node("n2");

        graph.add_edge(n0, n1, "n0->n1"); // e0
        graph.add_edge(n1, n2, "n1->n2"); // e1
        graph.add_edge(n2, n0, "n2->n0"); // e2

        let n0_outgoing_edges = graph.outgoing_edges(n0).unwrap();
        assert!(n0_outgoing_edges.len() == 1);

        let first_jump = graph.traverse(n0_outgoing_edges[0]).unwrap();
        assert!(*graph.get_node_data(first_jump).borrow() == "n1");

        let n1_outgoing_edges = graph.outgoing_edges(first_jump).unwrap();
        assert!(n1_outgoing_edges.len() == 1);

        let second_jump = graph.traverse(n1_outgoing_edges[0]).unwrap();
        assert!(*graph.get_node_data(second_jump).borrow() == "n2");

        let n2_outgoing_edges = graph.outgoing_edges(second_jump).unwrap();
        assert!(n2_outgoing_edges.len() == 1);

        let third_jump = graph.traverse(n2_outgoing_edges[0]).unwrap();
        assert!(*graph.get_node_data(third_jump).borrow() == "n0");
    }
}