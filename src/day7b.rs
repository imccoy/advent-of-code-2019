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

struct Machine {
    program: Vec<i32>,
    pc: usize
}

fn run_program(machine : &mut Machine, mut inputs: std::slice::Iter<i32>) -> Result<Option<i32>, String> 
{
    loop {
        let (instruction, instruction_length) = parse_instruction(&machine.program, machine.pc)?;
        let command = run_line(&machine.program, instruction)?;
        machine.pc += instruction_length + 1; // jump instructions will overwrite this
        match command {
            Command::Put(put) => {
                machine.program[put.address] = put.value;
            },
            Command::Print(print) => {
                return Ok(Some(print.value));
            },
            Command::Read(read) => {
                machine.program[read.address] = *inputs.next().unwrap();
            },
            Command::Jump(jump) => { 
                machine.pc = jump.target;
            },
            Command::Noop => { 
                // do nothing
            },
            Command::Halt => return Ok(None)
        }
    }
}

// settings is an array that maps setting values to the machine that has that setting.
fn explore<F: std::clone::Clone + Fn([i32;5]) -> Result<i32,String>>(settings: [i32;5], amp: usize, f: F) -> Result<i32, String> {
   if amp == 5 {
       let mut amp_settings : [i32;5] = [0;5];
       // transpose settings into an array that maps machines to their settings.
       for setting in 0..5 {
           let amp_index = settings[setting];
           amp_settings[amp_index as usize] = setting as i32;
       }
       return f(amp_settings);
   } else {
       let mut settings_inner = settings.clone();
       let output_levels = (0..=4).filter(|setting| settings[*setting as usize] == -1).map(|setting| {
           settings_inner[setting] = amp as i32;
           let final_level : i32 = explore(settings_inner, amp + 1, f.clone())?;
           settings_inner[setting] = -1;
           Ok(final_level)
       });
       return output_levels.max().ok_or("No results")?;
   }
}

fn main() -> io::Result<()> {
    let program_string = io::stdin().lock().lines().next().unwrap()?;
    let program : Vec<i32> = program_string.split(',')
        .filter(|line| line.len() != 0)
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let settings = [-1;5];
    println!("{:?}", explore(settings, 0, &|settings : [i32;5]| {
        let mut machines = [
            Machine { program: program.clone(), pc: 0 },
            Machine { program: program.clone(), pc: 0 },
            Machine { program: program.clone(), pc: 0 },
            Machine { program: program.clone(), pc: 0 },
            Machine { program: program.clone(), pc: 0 }
        ];
        let mut output = 0;
        for program_index in 0..5 {
            let next_output = run_program(&mut machines[program_index], [settings[program_index] + 5, output].iter())?.unwrap();
            output = next_output;
        }
        loop {
            for program_index in 0..5 {
                match run_program(&mut machines[program_index], [output].iter())? {
                    Some(next_output) => {
                        output = next_output;
                    },
                    None => {
                        return Ok(output);
                    }
                }
            }
        }
    }));
    Ok(())
}
