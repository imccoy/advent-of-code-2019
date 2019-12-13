use std::io::{self, BufRead};
use std::cmp::min;

const MAP_WIDTH : i32 = 24;
const MAP_HEIGHT : i32 = 24;

const STATION_X : i32 = 14;
const STATION_Y : i32 = 17;

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
    if x == 0  {
        return (0, 1);
    } else if y == 0 {
        return (1, 0);
    } else {
        let divisor = (2..=(min(x.abs(), y.abs()))).rev().filter(|divisor| { x % divisor == 0 && y % divisor == 0 }).next().unwrap_or(1);
        return (x / divisor, y / divisor);
    }
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

fn station_relative((x, y) : (i32, i32)) -> (i32, i32) {
    return (x - STATION_X, y - STATION_Y);
}

fn scan_order((angle_x, angle_y): (i32, i32)) -> f64 {
    let (float_angle_x, float_angle_y) : (f64, f64) = (angle_x.into(), angle_y.into());
    let angle : f64 = (float_angle_y / float_angle_x).atan();
    if angle_x > 0 {
        if angle_y < 0 {
            return 10.0 + angle; // add 10 to make sure all results are positive while preserving the order
        } else {
            return 20.0 + angle; // add 10 to make sure all results are positive while preserving the order
        }
    } else if angle_x < 0 {
        if angle_y < 0 {
            return 40.0 + angle; // add 10 to make sure all results are positive while preserving the order
        } else {
            return 30.0 + angle; // add 10 to make sure all results are positive while preserving the order
        }
    } else {
        if angle_y < 0 {
            return 5.0; // add 10 to make sure all results are positive while preserving the order
        } else {
            return 25.0; // add 10 to make sure all results are positive while preserving the order
        }
    }
}

fn group_by_angles(asteroids: &Vec<(i32, i32)>) -> Vec<Vec<&(i32, i32)>> {
    match asteroids.first() {
        None => Vec::new(),
        Some(first_asteroid) => {
            let mut angles : Vec<Vec<&(i32, i32)>> = Vec::new();

            let mut current_angle_asteroids : Vec<&(i32, i32)> = Vec::new();
            let mut current_angle : (i32, i32) = normalize_direction(station_relative(*first_asteroid));

            for asteroid in asteroids {
                let new_angle = normalize_direction(station_relative(*asteroid));
                if new_angle == current_angle {
                    println!("{:?} {:?} {:?} {:?}", current_angle, station_relative(*asteroid), scan_order(station_relative(*asteroid)), asteroid);
                    current_angle_asteroids.push(asteroid);
                } else {
                    println!("{:?} {:?} {:?} {:?}", new_angle, station_relative(*asteroid), scan_order(station_relative(*asteroid)), asteroid);
                    angles.push(current_angle_asteroids);
                    current_angle_asteroids = Vec::new();
                    current_angle_asteroids.push(asteroid);
                    current_angle = new_angle;
                }
            }
            if current_angle_asteroids.len() > 0 {
                angles.push(current_angle_asteroids);
            }
            return angles;
        }
    }
}

fn main() -> io::Result<()> {
    let mut asteroids = read_map()?;
    asteroids.sort_by(|a, b| scan_order(station_relative(*a)).partial_cmp(&scan_order(station_relative(*b))).unwrap_or(std::cmp::Ordering::Equal));
    let mut grouped_asteroid_sets = group_by_angles(&asteroids);
    dbg!(&grouped_asteroid_sets);
    for mut grouped_asteroid_set in &mut grouped_asteroid_sets {
        grouped_asteroid_set.sort_by_key(|asteroid| distance(**asteroid, (STATION_X, STATION_Y)));
    }
    dbg!(&grouped_asteroid_sets);
    let mut asteroid_number = 1;
    loop {
        for mut grouped_asteroid_set in &mut grouped_asteroid_sets {
            match grouped_asteroid_set.first() {
                None => { 
                }
                Some(asteroid) => {
                    println!("{:?} {:?}", asteroid_number, asteroid);
                    asteroid_number += 1;
                    if (asteroid_number > 200) {
                        return Ok(());
                    }
                    grouped_asteroid_set.remove(0);
                }
            }
        }
    }
    Ok(())
}
