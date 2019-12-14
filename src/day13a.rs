use std::io::{self, BufRead,Write};
use std::convert::TryInto;
use std::collections::HashMap;

extern crate num_derive;
use num_derive::FromPrimitive;

extern crate num_traits;
use num_traits::FromPrimitive;

mod intcode;

#[derive(FromPrimitive, Eq, PartialEq)]
enum Tile { Empty = 0, Wall = 1, Block = 2, HorizontalPaddle = 3, Ball = 4 }


type Screen = HashMap<(i32, i32), Tile>;


fn main() -> io::Result<()> {
    let program_string = io::stdin().lock().lines().next().unwrap()?;
    let program : Vec<i64> = program_string.split(',')
        .filter(|line| line.len() != 0)
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut machine = intcode::Machine { program: program.clone(), pc: 0, relative_base: 0 };
    let mut screen : Screen = HashMap::new();
    loop {
        match intcode::run_program(&mut machine, [].iter()) {
            Ok(Some(x)) => {
                match intcode::run_program(&mut machine, [].iter()) {
                    Ok(Some(y)) => {
                        match intcode::run_program(&mut machine, [].iter()) {
                            Ok(Some(tile_number)) => {
                                screen.insert((x as i32, y as i32), num_traits::cast::FromPrimitive::from_i64(tile_number).unwrap());
                            },
                            Err(message) => println!("I failed {}", message),
                            Ok(None) => break
                        };
                    },
                    Err(message) => println!("I failed {}", message),
                    Ok(None) => break
                }
            }
            Err(message) => println!("I failed {}", message),
            Ok(None) => break
        }
    }
    println!("{:?}", screen.values().filter(|tile| **tile == Tile::Block).count());
    //draw_painting(robot.painting)?;
    Ok(())
}
