use std::io::{self, BufRead};
use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;

fn count(graph : &Graph, distance: i32, node_name: String) -> i32 {
    graph.get(&node_name).map(|next_node_names| {
       let orbiter_counts = next_node_names.iter()
           .map(|next_node_name| {
               count(graph, distance + 1, next_node_name.to_string())
           });
       let orbiter_counts_sum : i32 = orbiter_counts.sum();
       return distance + orbiter_counts_sum;
    }).unwrap_or(distance)
}

fn main() -> io::Result<()> {
    let mut graph : Graph = HashMap::new();
    for line_result in io::stdin().lock().lines() {
       let line = line_result?;
       let mut iter = line.split(')');
       let center = iter.next().unwrap();
       let orbiting = iter.next().unwrap();

       let vec = graph.entry(center.to_string()).or_insert(Vec::new());
       vec.push(orbiting.to_string());
    }
    println!("{}", count(&graph, 0, "COM".to_string()));
    Ok(())
}
