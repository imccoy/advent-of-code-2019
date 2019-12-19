use std::io::{self,BufRead,Write};
use std::collections::HashMap;
use std::collections::HashSet;

extern crate num_derive;
use num_derive::FromPrimitive;

extern crate num_traits;
use num_traits::cast::FromPrimitive;

mod intcode;


// this one kind of kicked my ass and I don't know why.

#[derive(Debug, Eq, PartialEq)]
enum Space { Scaffold, Void }

#[derive(Clone,Copy,Debug, Eq, PartialEq)]
enum Direction { North, South, East, West }

type Map = Vec<Vec<Space>>;

fn draw_map(map: &Map, ((robot_x, robot_y), robot_direction) : ((usize, usize), Direction)) -> io::Result<()> {
    for (row_num, row) in map.iter().enumerate() {
        for (col_num, col) in row.iter().enumerate() {
            if row_num == robot_y && col_num == robot_x {
                io::stdout().write(match robot_direction { 
                    Direction::North => b"^",
                    Direction::South => b"v",
                    Direction::East => b"<",
                    Direction::West => b">",
                })?;
            } else {
                io::stdout().write(match col { 
                    Space::Scaffold => b"#",
                    Space::Void => b".",
                })?;
            }
        }
        io::stdout().write(b"\n")?;
    }
    io::stdout().write(b"\n\n")?;


    Ok(())
}

fn load_map(program: Vec<i64>) -> (Map, ((usize, usize), Direction)) {
    let mut map : Map = Vec::new();
    let mut current_row : Vec<Space> = Vec::new();
    let mut robot_state : Option<((usize, usize), Direction)> = None;
    for output in intcode::output_iter(program) {
        match output {
            10 => {
                map.push(current_row);
                current_row = Vec::new();
            },
            35 => {
                current_row.push(Space::Scaffold);
            },
            46 => {
                current_row.push(Space::Void);
            },
            94 => { // robot facing up
                robot_state = Some(((current_row.len(), map.len()), Direction::North));
                current_row.push(Space::Scaffold);
            },
            60 => { // robot facing left
                current_row.push(Space::Scaffold);
            },
            62 => { // robot facing right
                current_row.push(Space::Scaffold);
            },
            118 => { // robot facing down
                current_row.push(Space::Scaffold);
            },
            o => panic!("unknown output {:?}", o),
        }
    }
    return (map, robot_state.unwrap());
}

#[derive(Clone,Copy,Debug, Eq, PartialEq)]
enum TurnDirection { Left, Right }

#[derive(Clone,Copy,Debug, Eq, PartialEq)]
enum PathElement { Turn(TurnDirection), Distance(usize) }


fn turn_to(from: Direction, to: Direction) -> Vec<TurnDirection> {
    match (from, to) {
        (Direction::North, Direction::North) => vec![],
        (Direction::North, Direction::South) => vec![TurnDirection::Left, TurnDirection::Left],
        (Direction::North, Direction::East) => vec![TurnDirection::Right],
        (Direction::North, Direction::West) => vec![TurnDirection::Left],

        (Direction::South, Direction::South) => vec![],
        (Direction::South, Direction::North) => vec![TurnDirection::Left, TurnDirection::Left],
        (Direction::South, Direction::West) => vec![TurnDirection::Right],
        (Direction::South, Direction::East) => vec![TurnDirection::Left],

        (Direction::East, Direction::East) => vec![],
        (Direction::East, Direction::West) => vec![TurnDirection::Left, TurnDirection::Left],
        (Direction::East, Direction::South) => vec![TurnDirection::Right],
        (Direction::East, Direction::North) => vec![TurnDirection::Left],

        (Direction::West, Direction::West) => vec![],
        (Direction::West, Direction::East) => vec![TurnDirection::Left, TurnDirection::Left],
        (Direction::West, Direction::North) => vec![TurnDirection::Right],
        (Direction::West, Direction::South) => vec![TurnDirection::Left]
    }
}

fn map_path(map: &Map, (start_x, start_y) : (usize, usize), start_direction : Direction) -> Vec<PathElement> {
    let mut edges : Vec<PathElement> = Vec::new();
    let width = map[0].len();
    let height = map.len() - 1;
    let mut x = start_x;
    let mut y = start_y;
    let mut last_direction : Direction = start_direction;
    loop {
        if last_direction != Direction::North && last_direction != Direction::South {
            if y > 0 && map[y-1][x] == Space::Scaffold {
                let mut y1 = y;
                while y1 >= 1 && map[y1-1][x] == Space::Scaffold  {
                    y1 -= 1;
                }
                for turn in turn_to(last_direction, Direction::North) { edges.push(PathElement::Turn(turn)); }
                edges.push(PathElement::Distance(y - y1));
                last_direction = Direction::North;
                y = y1;
                continue;
            }
            if y < height - 1 && map[y+1][x] == Space::Scaffold {
                let mut y1 = y;
                while y1 + 1 <= height - 1 && map[y1+1][x] == Space::Scaffold  {
                    y1 += 1;
                }
                for turn in turn_to(last_direction, Direction::South) { edges.push(PathElement::Turn(turn)); }
                edges.push(PathElement::Distance(y1 - y));
                last_direction = Direction::South;
                y = y1;
                continue;
            }
        } 
        if last_direction != Direction::East && last_direction != Direction::West {
            if x > 0 && map[y][x-1] == Space::Scaffold {
                let mut x1 = x;
                while x1 >= 1 && map[y][x1-1] == Space::Scaffold  {
                    x1 -= 1;
                }
                for turn in turn_to(last_direction, Direction::West) { edges.push(PathElement::Turn(turn)); }
                edges.push(PathElement::Distance(x - x1));
                last_direction = Direction::West;
                x =  x1;
                continue;
            }
            if x < width - 1 && map[y][x+1] == Space::Scaffold {
                let mut x1 = x;
                while x1 + 1 <= width - 1 && map[y][x1+1] == Space::Scaffold  {
                    x1 += 1;
                }
                for turn in turn_to(last_direction, Direction::East) { edges.push(PathElement::Turn(turn)); }
                edges.push(PathElement::Distance(x1 - x));
                last_direction = Direction::East;
                x =  x1;
                continue;
            }
        }
        break;
    }
 
    return edges;
}

fn section(path: &Vec<PathElement>, min: usize, max : usize) -> Vec<usize> {
    let mut result : Vec<usize> = Vec::new();
    for i in 0..=path.len()-(max - min) {
        if path[i..i+(max-min)] == path[min..max] {
            result.push(i);
        }
    }
    return result;
}

fn add_commands<T: Copy+std::fmt::Debug, F:Fn(T) -> Vec<i64>>(commands: &mut Vec<i64>, src: &[T], prepare: F) {
    println!("{:?}", &src);
    for command_char in prepare(src[0]) {
        commands.push(command_char);
    }

    for src_item in src[1..].iter() {
        commands.push(44);
        for command_char in prepare(*src_item) {
            commands.push(command_char);
        }
    }
    commands.push(10);
}

fn path_element_chars(path_element: PathElement) -> Vec<i64> {
    match path_element {
        PathElement::Turn(TurnDirection::Left) => vec![76],
        PathElement::Turn(TurnDirection::Right) => vec![82],
        PathElement::Distance(mut distance) => {
            let mut chars : Vec<i64> = Vec::new();
            while distance != 0 {
                chars.insert(0, (48 + distance % 10) as i64);
                distance /= 10;
            }
            return chars;
        }
    }
}

fn main() -> io::Result<()> {
    let program_string = io::stdin().lock().lines().next().unwrap()?;
    let program : Vec<i64> = program_string.split(',')
        .filter(|line| line.len() != 0)
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let (map, (robot_position, robot_direction)) = load_map(program.clone());
    let width = map[0].len();
    let height = map.len() - 1;
    let crossing_points = (0..width).flat_map(|x| (0..height).map(move |y| (x,y)))
        .filter_map(|(x, y)| { 
            if x+1 >= width || y+1 >= height || x == 0 || y == 0 {
                return None;
            }
            if map[y-1][x] == Space::Scaffold &&
                    map[y+1][x] == Space::Scaffold &&
                    map[y][x-1] == Space::Scaffold &&
                    map[y][x-1] == Space::Scaffold &&
                    map[y][x] == Space::Scaffold {
               return Some((x, y));
           } else {
               return None;
           }
        });
    println!("{:?}", crossing_points.map(|(x, y)| x * y).sum::<usize>());
    draw_map(&map, (robot_position, robot_direction));

    let path = map_path(&map, robot_position, robot_direction);
    println!("{:?} {:?}", path.len(), &path);

    let mut starts : Vec<(usize, usize)> = Vec::new();
    // I figured out these values (0-8, 8-16, 24-30) with a version of the `section` function
    // that logged out when it found the section, and a bit of trial and error
    for start in section(&path, 0, 8) {
        starts.push((start, 0));
    }
    for start in section(&path, 8, 16) {
        starts.push((start, 1));
    }
    for start in section(&path, 24, 30) {
        starts.push((start, 2));
    }
    starts.sort();
    let mut commands = Vec::new();
    println!("{:?}", commands.len());
    add_commands(&mut commands, &starts, |start| vec![(start.1 + 65) as i64]);
    println!("{:?}", commands.len());
    add_commands(&mut commands, &path[0..8], |path_element| path_element_chars(path_element));
    println!("{:?}", commands.len());
    add_commands(&mut commands, &path[8..16], |path_element| path_element_chars(path_element));
    println!("{:?}", commands.len());
    add_commands(&mut commands, &path[24..30], |path_element| path_element_chars(path_element));
    println!("{:?}", commands.len());
    commands.push(110); // n for no video feed
    commands.push(10);
    println!("{:?}", commands);
    let mut command_str = String::new();
    for command in &commands {
        command_str.push(std::char::from_u32(*command as u32).unwrap());
    }
    println!("{:?}", command_str);
    let mut movement_program = program.clone();
    movement_program[0] = 2;
    let mut machine : intcode::Machine = intcode::Machine { program: movement_program, pc: 0, relative_base: 0 };
    let mut iter = commands.iter();
    loop {
        match intcode::run_program(&mut machine, &mut iter) {
            Ok(Some(v)) => println!("{:?}", v),
            Ok(None) => break,
            Err(_) => panic!("I failed")
        }
    }


    Ok(())
}
