use std::io::{self, BufRead};

#[derive(Clone,Debug)]
struct Velocity { x : i32, y : i32, z : i32 }
#[derive(Clone,Debug)]
struct Position { x : i32, y : i32, z : i32 }

#[derive(Clone,Debug)]
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

fn velocity(a: i32, b: i32) -> i32 {
  if a > b {
    return -1;
  } else if a < b {
    return 1;
  } else {
    return 0;
  }
}

fn main() -> io::Result<()> {
    let mut moons : Vec<Moon> = Vec::new();
    for line in io::stdin().lock().lines() {
        moons.push(parse_moon(&line?));
    }

    for step in (0..1000) {
        for moon_a_index in (0..moons.len()) {
            for moon_b_index in ((moon_a_index+1)..moons.len()) {
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
        println!("After {:?} steps: ", step+1);
        for mut moon in &mut moons {
            moon.position.x += moon.velocity.x;
            moon.position.y += moon.velocity.y;
            moon.position.z += moon.velocity.z;
            println!("pos=<x={:?}, y={:?}, z=>{:?}, vel=<x={:?}, y={:?}, z={:?}>",
                moon.position.x, moon.position.y, moon.position.z,
                moon.velocity.x, moon.velocity.y, moon.velocity.z);
        }
    }
    let total_energy : i32 = moons.iter().map(|moon| (moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs()) * (moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs())).sum();
    println!("{:?}", total_energy);


    Ok(())
}
