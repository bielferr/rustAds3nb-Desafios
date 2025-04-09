use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::dot::{Dot, Config};
pub struct MyGraph {
    graph: Graph<&'static str, &'static str>,
    node1: NodeIndex,
}

impl MyGraph {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        
        let node1 = graph.add_node("1");
        let node2 = graph.add_node("2");
        let node3 = graph.add_node("3");
        let node4 = graph.add_node("4");
        let node5 = graph.add_node("5");
        let node6 = graph.add_node("6");
        
        graph.add_edge(node1, node2, "");
        graph.add_edge(node1, node3, "");
        graph.add_edge(node2, node4, "");
        graph.add_edge(node2, node5, "");
        graph.add_edge(node3, node6, "");
        
        MyGraph { graph, node1 }
    }

    pub fn dfs_from_node1(&self) -> Vec<&'static str> {
        let mut dfs = Dfs::new(&self.graph, self.node1);
        let mut visited_nodes = Vec::new();
        
        while let Some(node) = dfs.next(&self.graph) {
            visited_nodes.push(self.graph[node]);
        }
        visited_nodes
    }

    pub fn to_dot(&self) -> String {
        format!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_visits_all_nodes() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();
        let mut sorted_result = result.clone();
        sorted_result.sort();
        let mut expected = vec!["1", "2", "3", "4", "5", "6"];
        expected.sort();
        assert_eq!(sorted_result, expected);
    }

    #[test]
    fn test_dfs_starts_at_node1() {
        let g = MyGraph::new();
        assert_eq!(g.dfs_from_node1().first(), Some(&"1"));
    }
}