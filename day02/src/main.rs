use once_cell::sync::Lazy;

static INPUT: Lazy<Instructions> =
    Lazy::new(|| load_input("data/day02.txt").expect("Failed to load input"));

const KEYPAD1: [&str; 5] = [
    "     ",
    " 123 ",
    " 456 ",
    " 789 ",
    "     ",
];

const KEYPAD2: [&str; 7] = [
    "       ",
    "   1   ",
    "  234  ",
    " 56789 ",
    "  ABC  ",
    "   D   ",
    "       ",
];

fn main() {
    println!("Part 1: {}", part1(&INPUT));
    println!("Part 2: {}", part2(&INPUT));
}

fn part1(instructions: &Instructions) -> String {
    Keypad(KEYPAD1.to_vec()).bathroom_code(&instructions)
}

fn part2(instructions: &Instructions) -> String {
    Keypad(KEYPAD2.to_vec()).bathroom_code(&instructions)
}

struct Keypad(Vec<&'static str>);

impl Keypad {
    fn starting(&self, digit: char) -> Position {
        for (y, line) in self.0.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == digit {
                    return Position { x, y };
                }
            }
        }
        panic!("Invalid digit");
    }

    fn char_at(&self, pos: &Position) -> char {
        self.0[pos.y].chars().nth(pos.x).unwrap()
    }

    fn is_valid(&self, pos: &Position) -> bool {
        self.char_at(pos) != ' '
    }

    fn bathroom_code(&self, instructions: &Instructions) -> String {
        let mut position = self.starting('5');
        let mut code = String::new();
        for line in &instructions.0 {
            code.push(self.bathroom_digit(line, &mut position));
        }
        code
    }

    fn bathroom_digit(&self, line: &Line, position: &mut Position) -> char {
        for movement in &line.0 {
            self.try_move(movement, position);
        }
        self.char_at(position)
    }

    fn try_move(&self, movement: &Movement, position: &mut Position) {
        let new_pos = Movement::next_position(movement, position);
        if self.is_valid(&new_pos) {
            *position = new_pos;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Instructions(Vec<Line>);

impl Instructions {
    fn from<T: AsRef<str>>(input: &[T]) -> Self {
        Self(
            input
                .iter()
                .map(|line| Line(line.as_ref().chars().map(Movement::from).collect()))
                .collect(),
        )
    }
}

#[derive(Debug)]
struct Line(Vec<Movement>);

#[derive(Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl Movement {
    fn next_position(&self, pos: &Position) -> Position {
        use Movement::*;
        match self {
            Up => Position {
                x: pos.x,
                y: pos.y - 1,
            },
            Down => Position {
                x: pos.x,
                y: pos.y + 1,
            },
            Left => Position {
                x: pos.x - 1,
                y: pos.y,
            },
            Right => Position {
                x: pos.x + 1,
                y: pos.y,
            },
        }
    }
}

impl From<char> for Movement {
    fn from(value: char) -> Self {
        match value {
            'U' => Movement::Up,
            'D' => Movement::Down,
            'L' => Movement::Left,
            'R' => Movement::Right,
            _ => panic!("Invalid movement"),
        }
    }
}

fn load_input(path: &str) -> Result<Instructions, Box<dyn std::error::Error>> {
    let input = common::read_file_as_lines(path)?;
    Ok(Instructions::from(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = vec!["ULL", "RRDDD", "LURDL", "UUUUD"];
        let instructions = Instructions::from(&input);
        assert_eq!(part1(&instructions), "1985");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), "74921");
    }

    #[test]
    fn test_example_part2() {
        let input = vec!["ULL", "RRDDD", "LURDL", "UUUUD"];
        let instructions = Instructions::from(&input);
        assert_eq!(part2(&instructions), "5DB3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), "A6B35");
    }
}
