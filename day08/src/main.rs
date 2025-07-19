use once_cell::sync::Lazy;
use std::fmt;
use std::str::FromStr;

static INPUT: Lazy<Vec<Operation>> =
    Lazy::new(|| common::read_file_as_elements("data/day08.txt").unwrap());

fn main() {
    println!("Print 1: {}", part1(&INPUT));
    println!("Print 2:");
    part2(&INPUT);
}

fn part1(input: &[Operation]) -> usize {
    let mut screen = Screen::new(50, 6);
    screen.run(input);
    screen.count_lit()
}

fn part2(input: &[Operation]) {
    let mut screen = Screen::new(50, 6);
    screen.run(input);
    println!("{}", screen);
}

#[derive(Debug, Clone)]
enum Operation {
    Rect { width: usize, height: usize },
    RotateRow { y: usize, by: usize },
    RotateColumn { x: usize, by: usize },
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Operation::*;

        static RECT_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"rect (\d+)x(\d+)").unwrap());
        static ROTATE_ROW_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap());
        static ROTATE_COLUMN_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap());

        if let Some(captures) = RECT_REGEX.captures(s) {
            let width = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let height = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            return Ok(Rect { width, height });
        } else if let Some(captures) = ROTATE_ROW_REGEX.captures(s) {
            let y = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let by = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            return Ok(RotateRow { y, by });
        } else if let Some(captures) = ROTATE_COLUMN_REGEX.captures(s) {
            let x = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let by = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            return Ok(RotateColumn { x, by });
        } else {
            return Err("Invalid operation".to_string());
        }
    }
}

struct Screen(Vec<Vec<bool>>);

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Screen(vec![vec![false; width]; height])
    }

    fn rect(&mut self, width: usize, height: usize) {
        for y in 0..height {
            for x in 0..width {
                self.0[y][x] = true;
            }
        }
    }

    fn rotate_row(&mut self, y: usize, by: usize) {
        self.0[y].rotate_right(by);
    }

    fn rotate_column(&mut self, x: usize, by: usize) {
        let mut column = self.0.iter().map(|row| row[x]).collect::<Vec<_>>();
        column.rotate_right(by);
        for (y, pixel) in self.0.iter_mut().enumerate() {
            pixel[x] = column[y];
        }
    }

    fn run(&mut self, operations: &[Operation]) {
        use Operation::*;
        for op in operations {
            match op {
                Rect { width, height } => self.rect(*width, *height),
                RotateRow { y, by } => self.rotate_row(*y, *by),
                RotateColumn { x, by } => self.rotate_column(*x, *by),
            }
        }
    }

    fn count_lit(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|&p| *p).count())
            .sum()
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for &pixel in row {
                write!(f, "{}", if pixel { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut screen = Screen::new(7, 3);
        screen.rect(3, 2);
        assert_eq!(
            screen.0,
            vec![
                vec![true, true, true, false, false, false, false],
                vec![true, true, true, false, false, false, false],
                vec![false, false, false, false, false, false, false],
            ]
        );
        screen.rotate_column(1, 1);
        assert_eq!(
            screen.0,
            vec![
                vec![true, false, true, false, false, false, false],
                vec![true, true, true, false, false, false, false],
                vec![false, true, false, false, false, false, false],
            ]
        );
        screen.rotate_row(0, 4);
        assert_eq!(
            screen.0,
            vec![
                vec![false, false, false, false, true, false, true],
                vec![true, true, true, false, false, false, false],
                vec![false, true, false, false, false, false, false],
            ]
        );
        screen.rotate_column(1, 1);
        assert_eq!(
            screen.0,
            vec![
                vec![false, true, false, false, true, false, true],
                vec![true, false, true, false, false, false, false],
                vec![false, true, false, false, false, false, false],
            ]
        );
        assert_eq!(screen.count_lit(), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 121);
    }
}
