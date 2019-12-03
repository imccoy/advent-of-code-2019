use std::io::{self, BufRead};
use std::cmp::min;

struct Horizontal { y: i32, x1: i32, x2: i32, cost: i32 }
struct Vertical { x: i32, y1: i32, y2: i32, cost: i32 }
struct Wire { horizontals: Vec<Horizontal>, verticals: Vec<Vertical> }

fn read_wire(line: String) -> Wire {
    let mut horizontals : Vec<Horizontal> = Vec::new();
    let mut verticals : Vec<Vertical> = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut cost = 0;
    for section in line.split(',') {
        let direction : &str = &section[0..1];
        let distance = section[1..].parse::<i32>().unwrap();
        match direction {
            "R" => {
                let x1 = x;
                let x2 = x + distance;
                horizontals.push(Horizontal { y, x1, x2, cost });
                x = x2;
            },
            "L" => {
                let x1 = x;
                let x2 = x - distance;
                horizontals.push(Horizontal { y, x1, x2, cost });
                x = x2;
            },
            "D" => {
                let y1 = y;
                let y2 = y + distance;
                verticals.push(Vertical { x, y1, y2, cost });
                y = y2;
            },
            "U" => {
                let y1 = y;
                let y2 = y - distance;
                verticals.push(Vertical { x, y1, y2, cost });
                y = y2;
            },
            _ => println!("unknown")
        }
        cost += distance;
    }
    Wire { horizontals, verticals }
}

fn between(p : i32, start : i32, end : i32) -> bool {
    return (start < end && p > start && p < end) || (end < start && between(p, end, start));
}

fn segment_cost(start_cost : i32, p : i32, start : i32, end : i32) -> i32 {
    if start < end {
        return start_cost + (p - start);
    } else {
        return start_cost + (start - p);
    }
}

fn find_closest_intersection_to_origin_between_segments(horizontals: Vec<Horizontal>, verticals: Vec<Vertical>) -> Option<i32> {
    let mut closest = None;
    for horizontal in horizontals {
        for vertical in &verticals {
            if between(horizontal.y, vertical.y1, vertical.y2) && between(vertical.x, horizontal.x1, horizontal.x2) {
                let distance = segment_cost(vertical.cost, horizontal.y, vertical.y1, vertical.y2) + segment_cost(horizontal.cost, vertical.x, horizontal.x1, horizontal.x2);
                closest = Some(closest.map_or(distance, |distance0| min(distance, distance0)));
            }
        }
    }
    return closest;
}


fn find_closest_intersection_to_origin_between_wires(wire1 : Wire, wire2 : Wire) -> Option<i32> {
    let a = find_closest_intersection_to_origin_between_segments(wire1.horizontals, wire2.verticals);
    let b = find_closest_intersection_to_origin_between_segments(wire2.horizontals, wire1.verticals);
    a.iter().chain(b.iter()).min().map(|distance| *distance)
}

fn main() -> io::Result<()> {
    let mut wires : Vec<Wire> = Vec::new();
    for line in io::stdin().lock().lines() {
        wires.push(read_wire(line?));
    }
    println!("{:?}", find_closest_intersection_to_origin_between_wires(wires.pop().unwrap(), wires.pop().unwrap()));
    Ok(())
}
