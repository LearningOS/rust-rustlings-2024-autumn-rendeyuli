/*
	graph
	This problem requires you to implement a basic graph functio
*/


use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        let (from,to,weight)=edge;
        if !self.contains(from){
            self.add_node(from);
        }
        if !self.contains(to){
            self.add_node(to);
        }
        let graph=self.adjacency_table_mutable();
        if let Some(from_node)=graph.get_mut(from){
            from_node.push((to.to_string(),weight));//注意这里的借用，如果是graph.get()的话，from_node
            //就不能更改，因此必须是get_mut(),而要想get_mut()，则graph必须是mutable，所以这里graph=self.adjacency_table_mutable()
        }
        if let Some(to_node)=graph.get_mut(to){
            to_node.push((from.to_string(),weight));
        }

    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        //TODO
        self.adjacency_table_mutable().insert(node.to_string(),Vec::new());
		true
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}