use once_cell::sync::Lazy;
use std::error::Error;
use std::str::FromStr;

static INPUT: Lazy<Vec<Turn>> =
    Lazy::new(|| load_input("data/day01.txt").expect("Failed to load input"));

fn main() {
    println!("Part 1: {}", part1(&INPUT));
    println!("Part 2: {}", part2(&INPUT));
}

fn part1(input: &[Turn]) -> u32 {
    let mut state = State::default();
    for turn in input {
        state.step(turn);
    }
    state.distance()
}

fn part2(input: &[Turn]) -> u32 {
    let mut visited = std::collections::HashSet::new();
    visited.insert((0, 0));
    let mut state = State::default();
    for turn in input {
        let steps = state.step_with_trace(turn);
        for (x, y) in steps {
            if visited.contains(&(x, y)) {
                return (x.abs() + y.abs()) as u32;
            }
            visited.insert((x, y));
        }
    }
    panic!("Should never get here");
}

fn load_input(path: &str) -> Result<Vec<Turn>, Box<dyn Error>> {
    common::read_file_as_string(path)?
        .split(", ")
        .map(|s| {
            s.parse()
                .map_err(|e| Box::<dyn Error>::from(format!("{:?}", e)))
        })
        .collect()
}

#[derive(Debug, Default)]
struct State {
    x: i32,
    y: i32,
    heading: Direction,
}

impl State {
    fn step(&mut self, turn: &Turn) {
        self.heading = self.heading.turn(turn);
        self.forward(turn.steps())
    }
    fn forward(&mut self, steps: u32) {
        match self.heading {
            Direction::North => self.y += steps as i32,
            Direction::East => self.x += steps as i32,
            Direction::South => self.y -= steps as i32,
            Direction::West => self.x -= steps as i32,
        }
    }
    fn step_with_trace(&mut self, turn: &Turn) -> Vec<(i32, i32)> {
        self.heading = self.heading.turn(turn);
        self.forward_with_trace(turn.steps())
    }
    fn forward_with_trace(&mut self, steps: u32) -> Vec<(i32, i32)> {
        match self.heading {
            Direction::North => {
                let trace = (self.y + 1..=self.y + steps as i32)
                    .map(|y| (self.x, y))
                    .collect();
                self.y += steps as i32;
                trace
            }
            Direction::East => {
                let trace = (self.x + 1..=self.x + steps as i32)
                    .map(|x| (x, self.y))
                    .collect();
                self.x += steps as i32;
                trace
            }
            Direction::South => {
                let trace = (self.y - steps as i32..=self.y - 1)
                    .map(|y| (self.x, y))
                    .rev()
                    .collect();
                self.y -= steps as i32;
                trace
            }
            Direction::West => {
                let trace = (self.x - steps as i32..=self.x - 1)
                    .map(|x| (x, self.y))
                    .rev()
                    .collect();
                self.x -= steps as i32;
                trace
            }
        }
    }
    fn distance(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

impl Direction {
    fn turn(&self, turn: &Turn) -> Self {
        match turn {
            Turn::Left(_) => self.left(),
            Turn::Right(_) => self.right(),
        }
    }
    fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}
#[derive(Debug)]
enum Turn {
    Left(u32),
    Right(u32),
}

#[derive(Debug)]
struct TurnParseError(String);

impl std::fmt::Display for TurnParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Turn parse error: {}", self.0)
    }
}

impl Error for TurnParseError {}

impl FromStr for Turn {
    type Err = TurnParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let steps = &s[1..].parse::<u32>().unwrap();
        if s.starts_with("L") {
            Ok(Turn::Left(*steps))
        } else if s.starts_with("R") {
            Ok(Turn::Right(*steps))
        } else {
            Err(TurnParseError(s.to_string()))
        }
    }
}

impl Turn {
    fn steps(&self) -> u32 {
        match self {
            Turn::Left(steps) => *steps,
            Turn::Right(steps) => *steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Turn::*;

    #[test]
    fn test_example1_part1() {
        let input = vec![Right(2), Left(3)];
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_example2_part1() {
        let input = vec![Right(2), Right(2), Right(2)];
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_example3_part1() {
        let input = vec![Right(5), Left(5), Right(5), Right(3)];
        assert_eq!(part1(&input), 12);
    }

    #[test]
    fn test_example1_part2() {
        let input = vec![Right(8), Right(4), Right(4), Right(8)];
        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 300);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 159);
    }
}
