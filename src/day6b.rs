use std::io::{self, BufRead};
use std::collections::HashMap;

type Graph = HashMap<String, String>;

fn get_santa_transfers(graph: &Graph) -> HashMap<String, i32> {
    let mut santa_transfers : HashMap<String, i32> = HashMap::new();
    let mut current_node = graph.get("SAN").unwrap().to_string();
    let mut current_distance = 0;
    while current_node != "COM" {
        santa_transfers.insert(current_node.clone(), current_distance);
        current_distance += 1;
        current_node = graph.get(&current_node).unwrap().to_string();
    }
    return santa_transfers;
}

fn main() -> io::Result<()> {
    let mut graph : Graph = HashMap::new();
    for line_result in io::stdin().lock().lines() {
       let line = line_result?;
       let mut iter = line.split(')');
       let center = iter.next().unwrap();
       let orbiting = iter.next().unwrap();

       graph.insert(orbiting.to_string(), center.to_string());
    }
    let santa_transfers = get_santa_transfers(&graph);

    let mut distance_from_you = 0;
    let mut current_node = graph.get("YOU").unwrap().to_string();
    loop {
        match santa_transfers.get(&current_node) {
            Some(santa_transfer_count) => {
                println!("{}", santa_transfer_count + distance_from_you);
                break;
            }
            None => {
                current_node = graph.get(&current_node).unwrap().to_string();
                distance_from_you += 1;
            }
        }
    }
    Ok(())
}
