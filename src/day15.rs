use std::io::{self,BufRead,Write};
use std::collections::HashMap;
use std::collections::HashSet;

extern crate num_derive;
use num_derive::FromPrimitive;

extern crate num_traits;
use num_traits::cast::FromPrimitive;

mod intcode;

#[derive(Copy,Clone,FromPrimitive, Eq, PartialEq,Debug)]
enum Cell { Wall = 0, Clear = 1, OxygenSystem = 2 }

#[derive(Copy,Clone,Debug)]
enum Direction { North = 1, South = 2, West = 3, East = 4 }


fn draw_map(position: (i32, i32), map: &HashMap<(i32, i32), Cell>) -> io::Result<()> {
    let xmin : i32 = *map.keys().map(|(x,_)| x).min().unwrap();
    let xmax : i32 = *map.keys().map(|(x,_)| x).max().unwrap();
    let ymin : i32 = *map.keys().map(|(_,y)| y).min().unwrap();
    let ymax : i32 = *map.keys().map(|(_,y)| y).max().unwrap();

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if x == position.0 && y == position.1 {
                io::stdout().write(b"R")?;
            } else {
                let cell = map.get(&(x, y));
                io::stdout().write(match cell { 
                    Some(Cell::Wall) => b"#",
                    Some(Cell::Clear) => b" ",
                    Some(Cell::OxygenSystem) => b"o",
                    None => b"?"
                })?;
            }
        }
        io::stdout().write(b"\n")?;
    }
    io::stdout().write(b"\n\n")?;


    Ok(())
}



fn after_move((position_x, position_y) : (i32, i32), direction: Direction) -> (i32, i32) {
    match direction {
        Direction::North => (position_x, position_y - 1),
        Direction::South => (position_x, position_y + 1),
        Direction::West => (position_x - 1, position_y),
        Direction::East => (position_x + 1, position_y)
    }
}

fn shortest_path(map: &HashMap<(i32, i32), Cell>, position: (i32, i32), mut except: &mut HashSet<(i32, i32)>, goal: Option<Cell>) -> Option<(Direction, i32)> {
    let directions = [Direction::North, Direction::South, Direction::West, Direction::East];
    let directions_unexplored_distances : Vec<Option<(Direction, i32)>> = directions.iter().map(|direction| {
        let new_position = after_move(position, *direction);
        let cell = map.get(&new_position);
        if cell.map(|c| *c) == goal {
            return Some((*direction, 0));
        }
         
        match map.get(&new_position) {
            Some(Cell::Wall) => None,
            _ => {
                if !except.contains(&new_position) {
                    except.insert(position);
                    let next = shortest_path(map, new_position, &mut except, goal);
                    except.remove(&position);
                    next.map(|(_, distance)| (*direction, distance + 1))
                } else {
                    None
                }
           }
        }
    }).collect();
    return directions_unexplored_distances.iter()
        .filter_map(|x| x.as_ref())
        .min_by_key(|(_, distance)| *distance)
        .map(|x| *x);
}

fn direction_to_nearest_unexplored(map: &HashMap<(i32, i32), Cell>, position: (i32, i32)) -> Option<Direction> {
    let mut already_visited : HashSet<(i32,i32)> = HashSet::new();
    return shortest_path(map, position, &mut already_visited, None)
        .map(|(direction, _)| direction);
}

fn distance_to_oxygen_system(map: &HashMap<(i32, i32), Cell>, position: (i32, i32)) -> Option<i32> {
    let mut already_visited : HashSet<(i32,i32)> = HashSet::new();
    return shortest_path(map, position, &mut already_visited, Some(Cell::OxygenSystem))
        .map(|(_, distance)| distance);
}

fn main() -> io::Result<()> {
    let program_string = io::stdin().lock().lines().next().unwrap()?;
    let program : Vec<i64> = program_string.split(',')
        .filter(|line| line.len() != 0)
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut machine = intcode::Machine { program: program.clone(), pc: 0, relative_base: 0 };
    let mut map : HashMap<(i32, i32), Cell> = HashMap::new();
    let mut position = (0,0);
    loop {
        match direction_to_nearest_unexplored(&map, position) {
            Some(direction) => {
                match intcode::run_program(&mut machine, [direction as i64].iter()) {
                    Ok(Some(cell)) => {
                        let cell = FromPrimitive::from_i64(cell).unwrap();
                        let move_position = after_move(position, direction);

                        map.insert(move_position, cell);
                        position = if cell != Cell::Wall { move_position} else { position };
                    },
                    Err(message) => println!("I failed {}", message),
                    Ok(None) => break
                }
            },
            None => break
        };
    }
    draw_map((0,0), &map);
    println!("1 + {:?}", distance_to_oxygen_system(&map, (0, 0)));

    let empty_cells : HashSet<(i32, i32)> = map.iter().filter_map(|(position, cell)| if *cell == Cell::Clear { Some(*position) } else { None }).collect();
    let mut oxygen_set : HashSet<(i32, i32)> = map.iter().filter_map(|(position, cell)| if *cell == Cell::OxygenSystem { Some(*position) } else { None }).collect();
    let mut iterations = 0;
    while empty_cells.len() != oxygen_set.len() {
        let mut oxygen_set_neighbours : HashSet<(i32, i32)> = HashSet::new();
        for position in &oxygen_set {
            let directions = [Direction::North, Direction::South, Direction::West, Direction::East];
            for direction in &directions {
                oxygen_set_neighbours.insert(after_move(*position, *direction));
            }
        }
        for position in empty_cells.intersection(&oxygen_set_neighbours) {
            oxygen_set.insert(*position);
        }
        iterations += 1;
    }
    println!("it took 1 + {:?}", iterations);
    Ok(())
}
