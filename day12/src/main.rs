use once_cell::sync::Lazy;
use std::num::ParseIntError;
use std::str::FromStr;

static PROGRAM: Lazy<Program> = Lazy::new(|| {
    common::read_file_as_elements("data/day12.txt")
        .unwrap()
        .into()
});

fn main() {
    println!("Part 1: {}", part1(&PROGRAM));
    println!("Part 2: {}", part2(&PROGRAM));
}

fn part1(program: &Program) -> i32 {
    let mut computer = Computer::default();
    computer.run(program);
    computer.registers[0]
}

fn part2(program: &Program) -> i32 {
    let mut computer = Computer::default();
    computer.registers[2] = 1;
    computer.run(program);
    computer.registers[0]
}

#[derive(Clone, Debug)]
enum Instruction {
    CpyV(i32, usize),
    CpyR(usize, usize),
    IncR(usize),
    DecR(usize),
    JnzV(i32, isize),
    JnzR(usize, isize),
}

#[derive(Debug)]
enum ParseError {
    Int(ParseIntError),
    Custom(String),
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::Int(e)
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
                    Ok(CpyV(value, right?))
                } else {
                    Ok(CpyR(to_register(parts[1])?, right?))
                }
            }
            "inc" => Ok(IncR(to_register(parts[1])?)),
            "dec" => Ok(DecR(to_register(parts[1])?)),
            "jnz" => {
                let left = parts[1].parse::<i32>();
                let right = parts[2].parse::<isize>();
                if let Ok(value) = left {
                    Ok(JnzV(value, right?))
                } else {
                    Ok(JnzR(to_register(parts[1])?, right?))
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
    fn run(&mut self, program: &Program) {
        use Instruction::*;
        let mut ip = 0;
        while ip < program.0.len() {
            let instruction = &program.0[ip];
            match instruction {
                CpyV(value, to) => self.registers[*to] = *value,
                CpyR(from, to) => self.registers[*to] = self.registers[*from],
                IncR(reg) => self.registers[*reg] += 1,
                DecR(reg) => self.registers[*reg] -= 1,
                JnzV(value, step) => {
                    if *value != 0 {
                        ip = (ip as isize + *step) as usize;
                        continue;
                    }
                }
                JnzR(reg, step) => {
                    if self.registers[*reg] != 0 {
                        ip = (ip as isize + *step) as usize;
                        continue;
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
    fn test_example() {
        let input = vec!["cpy 41 a", "inc a", "inc a", "dec a", "jnz a 2", "dec a"];
        let program = Program(input.into_iter().map(|s| s.parse().unwrap()).collect());
        let mut computer = Computer::default();
        computer.run(&program);
        assert_eq!(computer.registers, [42, 0, 0, 0]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&PROGRAM), 318083);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&PROGRAM), 9227737);
    }
}
