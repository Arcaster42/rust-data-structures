use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
struct GraphError(String);

impl fmt::Display for GraphError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Error for GraphError {}

pub struct GraphStruct {
  name: String,
  nodes: Vec<Node>,
  edges: Vec<Edge>
}

#[derive(Clone)]
#[derive(PartialEq)]
struct Node {
  name: String,
  edges: Vec<Edge>
}

#[derive(Clone)]
#[derive(PartialEq)]
struct Edge {
  start: Node,
  end: Node
}

impl GraphStruct {
  fn new(name: String) -> Self {
    Self { name: name, nodes: Vec::new(), edges: Vec::new() }
  }

  fn get_name(&self) -> &str {
    return &self.name;
  }

  fn add_node(&mut self, name: String) {
    self.nodes.push(Node { name: name, edges: Vec::new() });
  }

  fn add_edge(&mut self, start: &str, end: &str) -> Result<(), GraphError> {
    let start_node = self.nodes.iter().find(|node| node.name == start);
    let end_node = self.nodes.iter().find(|node| node.name == end);
    match (start_node, end_node) {
      (Some(start_node), Some(end_node)) => {
        self.edges.push(Edge { start: start_node.clone(), end: end_node.clone() });
        return Ok(());
      }

      _ => {
        return Err(GraphError("Invalid Nodes".to_string()));
      }
    }
  }

  fn delete_node(&mut self, name: &str) -> Result<(), GraphError> {
    match self.nodes.iter().find(|node| node.name == name) {
      Some(_node) => {
        self.nodes.retain(|node| node.name != name);
        self.edges.retain(|edge| edge.start.name != name && edge.end.name != name);
        return Ok(());
      }
      None => {
        return Err(GraphError("Node not found".to_string()));
      }
    }
  }

  fn delete_edge(&mut self, start: &str, end: &str) -> Result<(), GraphError> {
    match self.edges.iter().find(|edge| edge.start.name == start && edge.end.name == end) {
      Some(_edge) => {
        self.edges.retain(|edge| edge.start.name != start || edge.end.name != end);
        return Ok(())
      }
      None => {
        return Err(GraphError("Edge not found".to_string()));
      }
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = GraphStruct::new("Test Graph".to_string());
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.get_name(), "Test Graph");
    }

    #[test]
    fn test_add_edge() {
        let mut graph = GraphStruct::new("Test Graph".to_string());
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        let result = graph.add_edge("A", "B");
        assert_eq!(result, Ok(()));
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges[0].start.name, "A".to_string());
        assert_eq!(graph.edges[0].end.name, "B".to_string());
    }

    #[test]
    fn test_add_edge_with_invalid_nodes() {
        let mut graph = GraphStruct::new("Test Graph".to_string());
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        let result = graph.add_edge("A", "C");
        assert_eq!(result, Err(GraphError("Invalid Nodes".to_string())));
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn test_add_edge_with_missing_nodes() {
        let mut graph = GraphStruct::new("Test Graph".to_string());
        let result = graph.add_edge("A", "B");
        assert_eq!(result, Err(GraphError("Invalid Nodes".to_string())));
    }

    #[test]
    fn test_complex_graph() {
        let mut graph = GraphStruct::new("test_graph".to_string());
        
        // Add nodes
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_node("C".to_string());
        graph.add_node("D".to_string());

        // Add edges
        graph.add_edge("A", "B").unwrap();
        graph.add_edge("B", "C").unwrap();
        graph.add_edge("C", "D").unwrap();
        graph.add_edge("D", "A").unwrap();
        graph.add_edge("B", "D").unwrap();
        
        // Check node count
        assert_eq!(graph.nodes.len(), 4);
        
        // Check edge count
        assert_eq!(graph.edges.len(), 5);
        
        // Check edge start and end nodes
        assert_eq!(graph.edges[0].start.name, "A".to_string());
        assert_eq!(graph.edges[0].end.name, "B".to_string());
        assert_eq!(graph.edges[1].start.name, "B".to_string());
        assert_eq!(graph.edges[1].end.name, "C".to_string());
        assert_eq!(graph.edges[2].start.name, "C".to_string());
        assert_eq!(graph.edges[2].end.name, "D".to_string());
        assert_eq!(graph.edges[3].start.name, "D".to_string());
        assert_eq!(graph.edges[3].end.name, "A".to_string());
        assert_eq!(graph.edges[4].start.name, "B".to_string());
        assert_eq!(graph.edges[4].end.name, "D".to_string());
    }

    #[test]
    fn test_node_deletion() {
        let mut graph = GraphStruct::new("test".to_string());
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_edge("A", "B").unwrap();
        graph.delete_node("A").unwrap();
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn test_edge_deletion() {
        let mut graph = GraphStruct::new("test".to_string());
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_edge("A", "B").unwrap();
        graph.delete_edge("A", "B").unwrap();
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn test_node_deletion_with_invalid_node() {
        let mut graph = GraphStruct::new("test".to_string());
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_edge("A", "B").unwrap();
        let result = graph.delete_node("C");
        assert!(result.is_err());
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn test_edge_deletion_with_invalid_nodes() {
        let mut graph = GraphStruct::new("test".to_string());
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_edge("A", "B").unwrap();
        let result = graph.delete_edge("A", "C");
        assert!(result.is_err());
        assert_eq!(graph.edges.len(), 1);
    }
}
