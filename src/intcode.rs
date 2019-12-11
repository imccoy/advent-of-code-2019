#[derive(Debug)]
struct ReadDetails {
    address: usize
}

#[derive(Debug)]
struct PrintDetails {
    value: i64
}

#[derive(Debug)]
struct JumpDetails {
    target: usize
}

#[derive(Debug)]
struct PutDetails {
    address: usize,
    value: i64
}

#[derive(Debug)]
struct SetRelativeBaseDetails {
    address: usize
}

#[derive(Debug)]
enum Command { Put(PutDetails), Read(ReadDetails), Print(PrintDetails), Jump(JumpDetails), SetRelativeBase(SetRelativeBaseDetails), Noop, Halt }

enum Operand {
  Immediate(i64),
  Position(usize),
  Relative(i64)
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
    AdjustRelativeBase(UnOp),
    Halt
}

fn operand_for_mode(mode : i64, value : i64) -> Result<Operand, String> {
    if mode == 0 {
        Ok(Operand::Position(value as usize))
    } else if mode == 1 {
        Ok(Operand::Immediate(value))
    } else if mode == 2 {
        Ok(Operand::Relative(value))
    } else {
        Err(format!("unknown operand mode {}", mode))
    }
}


fn parse_bin_op(program: &Vec<i64>, pc : usize, modes : i64) -> Result<BinOp, String> {
    let mode1 = modes % 10;
    let mode2 = (modes / 10) % 10;
    Ok(BinOp {
      op1: operand_for_mode(mode1, program[pc + 1])?, 
      op2: operand_for_mode(mode2, program[pc + 2])?
    })
}


fn parse_bin_result_op(program: &Vec<i64>, pc : usize, modes : i64) -> Result<BinResultOp, String> {
    let mode1 = modes % 10;
    let mode2 = (modes / 10) % 10;
    let mode3 = (modes / 100) % 10;
    Ok(BinResultOp {
      op1: operand_for_mode(mode1, program[pc + 1])?, 
      op2: operand_for_mode(mode2, program[pc + 2])?, 
      result: operand_for_mode(mode3, program[pc + 3])? 
    })
}

fn parse_un_op(program: &Vec<i64>, pc : usize, modes : i64) -> Result<UnOp, String> {
    let mode = modes % 10;
    Ok(UnOp {
      op: operand_for_mode(mode, program[pc + 1])?
    })
}

fn parse_instruction(program: &Vec<i64>, pc : usize) -> Result<(Instruction, usize), String> {
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
    } else if opcode == 9 {
        Ok((Instruction::AdjustRelativeBase(parse_un_op(program, pc, modes)?), 1))
    } else if opcode == 99 {
        Ok((Instruction::Halt, 0))
    } else {
        Err(format!("unknown opcode {} at {}", opcode, pc))
    }
}

fn read_from_position(machine: &Machine, position: usize) -> i64 {
   if position >= machine.program.len() {
       return 0;
   }
   return machine.program[position];
}

fn write_to_position(machine: &mut Machine, position: usize, value: i64) {
   while position > machine.program.len() {
       machine.program.resize(machine.program.len() * 2, 0);
   }
   machine.program[position] = value;
}

fn resolve_op(machine: &Machine, op : Operand) -> i64 {
    match op {
        Operand::Immediate(value) => value,
        Operand::Position(position) => read_from_position(machine, position),
        Operand::Relative(relative) => read_from_position(machine,((machine.relative_base as i64) + relative) as usize)
    }
}

fn resolve_op_position(machine: &Machine, op : Operand) -> Result<usize, String> {
    match op {
        Operand::Immediate(value) => Err(format!("got Immediate operand {}", value)),
        Operand::Position(position) => Ok(position),
        Operand::Relative(relative) => Ok(((machine.relative_base as i64) + relative) as usize)
    }
}

fn run_bin_result_op<F>(machine: &Machine, operands: BinResultOp, op: F) -> Result<Command, String> 
    where F: Fn(i64, i64) -> i64
{
    let op1 = resolve_op(machine, operands.op1);
    let op2 = resolve_op(machine, operands.op2);
    let dest = resolve_op_position(machine, operands.result)?;
    Ok(Command::Put(PutDetails { address: dest, value: op(op1, op2) }))
}

fn run_line(machine: &Machine, instruction: Instruction) -> Result<Command, String> {
    match instruction {
        Instruction::Add(operands) => {
            run_bin_result_op(machine, operands, |op1, op2| op1 + op2)
        },
        Instruction::Multiply(operands) => {
            run_bin_result_op(machine, operands, |op1, op2| op1 * op2)
        },
        Instruction::Read(operands) => {
            let op = resolve_op_position(machine, operands.op)?;
            Ok(Command::Read(ReadDetails { address: op }))
        },
        Instruction::Print(operands) => {
            let op = resolve_op(machine, operands.op);
            Ok(Command::Print(PrintDetails { value: op }))
        },
        Instruction::JumpIfTrue(operands) => {
            let op1 = resolve_op(machine, operands.op1);
            let target = resolve_op(machine, operands.op2);
            if op1 != 0 {
                Ok(Command::Jump(JumpDetails { target: target as usize }))
            } else {
                Ok(Command::Noop)
            }
        },
        Instruction::JumpIfFalse(operands) => {
            let op1 = resolve_op(machine, operands.op1);
            let target = resolve_op(machine, operands.op2);
            if op1 == 0 {
                Ok(Command::Jump(JumpDetails { target: target as usize }))
            } else {
                Ok(Command::Noop)
            }
        },
        Instruction::LessThan(operands) => {
            run_bin_result_op(machine, operands, |op1, op2| {
                if op1 < op2 {
                    1
                } else {
                    0
                }
            })
        },
        Instruction::Equals(operands) => {
            run_bin_result_op(machine, operands, |op1, op2| {
                if op1 == op2 {
                    1
                } else {
                    0
                }
            })
        },
        Instruction::AdjustRelativeBase(operands) => {
            let op = resolve_op(machine, operands.op);
            Ok(Command::SetRelativeBase(SetRelativeBaseDetails { address: ((machine.relative_base as i64 + op) as usize) }))
        },
        Instruction::Halt => {
            Ok(Command::Halt)
        }
    }
}

pub struct Machine {
    pub program: Vec<i64>,
    pub pc: usize,
    pub relative_base: usize
}

pub fn run_program(machine : &mut Machine, mut inputs: std::slice::Iter<i64>) -> Result<Option<i64>, String> 
{
    loop {
        let (instruction, instruction_length) = parse_instruction(&machine.program, machine.pc)?;
        let command = run_line(&machine, instruction)?;
        machine.pc += instruction_length + 1; // jump instructions will overwrite this
        match command {
            Command::Put(put) => {
                write_to_position(machine, put.address, put.value);
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
            Command::SetRelativeBase(set_relative_base) => {
                machine.relative_base = set_relative_base.address;
            },
            Command::Noop => { 
                // do nothing
            },
            Command::Halt => return Ok(None)
        }
    }
}

