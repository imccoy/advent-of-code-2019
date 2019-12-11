use std::io::{self, BufRead,Write};
use std::convert::TryInto;
use std::collections::HashMap;

mod intcode;

#[derive(Clone,Copy,Debug)]
enum Direction { North, South, East, West }

#[derive(Clone,Copy,Debug)]
enum Turn { Clockwise, AntiClockwise }

#[derive(Clone,Copy,Debug)]
enum Colour { Black, White }

type Painting = HashMap<(i32, i32), Colour>;

struct PaintingRobot { position : (i32, i32), direction : Direction, painting : Painting }

fn code_for_colour(colour: Colour) -> usize {
    match colour {
        Colour::Black => 0,
        Colour::White => 1
    }
}

fn colour_for_code(code: usize) -> Colour {
    match code {
        0 => Colour::Black,
        _ => Colour::White
    }
}

fn turn_for_code(code: usize) -> Turn {
    match code {
        0 => Turn::AntiClockwise,
        _ => Turn::Clockwise
    }
}

fn move_robot((x, y) : (i32, i32), direction : Direction) -> (i32, i32) {
    match direction {
        Direction::North => (x, y-1),
        Direction::South => (x, y+1),
        Direction::East => (x+1, y), 
        Direction::West => (x-1, y)
    }
}

fn turn(direction: Direction, turn: Turn) -> Direction {
    match turn {
        Turn::Clockwise => match direction {
                               Direction::North => Direction::East,
                               Direction::East => Direction::South,
                               Direction::South => Direction::West,
                               Direction::West => Direction::North
                           },
        Turn::AntiClockwise => match direction {
                                   Direction::North => Direction::West,
                                   Direction::West => Direction::South,
                                   Direction::South => Direction::East,
                                   Direction::East => Direction::North
                               }
    }
}

fn draw_painting(painting: Painting) -> io::Result<()> {
    let xmin : i32 = *painting.keys().map(|(x,_)| x).min().unwrap();
    let xmax : i32 = *painting.keys().map(|(x,_)| x).max().unwrap();
    let ymin : i32 = *painting.keys().map(|(_,y)| y).min().unwrap();
    let ymax : i32 = *painting.keys().map(|(_,y)| y).max().unwrap();
    dbg!(xmin, xmax, ymin, ymax);

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let colour = painting.get(&(x, y)).unwrap_or(&Colour::White);
            io::stdout().write(match colour { Colour::Black => b"#", Colour::White => b" " })?;
        }
        io::stdout().write(b"\n")?;
    }
    io::stdout().write(b"\n\n")?;


    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let colour = painting.get(&(xmax - x, y)).unwrap_or(&Colour::White);
            io::stdout().write(match colour { Colour::Black => b"#", Colour::White => b" " })?;
        }
        io::stdout().write(b"\n")?;
    }
    io::stdout().write(b"\n\n")?;

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let colour = painting.get(&(x, ymax - y)).unwrap_or(&Colour::White);
            io::stdout().write(match colour { Colour::Black => b"#", Colour::White => b" " })?;
        }
        io::stdout().write(b"\n")?;
    }
    io::stdout().write(b"\n\n")?;

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let colour = painting.get(&(xmax - x, ymax - y)).unwrap_or(&Colour::White);
            io::stdout().write(match colour { Colour::Black => b"#", Colour::White => b" " })?;
        }
        io::stdout().write(b"\n")?;
    }
    io::stdout().write(b"\n\n")?;

    for x in xmin..=xmax {
        for y in ymin..=ymax {
            let colour = painting.get(&(x, y)).unwrap_or(&Colour::White);
            io::stdout().write(match colour { Colour::Black => b"#", Colour::White => b" " })?;
        }
        io::stdout().write(b"   ")?;

        for y in ymin..=ymax {
            let colour = painting.get(&(x, ymax - y)).unwrap_or(&Colour::White);
            io::stdout().write(match colour { Colour::Black => b"#", Colour::White => b" " })?;
        }
        io::stdout().write(b"   ")?;

        for y in ymin..=ymax {
            let colour = painting.get(&(xmax - x, y)).unwrap_or(&Colour::White);
            io::stdout().write(match colour { Colour::Black => b"#", Colour::White => b" " })?;
        }
        io::stdout().write(b"   ")?;

        for y in ymin..=ymax {
            let colour = painting.get(&(xmax - x, ymax - y)).unwrap_or(&Colour::White);
            io::stdout().write(match colour { Colour::Black => b"#", Colour::White => b" " })?;
        }
        io::stdout().write(b"   ")?;

        io::stdout().write(b"\n")?;
    }
    io::stdout().write(b"\n\n")?;

    for x in xmin..=xmax {
        for _ in 0..(xmax - x) {
            io::stdout().write(b" ")?;
        }
        for y in ymin..=ymax {
            let colour = painting.get(&(x, y)).unwrap_or(&Colour::White);
            io::stdout().write(match colour { Colour::Black => b"#", Colour::White => b" " })?;
        }
        io::stdout().write(b"\n")?;
    }
    io::stdout().write(b"\n\n")?;



    Ok(())
}

fn main() -> io::Result<()> {
    let program_string = io::stdin().lock().lines().next().unwrap()?;
    let program : Vec<i64> = program_string.split(',')
        .filter(|line| line.len() != 0)
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut machine = intcode::Machine { program: program.clone(), pc: 0, relative_base: 0 };
    let mut robot = PaintingRobot { position: (0,0), direction: Direction::North, painting: HashMap::new() };
    robot.painting.insert((0,0), Colour::White);
    loop {
        let current_colour = robot.painting.get(&robot.position).unwrap_or(&Colour::Black);

        match intcode::run_program(&mut machine, [code_for_colour(*current_colour) as i64].iter()) {
            Ok(Some(new_colour_code)) => {
                match intcode::run_program(&mut machine, [].iter()) {
                    Ok(Some(turning_code)) => {
                        robot.painting.insert(robot.position, colour_for_code(new_colour_code.try_into().unwrap()));
                        robot.direction = turn(robot.direction, turn_for_code(turning_code.try_into().unwrap()));
                        robot.position = move_robot(robot.position, robot.direction);
                    },
                    Err(message) => println!("I failed {}", message),
                    Ok(None) => break
                }
            }
            Err(message) => println!("I failed {}", message),
            Ok(None) => break
        }
    }
    println!("{:?}", robot.painting.len());
    draw_painting(robot.painting)?;
    Ok(())
}
