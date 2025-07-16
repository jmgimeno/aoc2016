use once_cell::sync::Lazy;

static INPUT: Lazy<Instructions> =
    Lazy::new(|| load_input("data/day02.txt").expect("Failed to load input"));

fn main() {
    println!("{:?}", part1(&INPUT));
}

fn part1(instructions: &Instructions) -> String {
    instructions.bathroom_code_part1()
}

struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn to_char(&self) -> String {
        (self.x + self.y * 3 + 1).to_string()
    }

    fn update_part1(&mut self, movement: &Movement) {
        use Movement::*;
        match movement {
            Up => self.y = if self.y == 0 { 0 } else { self.y - 1 },
            Down => self.y = if self.y == 2 { 2 } else { self.y + 1 },
            Left => self.x = if self.x == 0 { 0 } else { self.x - 1 },
            Right => self.x = if self.x == 2 { 2 } else { self.x + 1 },
        }
    }
}

#[derive(Debug)]
struct Instructions(Vec<Line>);

impl Instructions {
    fn from<T: AsRef<str>>(input: &[T]) -> Self {
        let mut instructions = Vec::new();
        for line in input {
            let movements: Vec<_> = line.as_ref().chars().map(Movement::from).collect();
            instructions.push(Line(movements));
        }
        Self(instructions)
    }

    fn bathroom_code_part1(&self) -> String {
        let mut position = Position { x: 1, y: 1 };
        let mut code = String::new();
        for line in &self.0 {
            code.push_str(line.bathroom_code_part1(&mut position).as_str());
        }
        code
    }
}

#[derive(Debug)]
struct Line(Vec<Movement>);

impl Line {
    fn bathroom_code_part1(&self, position: &mut Position) -> String {
        for movement in &self.0 {
            position.update_part1(movement);
        }
        position.to_char()
    }
}

#[derive(Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
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
}
