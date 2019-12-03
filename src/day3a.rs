use std::io::{self, BufRead};
use std::cmp::min;

struct Horizontal { y: i32, x1: i32, x2: i32 }
struct Vertical { x: i32, y1: i32, y2: i32 }
struct Wire { horizontals: Vec<Horizontal>, verticals: Vec<Vertical> }

fn read_wire(line: String) -> Wire {
    let mut horizontals : Vec<Horizontal> = Vec::new();
    let mut verticals : Vec<Vertical> = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for section in line.split(',') {
        let direction : &str = &section[0..1];
        let distance = section[1..].parse::<i32>().unwrap();
        match direction {
            "R" => {
                let x1 = x;
                let x2 = x + distance;
                horizontals.push(Horizontal { y, x1, x2 });
                x = x2;
            },
            "L" => {
                let x1 = x - distance;
                let x2 = x;
                horizontals.push(Horizontal { y, x1, x2 });
                x = x1;
            },
            "D" => {
                let y1 = y;
                let y2 = y + distance;
                verticals.push(Vertical { x, y1, y2 });
                y = y2;
            },
            "U" => {
                let y1 = y - distance;
                let y2 = y;
                verticals.push(Vertical { x, y1, y2 });
                y = y1;
            },
            _ => println!("unknown")
        }
    }
    Wire { horizontals, verticals }
}

fn find_closest_intersection_to_origin_between_segments(horizontals: Vec<Horizontal>, verticals: Vec<Vertical>) -> Option<i32> {
    let mut closest = None;
    for horizontal in horizontals {
        for vertical in &verticals {
            if horizontal.y > vertical.y1 && horizontal.y < vertical.y2 && vertical.x > horizontal.x1 && vertical.x < horizontal.x2 {
                let distance = horizontal.y.abs() + vertical.x.abs();
                closest = Some(closest.map_or(distance, |distance0| min(distance, distance0)));
            }
        }
    }
    return closest;
}


fn find_closest_intersection_to_origin_between_wires(wire1 : Wire, wire2 : Wire) -> Option<i32> {
    let a = find_closest_intersection_to_origin_between_segments(wire1.horizontals, wire2.verticals);
    let b = find_closest_intersection_to_origin_between_segments(wire2.horizontals, wire1.verticals);
    match (a, b) {
        (None, None) => None,
        (Some(n), None) => Some(n),
        (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => Some(min(n1, n2))
    }
}

fn main() -> io::Result<()> {
    let mut wires : Vec<Wire> = Vec::new();
    for line in io::stdin().lock().lines() {
        wires.push(read_wire(line?));
    }
    println!("{:?}", find_closest_intersection_to_origin_between_wires(wires.pop().unwrap(), wires.pop().unwrap()));
    Ok(())
}
