use std::io::{self, BufRead,Write};
use std::collections::HashMap;

extern crate num_derive;
use num_derive::FromPrimitive;

extern crate num_traits;
use num_traits::FromPrimitive;

mod intcode;

#[derive(FromPrimitive, Eq, PartialEq, Clone)]
enum Tile { Empty = 0, Wall = 1, Block = 2, HorizontalPaddle = 3, Ball = 4 }


type Screen = HashMap<(i32, i32), Tile>;


fn draw_screen(screen: &Screen, score: usize) -> io::Result<()> {
    let xmin : i32 = *screen.keys().map(|(x,_)| x).min().unwrap();
    let xmax : i32 = *screen.keys().map(|(x,_)| x).max().unwrap();
    let ymin : i32 = *screen.keys().map(|(_,y)| y).min().unwrap();
    let ymax : i32 = *screen.keys().map(|(_,y)| y).max().unwrap();

    println!("SCORE: {:?}", score);
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let tile = screen.get(&(x, y)).unwrap_or(&Tile::Empty);
            io::stdout().write(match tile { 
                Tile::Empty => b" ",
                Tile::Wall => b"*",
                Tile::Block => b"#",
                Tile::HorizontalPaddle => b"=",
                Tile::Ball => b"o"
            })?;
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
    let mut screen_drawn : Screen = HashMap::new();
    let mut screen : Screen = HashMap::new();
    let mut score : usize = 0;
    let mut ball_x : i64 = 0;
    let mut paddle_x : i64 = 0;
    let mut current_input : i64 = 0;
    machine.program[0] = 2;
    loop {
        match intcode::run_program(&mut machine, [current_input].iter()) {
            Ok(Some(x)) => {
                match intcode::run_program(&mut machine, [current_input].iter()) {
                    Ok(Some(y)) => {
                        match intcode::run_program(&mut machine, [current_input].iter()) {
                            Ok(Some(tile_number_or_score)) => {
                                if x == -1 && y == 0 {
                                    score = tile_number_or_score as usize;
                                } else {
                                    let tile_number = tile_number_or_score;
                                    let tile : Tile = num_traits::cast::FromPrimitive::from_i64(tile_number).unwrap();
                                    match tile {
                                        Tile::Ball => { 
                                            ball_x = x;
                                            current_input = (ball_x - paddle_x).signum();
                                        },
                                        Tile::HorizontalPaddle => {
                                            paddle_x = x;
                                            current_input = (ball_x - paddle_x).signum();
                                        },
                                        _ => {},
                                    }
                                    screen.insert((x as i32, y as i32), tile);
                                }
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
        if (false && screen_drawn != screen) {
            draw_screen(&screen, score)?;
            screen_drawn = screen.clone();
        }
    }
    println!("{:?}", score);
    Ok(())
}
