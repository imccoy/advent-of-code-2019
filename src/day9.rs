use std::io::{self, BufRead};

mod intcode;

fn main() -> io::Result<()> {
    let program_string = io::stdin().lock().lines().next().unwrap()?;
    let program : Vec<i64> = program_string.split(',')
        .filter(|line| line.len() != 0)
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut machine = intcode::Machine { program: program.clone(), pc: 0, relative_base: 0 };
    loop {
        match intcode::run_program(&mut machine, [2].iter()) {
            Ok(Some(output)) => println!("{}", output),
            Err(message) => println!("I failed {}", message),
            Ok(None) => break
        }
    }
    Ok(())
}
