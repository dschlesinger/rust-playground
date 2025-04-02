use rand::prelude::*;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

struct Node {
    // doing prev + next -> next as undirected
    index: i32,
    next: Vec<i32>,
}

struct Tree {
    nodes: i32,
    data: Vec<Node>,
}

fn generate_tree(n: i32) -> Tree {

    let mut t: Tree = Tree { nodes: 1, data: Vec::with_capacity((n + 1) as usize) };

    t.data.push(Node { index: 0, next: vec![] } );

    let mut empty: Vec<i32> = vec![0];

    while (t.data.len() as i32) < n {

        let mut rng = rand::rng();

        // Number of new children
        let nc: i32 = *vec![1,2].choose(&mut rng).unwrap();

        t.nodes += nc;

        // Victim Node
        let target_index = *(0..(empty.len() as i32)).collect::<Vec<i32>>().choose(&mut rng).unwrap();

        let target = empty[target_index as usize];

        empty.remove(target_index as usize);

        for i in 1..(nc + 1) {

            // new node index
            let ni: i32 = t.data.len() as i32;

            empty.push(ni);

            // Update Parent
            t.data[target as usize].next.push(ni);

            // Add child, with connection to parent
            t.data.push(Node { index: ni, next: vec![target] } );

        }

    }

    t
}

fn save_tree(t: &Tree, filename: &str) -> () {
    /// Saves tree to file
    /// Errors if files exists
    
    let testPath = Path::new(filename);

    if testPath.exists() {
        // panic!("File already exists");
    }
    
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename).expect("File Errored");

    writeln!(file, "{}", t.nodes);

    for node in t.data.iter() {

        for next in node.next.iter() {
            writeln!(file, "{} {}", node.index, t.data[*next as usize].index);
        }

    }

}

fn index_of_farthest(ni: i32, t: &Tree) -> (i32, i32) {
    /// Returns (index, distance)

    // Init counter array with count distance from ni
    let mut count: Vec<i32> = vec![-1; t.nodes as usize];
    count[ni as usize] = 0;

    let mut que: Vec<i32> = vec![ni];

    while que.len() != 0 {

        let curr_index: i32 = que.remove(0);

        let curr_dis: i32 = count[curr_index as usize];

        for next_index in t.data[curr_index as usize].next.iter() {
            
            // Check if unvisited
            if count[*next_index as usize] == -1 {
                count[*next_index as usize] = curr_dis + 1;
                que.push(*next_index);
            }

        }

    }

    // Find max index

    let mut max_value: i32 = -1;
    let mut max_index: i32 = -1;

    for (pos, val) in count.iter().enumerate() {
        if *val > max_value {
            max_value = *val;
            max_index = pos as i32;
        }
    }

    println!("Index: {}\nMax: {}", max_index, max_value);

    (max_index, max_value)

}

fn diameter(t: &Tree) -> i32 {
    /// Does two BFS to find diameter

    // Random start
    let mut rng = rand::rng();

    let start_index = *(0..(t.data.len() as i32)).collect::<Vec<i32>>().choose(&mut rng).unwrap();

    println!("Start Index: {}", start_index);

    let (first_index, _) = index_of_farthest(start_index, &t);

    let (_, diameter) = index_of_farthest(first_index, &t);

    diameter

}

fn main() {

    let t: Tree = generate_tree(120);

    println!("{}", diameter(&t));

    // In this context bfs and dfs are the same as the graph is unweighted and acyclic
    // We can optimize by doing two searchs, one find the farthest from an arbitrary point
    // The second find the distance from that point to its furthest
}