use std::collections::HashMap;
use rand::prelude::*;

pub struct Node {
    pub index: i32,
    pub next: Vec<i32>,
}

pub struct Graph {
    pub nodes: i32,
    pub data: HashMap<i32, Node>,
}

fn random_select(v: &Vec<i32>) -> i32 {
    let mut rng = rand::rng();
    *v.choose(&mut rng).expect("Vector is empty")
}

fn get_random_node(g: &Graph) -> i32 {
    let mut rng = rand::rng();
    *g.data.keys().choose(&mut rng).expect("Graph is empty")
}

pub fn random_walk(g: &Graph, n: Option<i32>, steps: i32) -> i32 {

    let mut rng = rand::rng();
    let mut current_node = match n {
        Some(node_id) => node_id, 
        None => get_random_node(&g), // Pick a random start node
    };

    for i in 0..steps {
        if let Some(node) = g.data.get(&current_node) {
            if !node.next.is_empty() {

                let rn: i32 = random_select(&(1..11).collect());

                // 8 / 10 select random next
                if (rn >= 3) {
                    current_node = random_select(&node.next);
                }
                
                // 2 / 10 jump
                else {
                    current_node = get_random_node(&g);
                }

            } else {
                // Random jump, no next
                current_node = get_random_node(&g);
            }
        }
    }

    current_node
}
