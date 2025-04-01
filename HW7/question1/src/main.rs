use std::{fs, io};
use std::collections::HashMap;

mod utils;
use utils::file::{ read_graph };
use utils::graph::{ Node, Graph, random_walk };

fn get_top5(g: &Graph, results: &Vec<i32>) -> Vec<(i32, f32)> {
    let scores = (0..g.nodes).map(|i| { results.iter().filter(|&&x| x == i).count() as i32 });

    let mut score_pairs: Vec<(i32, i32)> = (0..g.nodes)
        .zip(scores.into_iter())
        .collect();

    // compare b which is score
    score_pairs.sort_by_key(|&(a, b)| b);

    let top5: Vec<(i32, i32)> = score_pairs.iter().rev().take(5).copied().collect();

    // Divide by total
    top5.iter().map(|&(index, score)| (index, score as f32 / 90.0)).collect()
}

fn main() {
    let g: Graph = read_graph("data/pagerank_data.txt");

    // println!("{:?}", _graph.data.get(&0).unwrap().next);

    let mut results: Vec<i32> = Vec::with_capacity(90);

    for _ in 0..90 {
        results.push(random_walk(&g, None, 90));
    }

    let top5 = get_top5(&g, &results);

    for (vertex, num) in top5.iter() {
        println!("Vertex {}: approximate PageRank {}", vertex, num);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {

        let g: Graph = read_graph("data/test2.txt");

    // println!("{:?}", _graph.data.get(&0).unwrap().next);

    let mut results: Vec<i32> = Vec::with_capacity(90);

    for _ in 0..90 {
        results.push(random_walk(&g, None, 90));
    }

    let top5 = get_top5(&g, &results);

    for (vertex, num) in top5.iter() {
        println!("Vertex {}: approximate PageRank {}", vertex, num);
    }

    assert_eq!(top5.first().map(|&(i, s)| i), Some(3));
        
    }
}