use once_cell::sync::Lazy;
use std::num::ParseIntError;
use std::str::FromStr;

static PROGRAM: Lazy<Program> = Lazy::new(|| {
    common::read_file_as_elements("data/day25.txt")
        .unwrap()
        .into()
});

fn main() {
    for i in 0..256 {
        let mut computer = Computer::default();
        print!("Input {}: ", i);
        computer.registers[0] = i;
        computer.run(&PROGRAM, 20);
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Cpy(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Jnz(Arg, Arg),
    Out(Arg),
}


#[derive(Clone, Debug)]
enum Arg {
    Register(usize),
    Value(i32),
}

#[derive(Debug)]
enum ParseError {
    Int(String),
    Custom(String),
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::Int(e.to_string())
    }
}

impl From<String> for ParseError {
    fn from(e: String) -> Self {
        ParseError::Custom(e)
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Arg::*;
        use Instruction::*;
        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.is_empty() {
            return Err(ParseError::Custom("Empty instruction".to_string()));
        }
        match parts[0] {
            "cpy" => {
                let left = parts[1].parse::<i32>();
                let right = to_register(parts[2]);
                if let Ok(value) = left {
                    Ok(Cpy(Value(value), Register(right?)))
                } else {
                    Ok(Cpy(Register(to_register(parts[1])?), Register(right?)))
                }
            }
            "inc" => Ok(Inc(Register(to_register(parts[1])?))),
            "dec" => Ok(Dec(Register(to_register(parts[1])?))),
            "jnz" => {
                let left = parts[1].parse::<i32>();
                let left_arg = if let Ok(value) = left {
                    Value(value)
                } else {
                    Register(to_register(parts[1])?)
                };
                let right = parts[2].parse::<i32>();
                let right_arg = if let Ok(value) = right {
                    Value(value)
                } else {
                    Register(to_register(parts[2])?)
                };
                Ok(Jnz(left_arg, right_arg))
            }
            "out" => Ok(Out(Register(to_register(parts[1])?))),
            _ => Err(ParseError::Custom(format!("Unknown instruction: {}", s))),
        }
    }
}

fn to_register(register: &str) -> Result<usize, ParseError> {
    let reg_letter = register.chars().next().unwrap();
    Ok(reg_letter as usize - 'a' as usize)
}

#[derive(Clone, Debug)]
struct Program(Vec<Instruction>);

impl From<Vec<Instruction>> for Program {
    fn from(value: Vec<Instruction>) -> Self {
        Self(value)
    }
}

#[derive(Default)]
struct Computer {
    registers: [i32; 4],
}

impl Computer {
    fn run(&mut self, program: &Program, max_output: usize) {
        use Arg::*;
        use Instruction::*;
        let mut ip = 0;
        let mut count_output = 0;
        while ip < program.0.len() {
            let instruction = &program.0[ip];
            match instruction {
                Cpy(from, to) => {
                    if let Register(to_reg) = to {
                        self.registers[*to_reg] = match from {
                            Register(from_reg) => self.registers[*from_reg],
                            Value(from_value) => *from_value,
                        }
                    }
                }
                Inc(reg) => {
                    if let Register(to_reg) = reg {
                        self.registers[*to_reg] += 1;
                    }
                }
                Dec(reg) => {
                    if let Register(to_reg) = reg {
                        self.registers[*to_reg] -= 1;
                    }
                }
                Jnz(value, step) => {
                    let test_value = match value {
                        Register(reg) => self.registers[*reg],
                        Value(value) => *value,
                    };
                    if test_value != 0 {
                        let step = match step {
                            Register(reg) => self.registers[*reg],
                            Value(value) => *value,
                        } as isize;
                        ip = (ip as isize + step) as usize;
                        continue;
                    }
                }
                Out(reg) => {
                    if let Register(to_reg) = reg {
                        print!("{}", self.registers[*to_reg]);
                        count_output += 1;
                        if count_output == max_output {
                            println!();
                            return;
                        }
                    }
                }
            }
            ip += 1;
        }
    }
}
