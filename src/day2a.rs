use std::io::{self, BufRead};


#[derive(Debug)]
struct Put {
    address: usize,
    value: usize 
}

fn parse_arith(program: &Vec<usize>, pc : usize) -> (usize, usize, usize) {
    let op1 = program[program[pc + 1]];
    let op2 = program[program[pc + 2]];
    let dest = program[pc + 3];
    return (op1, op2, dest);
}

fn run_line(program: &Vec<usize>, pc : usize) -> Result<Option<Put>, String> {
    let opcode = program[pc];
    if opcode == 1 {
        let (op1, op2, dest) = parse_arith(program, pc);
        return Ok(Some(Put { address: dest, value: op1 + op2 }));
    } else if opcode == 2 {
        let (op1, op2, dest) = parse_arith(program, pc);
        return Ok(Some(Put { address: dest as usize, value: op1 * op2 }));
    } else if opcode == 99 {
        return Ok(None);
    }
    return Err(format!("unknown opcode {} at {}", opcode, pc));
}

fn run_program(program: &mut Vec<usize>) -> Result<(), String> {
    let mut pc = 0;
    loop {
        match run_line(&program, pc)? {
            Some(put) => {
                println!("{:?}", put);
                program[put.address] = put.value
            },
            None => return Ok(())
        }
        pc += 4;
    }
}

fn main() -> io::Result<()> {
    let program_string = io::stdin().lock().lines().next().unwrap()?;
    let mut program : Vec<usize> = program_string.split(',')
        .filter(|line| line.len() != 0)
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    println!("{:?}", program);
    program[1] = 12;
    program[2] = 2;
    match run_program(&mut program) {
        Ok(_) => println!("{:?}", program),
        Err(message) => println!("it ded {}", message)
    }
    Ok(())
}
