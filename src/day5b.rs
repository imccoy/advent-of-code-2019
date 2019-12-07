use std::io::{self, BufRead};

#[derive(Debug)]
struct ReadDetails {
    address: usize
}

#[derive(Debug)]
struct PrintDetails {
    value: i32
}

#[derive(Debug)]
struct JumpDetails {
    target: usize
}

#[derive(Debug)]
struct PutDetails {
    address: usize,
    value: i32
}

#[derive(Debug)]
enum Command { Put(PutDetails), Read(ReadDetails), Print(PrintDetails), Jump(JumpDetails), Noop, Halt }

enum Operand {
  Immediate(i32),
  Position(usize)
}

struct BinResultOp { op1 : Operand, op2 : Operand, result : Operand }
struct BinOp { op1 : Operand, op2 : Operand }
struct UnOp { op : Operand }

enum Instruction {
    Add(BinResultOp),
    Multiply(BinResultOp),
    Print(UnOp),
    Read(UnOp),
    JumpIfTrue(BinOp),
    JumpIfFalse(BinOp),
    LessThan(BinResultOp),
    Equals(BinResultOp),
    Halt
}

fn operand_for_mode(mode : i32, value : i32) -> Result<Operand, String> {
    if mode == 0 {
        Ok(Operand::Position(value as usize))
    } else if mode == 1 {
        Ok(Operand::Immediate(value))
    } else {
        Err(format!("unknown operand mode {}", mode))
    }
}


fn parse_bin_op(program: &Vec<i32>, pc : usize, modes : i32) -> Result<BinOp, String> {
    let mode1 = modes % 10;
    let mode2 = (modes / 10) % 10;
    Ok(BinOp {
      op1: operand_for_mode(mode1, program[pc + 1])?, 
      op2: operand_for_mode(mode2, program[pc + 2])?
    })
}


fn parse_bin_result_op(program: &Vec<i32>, pc : usize, modes : i32) -> Result<BinResultOp, String> {
    let mode1 = modes % 10;
    let mode2 = (modes / 10) % 10;
    let mode3 = (modes / 100) % 10;
    Ok(BinResultOp {
      op1: operand_for_mode(mode1, program[pc + 1])?, 
      op2: operand_for_mode(mode2, program[pc + 2])?, 
      result: operand_for_mode(mode3, program[pc + 3])? 
    })
}

fn parse_un_op(program: &Vec<i32>, pc : usize, modes : i32) -> Result<UnOp, String> {
    let mode = modes % 10;
    Ok(UnOp {
      op: operand_for_mode(mode, program[pc + 1])?
    })
}

fn parse_instruction(program: &Vec<i32>, pc : usize) -> Result<(Instruction, usize), String> {
    let opcode = program[pc] % 100;
    let modes = program[pc] / 100;

    if opcode == 1 {
        Ok((Instruction::Add(parse_bin_result_op(program, pc, modes)?), 3))
    } else if opcode == 2 {
        Ok((Instruction::Multiply(parse_bin_result_op(program, pc, modes)?), 3))
    } else if opcode == 3 {
        Ok((Instruction::Read(parse_un_op(program, pc, modes)?), 1))
    } else if opcode == 4 {
        Ok((Instruction::Print(parse_un_op(program, pc, modes)?), 1))
    } else if opcode == 5 {
        Ok((Instruction::JumpIfTrue(parse_bin_op(program, pc, modes)?), 2))
    } else if opcode == 6 {
        Ok((Instruction::JumpIfFalse(parse_bin_op(program, pc, modes)?), 2))
    } else if opcode == 7 {
        Ok((Instruction::LessThan(parse_bin_result_op(program, pc, modes)?), 3))
    } else if opcode == 8 {
        Ok((Instruction::Equals(parse_bin_result_op(program, pc, modes)?), 3))
    } else if opcode == 99 {
        Ok((Instruction::Halt, 0))
    } else {
        Err(format!("unknown opcode {} at {}", opcode, pc))
    }
}

fn resolve_op(program: &Vec<i32>, op : Operand) -> i32 {
    match op {
        Operand::Immediate(value) => value,
        Operand::Position(position) => program[position]
    }
}

fn resolve_op_position(op : Operand) -> Result<usize, String> {
    match op {
        Operand::Immediate(value) => Err(format!("got Immediate operand {}", value)),
        Operand::Position(position) => Ok(position)
    }
}

fn run_bin_result_op<F>(program: &Vec<i32>, operands: BinResultOp, op: F) -> Result<Command, String> 
    where F: Fn(i32, i32) -> i32
{
    let op1 = resolve_op(program, operands.op1);
    let op2 = resolve_op(program, operands.op2);
    let dest = resolve_op_position(operands.result)?;
    Ok(Command::Put(PutDetails { address: dest, value: op(op1, op2) }))
}

fn run_line(program: &Vec<i32>, instruction: Instruction) -> Result<Command, String> {
    match instruction {
        Instruction::Add(operands) => {
            run_bin_result_op(program, operands, |op1, op2| op1 + op2)
        },
        Instruction::Multiply(operands) => {
            run_bin_result_op(program, operands, |op1, op2| op1 * op2)
        },
        Instruction::Read(operands) => {
            let op = resolve_op_position(operands.op)?;
            Ok(Command::Read(ReadDetails { address: op }))
        },
        Instruction::Print(operands) => {
            let op = resolve_op(program, operands.op);
            Ok(Command::Print(PrintDetails { value: op }))
        },
        Instruction::JumpIfTrue(operands) => {
            let op1 = resolve_op(program, operands.op1);
            let target = resolve_op(program, operands.op2);
            if op1 != 0 {
                Ok(Command::Jump(JumpDetails { target: target as usize }))
            } else {
                Ok(Command::Noop)
            }
        },
        Instruction::JumpIfFalse(operands) => {
            let op1 = resolve_op(program, operands.op1);
            let target = resolve_op(program, operands.op2);
            if op1 == 0 {
                Ok(Command::Jump(JumpDetails { target: target as usize }))
            } else {
                Ok(Command::Noop)
            }
        },
        Instruction::LessThan(operands) => {
            run_bin_result_op(program, operands, |op1, op2| {
                if op1 < op2 {
                    1
                } else {
                    0
                }
            })
        },
        Instruction::Equals(operands) => {
            run_bin_result_op(program, operands, |op1, op2| {
                if op1 == op2 {
                    1
                } else {
                    0
                }
            })
        },
        Instruction::Halt => {
            Ok(Command::Halt)
        }
    }
}

fn run_program(program: &mut Vec<i32>) -> Result<(), String> {
    let mut pc = 0;
    loop {
        let (instruction, instruction_length) = parse_instruction(&program, pc)?;
        let command = run_line(&program, instruction)?;
        let pc_increment = match command {
            Command::Jump(_) => 0,
            _ => instruction_length + 1
        };
        match command {
            Command::Put(put) => {
                program[put.address] = put.value;
            },
            Command::Print(print) => {
                println!("{}", print.value);
            },
            Command::Read(read) => {
                let line = io::stdin().lock().lines().next().unwrap().map_err(|e| format!("{:?}", e))?;
                program[read.address] = line.parse::<i32>().unwrap();
            },
            Command::Jump(jump) => { 
                pc = jump.target;
            },
            Command::Noop => { 
                // do nothing
            },
            Command::Halt => return Ok(())
        }
        pc += pc_increment;
    }
}

fn main() -> io::Result<()> {
    let program_string = io::stdin().lock().lines().next().unwrap()?;
    let mut program : Vec<i32> = program_string.split(',')
        .filter(|line| line.len() != 0)
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", program);
    match run_program(&mut program) {
        Ok(_) => println!("{:?}", program),
        Err(message) => println!("it ded {}", message)
    }
    Ok(())
}
