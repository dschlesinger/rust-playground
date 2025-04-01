use super::graph::{Node, Graph};

use std::{fs, io};
use std::collections::HashMap;

pub fn read_graph(filename: &str) -> Graph {
    // Get file
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut lines: Vec<&str> = contents.lines().collect();

    // Get number of nodes
    let nodes: i32 = lines
        .remove(0)
        .parse()
        .expect("Not a valid number of nodes");

    // Initialize HashMap with empty nodes
    let mut map: HashMap<i32, Node> = HashMap::new();
    for i in 0..nodes {
        map.insert(i, Node { index: i, next: vec![] });
    }

    // Load edges
    for l in lines.iter() {
        let mut parts = l.split_whitespace();

        let from: i32 = parts.next().unwrap().parse().expect("Invalid node");
        let to: i32 = parts.next().unwrap().parse().expect("Invalid node");

        // Insert edge into the graph
        if let Some(node) = map.get_mut(&from) {
            node.next.push(to);
        }
    }

    Graph { nodes, data: map }
}
