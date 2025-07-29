use once_cell::sync::Lazy;
use std::num::ParseIntError;
use std::str::FromStr;

static PROGRAM: Lazy<Program> = Lazy::new(|| {
    common::read_file_as_elements("data/day23.txt")
        .unwrap()
        .into()
});

fn main() {
    println!("Part 1: {}", part1(&PROGRAM));
    println!("Part 1: {}", part2(&PROGRAM));
}

fn part1(program: &Program) -> i32 {
    let mut program = program.clone();
    let mut computer = Computer::default();
    computer.registers[0] = 7;
    computer.run(&mut program);
    computer.registers[0]
}

fn part2(program: &Program) -> i32 {
    let mut program = program.clone();
    let mut computer = Computer::default();
    computer.registers[0] = 12;
    computer.run(&mut program);
    computer.registers[0]
}

#[derive(Clone, Debug)]
enum Instruction {
    Cpy(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Jnz(Arg, Arg),
    Toggle(Arg),
}

impl Instruction {
    fn toggle(&self) -> Instruction {
        use Instruction::*;
        match self {
            Cpy(from, to) => Jnz(from.clone(), to.clone()),
            Inc(reg) => Dec(reg.clone()),
            Dec(reg) => Inc(reg.clone()),
            Jnz(value, step) => Cpy(value.clone(), step.clone()),
            Toggle(step) => Inc(step.clone()),
        }
    }
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
            "tgl" => {
                let steps = parts[1].parse::<i32>();
                if let Ok(value) = steps {
                    Ok(Toggle(Value(value)))
                } else {
                    Ok(Toggle(Register(to_register(parts[1])?)))
                }
            }
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
    fn run(&mut self, program: &mut Program) {
        use Arg::*;
        use Instruction::*;
        let mut ip = 0;
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
                Toggle(step) => {
                    let step = match step {
                        Register(reg) => self.registers[*reg],
                        Value(value) => *value,
                    } as isize;
                    let addr = ip as isize + step;
                    if addr >= 0 && addr < program.0.len() as isize {
                        program.0[addr as usize] = program.0[addr as usize].toggle();
                    }
                }
            }
            ip += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_day12() {
        let input = vec!["cpy 41 a", "inc a", "inc a", "dec a", "jnz a 2", "dec a"];
        let mut program = Program(input.into_iter().map(|s| s.parse().unwrap()).collect());
        let mut computer = Computer::default();
        computer.run(&mut program);
        assert_eq!(computer.registers, [42, 0, 0, 0]);
    }

    #[test]
    fn test_day12_part1() {
        let mut program: Program = common::read_file_as_elements("data/day12.txt")
            .unwrap()
            .into();
        let mut computer = Computer::default();
        computer.run(&mut program);
        assert_eq!(computer.registers[0], 318083);
    }

    #[test]
    fn test_day12_part2() {
        let mut program: Program = common::read_file_as_elements("data/day12.txt")
            .unwrap()
            .into();
        let mut computer = Computer::default();
        computer.registers[2] = 1;
        computer.run(&mut program);
        assert_eq!(computer.registers[0], 9227737);
    }

    #[test]
    fn test_example_part1() {
        let input = vec![
            "cpy 2 a", "tgl a", "tgl a", "tgl a", "cpy 1 a", "dec a", "dec a",
        ];
        let mut program = Program(input.into_iter().map(|s| s.parse().unwrap()).collect());
        let mut computer = Computer::default();
        computer.run(&mut program);
        assert_eq!(computer.registers[0], 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&PROGRAM), 12000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&PROGRAM), 479008560);
    }
}
