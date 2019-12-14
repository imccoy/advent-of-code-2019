use std::io::{self, BufRead};
use std::collections::hash_map::{HashMap, Entry};
use num::BigUint;
use num::Integer;

#[derive(Clone,Debug,Hash,PartialEq,Eq,Ord,PartialOrd)]
struct Velocity { x : i32, y : i32, z : i32 }
#[derive(Clone,Debug,Hash,PartialEq,Eq,Ord,PartialOrd)]
struct Position { x : i32, y : i32, z : i32 }

#[derive(Clone,Debug,Hash,PartialEq,Eq,Ord,PartialOrd)]
struct Moon { position: Position, velocity: Velocity }

fn parse_moon(line: &str) -> Moon {
    let mut sections = line[1..line.len()-1].split(',');
    let x = &sections.next().unwrap()[2..];
    let y = &sections.next().unwrap()[3..];
    let z = &sections.next().unwrap()[3..];
    return Moon { 
        position: Position { 
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap(),
            z: z.parse::<i32>().unwrap()
        },
        velocity: Velocity { x: 0, y: 0, z: 0 }
    };
}

fn print_moon(moon : &Moon) {
    println!("pos=<x={:?}, y={:?}, z=>{:?}, vel=<x={:?}, y={:?}, z={:?}>",
        moon.position.x, moon.position.y, moon.position.z,
        moon.velocity.x, moon.velocity.y, moon.velocity.z);
}

fn print_moons(moons : &Vec<Moon>) {
    for moon in moons {
        print_moon(moon);
    }
    println!("");
}

fn velocity(a: i32, b: i32) -> i32 {
  if a > b {
    return -1;
  } else if a < b {
    return 1;
  } else {
    return 0;
  }
}

fn state_check(states: &mut HashMap<Vec<(i32, i32)>, (usize, usize)>, step: usize, axis: usize, posvels: Vec<(i32, i32)>) -> Option<usize> {
    let entry = states.entry(posvels.clone());
    match entry {
        Entry::Occupied(occupied_entry) => {
            println!("Repeated {:?} with {:?} appearing at {:?} and {:?}", posvels, axis, occupied_entry.get(), step);
            return Some(step);
        },
        Entry::Vacant(vacant_entry) => {
            vacant_entry.insert((axis, step));
            return None;
        }
    }
}

fn main() -> io::Result<()> {
    let mut moons : Vec<Moon> = Vec::new();
    for line in io::stdin().lock().lines() {
        moons.push(parse_moon(&line?));
    }

    let mut states : HashMap<Vec<(i32, i32)>, (usize, usize)> = HashMap::new();
    let mut done_x = None;
    let mut done_y = None;
    let mut done_z = None;
    state_check(&mut states, 0, 0, moons.iter().map(|moon| (moon.position.x, moon.velocity.x)).collect());
    state_check(&mut states, 0, 1, moons.iter().map(|moon| (moon.position.y, moon.velocity.y)).collect());
    state_check(&mut states, 0, 2, moons.iter().map(|moon| (moon.position.z, moon.velocity.z)).collect());
    

    let mut step = 1;
    while (done_x.is_none() || done_y.is_none() || done_z.is_none()) {
        if step % 50000 == 0 {
            println!("{:?}", step);
        }
        for moon_a_index in 0..moons.len() {
            for moon_b_index in (moon_a_index+1)..moons.len() {
                let mut moon_a = moons[moon_a_index].clone();
                let mut moon_b = moons[moon_b_index].clone();
                let x_velocity = velocity(moon_a.position.x, moon_b.position.x);
                let y_velocity = velocity(moon_a.position.y, moon_b.position.y);
                let z_velocity = velocity(moon_a.position.z, moon_b.position.z);
                moon_a.velocity.x += x_velocity;
                moon_b.velocity.x -= x_velocity;
    
                moon_a.velocity.y += y_velocity;
                moon_b.velocity.y -= y_velocity;
    
                moon_a.velocity.z += z_velocity;
                moon_b.velocity.z -= z_velocity;
    
                moons[moon_a_index] = moon_a;
                moons[moon_b_index] = moon_b;
            }
        }
        for mut moon in &mut moons {
            moon.position.x += moon.velocity.x;
            moon.position.y += moon.velocity.y;
            moon.position.z += moon.velocity.z;
        }

        if (done_x.is_none()) {
            done_x = state_check(&mut states, step, 0, moons.iter().map(|moon| (moon.position.x, moon.velocity.x)).collect());
        }
        if (done_y.is_none()) {
            done_y = state_check(&mut states, step, 1, moons.iter().map(|moon| (moon.position.y, moon.velocity.y)).collect());
        }
        if (done_z.is_none()) {
            done_z = state_check(&mut states, step, 2, moons.iter().map(|moon| (moon.position.z, moon.velocity.z)).collect());
        }
        step += 1;
    }
    let cycle_x : BigUint = (done_x.unwrap() as u32).into();
    let cycle_y : BigUint = (done_y.unwrap() as u32).into();
    let cycle_z : BigUint = (done_z.unwrap() as u32).into();
    println!("{:?}", cycle_x.lcm(&cycle_y).lcm(&cycle_z).to_string());
    


    Ok(())
}
