use std::io::{self, BufRead};
use std::cmp::min;

fn read_map() -> Result<Vec<(i32, i32)>, std::io::Error> {
    let mut asteroids : Vec<(i32, i32)> = Vec::new();
    for (row, line) in io::stdin().lock().lines().enumerate() {
        for (col, char) in line?.chars().enumerate() {
            if char == '#' {
                asteroids.push((col as i32, row as i32));
            }
        }
    }
    return Ok(asteroids);
}

fn normalize_direction((x, y) : (i32, i32)) -> (i32, i32) {
    let divisor = (2..=(min(x.abs(), y.abs()))).rev().filter(|divisor| { x % divisor == 0 && y % divisor == 0 }).next().unwrap_or(1);
    return (x / divisor, y / divisor);
}

fn asteroid_blocked((asteroid_x, asteroid_y) : (i32, i32), (start_x, start_y): (i32, i32), (potential_blocker_x, potential_blocker_y) : (i32, i32)) -> bool {
   let (distance_to_blocker_x, distance_to_blocker_y) = (potential_blocker_x - start_x, potential_blocker_y - start_y);
   let (distance_to_subject_x, distance_to_subject_y) = (asteroid_x - start_x, asteroid_y - start_y);

   let same_x = start_x == asteroid_x && start_x == potential_blocker_x;
   let same_y = start_y == asteroid_y && start_y == potential_blocker_y;

   let out_past_x = (distance_to_blocker_x < 0 && distance_to_subject_x < distance_to_blocker_x) || (0 < distance_to_blocker_x && distance_to_blocker_x < distance_to_subject_x);
   let out_past_y = (distance_to_blocker_y < 0 && distance_to_subject_y < distance_to_blocker_y) || (0 < distance_to_blocker_y && distance_to_blocker_y < distance_to_subject_y);

   if same_x && out_past_y {
       return true;
   } else if same_y && out_past_x {
       return true;
   } else if out_past_x && out_past_y {
       let (direction_x, direction_y) = normalize_direction((distance_to_blocker_x, distance_to_blocker_y));
       return (distance_to_subject_x % direction_x == 0) && (distance_to_subject_y % direction_y == 0) && (distance_to_subject_x / direction_x == distance_to_subject_y / direction_y);
   } else {
       return false;
   }
}

fn distance((x, y) : (i32, i32), (start_x, start_y) : (i32, i32)) -> usize {
   return ((start_x - x).abs().pow(2) + (start_y - y).abs().pow(2)) as usize;
}

fn suitability(asteroids: &Vec<(i32, i32)>, start : (i32, i32)) -> usize {
    let mut sighted : Vec<(i32, i32)> = Vec::new();
    let mut sorted = asteroids.clone();
    sorted.sort_by_key(|asteroid| distance(*asteroid, start));
    for asteroid in sorted {
        if asteroid == start {
            continue;
        }

        if sighted.iter().find(|potential_blocker| asteroid_blocked(asteroid, start, **potential_blocker)).is_none() {
            sighted.push(asteroid);
        }
   }
   return sighted.len();
}

fn suitabilities(asteroids : &Vec<(i32, i32)>) -> Vec<(&(i32, i32), usize)> {
   asteroids.iter().map(|asteroid| (asteroid, suitability(asteroids, *asteroid))).collect()
}

fn main() -> io::Result<()> {
    let map = read_map()?;
    println!("{:?}", suitabilities(&map).iter().max_by_key(|suitability_row| suitability_row.1));
    Ok(())
}
