use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

static OPERATIONS: Lazy<Vec<Operation>> =
    Lazy::new(|| common::read_file_as_elements("data/day21.txt").unwrap());

fn main() {
    println!("Part 1: {}", part1(&OPERATIONS, "abcdefgh"));
    println!("Part 2: {}", part2(&OPERATIONS, "fbgdceah"));
}

fn part1(operations: &[Operation], input: &str) -> String {
    Scrambler::new(input).scramble(operations)
}

fn part2(operations: &[Operation], input: &str) -> String {
    let inverted = invert(operations);
    Scrambler::new(input).scramble(&inverted)
}

fn invert(operations: &[Operation]) -> Vec<Operation> {
    operations.iter().rev().map(|op| op.invert()).collect()
}

struct Scrambler {
    state: Vec<u8>,
}

impl Scrambler {
    fn new(input: &str) -> Self {
        Self {
            state: input.as_bytes().to_vec(),
        }
    }

    fn scramble(&mut self, operations: &[Operation]) -> String {
        use Operation::*;
        for op in operations {
            match op {
                MovePosition(from, to) => self.move_position(from, to),
                SwapPosition(from, to) => self.swap_position(from, to),
                SwapLetter(x, y) => self.swap_letter(x, y),
                RotateRight(pos) => self.rotate_right(pos),
                RotateLeft(pos) => self.rotate_left(pos),
                ReversePositions(from, to) => self.reverse_positions(from, to),
                RotateBasedOnPositionOfLetter(c) => self.rotate_based_on_position_of_letter(c),
                InvertedRotateBasedOnPositionOfLetter(c) => {
                    self.inverted_rotate_based_on_position_of_letter(c)
                }
            }
        }
        self.result()
    }

    fn result(&self) -> String {
        self.state.iter().map(|&c| c as char).collect()
    }

    fn move_position(&mut self, from: &usize, to: &usize) {
        let c = self.state.remove(*from);
        self.state.insert(*to, c);
    }

    fn swap_position(&mut self, x: &usize, y: &usize) {
        self.state.swap(*x, *y);
    }

    fn swap_letter(&mut self, x: &u8, y: &u8) {
        let x = self.state.iter().position(|&c| c == *x).unwrap();
        let y = self.state.iter().position(|&c| c == *y).unwrap();
        self.swap_position(&x, &y);
    }

    fn rotate_right(&mut self, pos: &usize) {
        self.state.rotate_right(*pos);
    }

    fn rotate_left(&mut self, pos: &usize) {
        self.state.rotate_left(*pos);
    }

    fn reverse_positions(&mut self, from: &usize, to: &usize) {
        let mut i = usize::min(*from, *to);
        let mut j = usize::max(*from, *to);
        while i < j {
            self.state.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    fn rotate_based_on_position_of_letter(&mut self, letter: &u8) {
        let idx = self.state.iter().position(|&c| c == *letter).unwrap();
        let steps = (1 + idx + if idx >= 4 { 1 } else { 0 }) % self.state.len();
        self.rotate_right(&steps);
    }

    fn inverted_rotate_based_on_position_of_letter(&mut self, letter: &u8) {
        // inversion for inputs of size 8
        let inverse_steps_to_left = [1, 1, 6, 2, 7, 3, 0, 4];
        let idx = self.state.iter().position(|&c| c == *letter).unwrap();
        let steps = inverse_steps_to_left[idx];
        self.rotate_left(&steps);
    }
}

#[derive(Debug)]
enum Operation {
    MovePosition(usize, usize),
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    RotateRight(usize),
    RotateLeft(usize),
    ReversePositions(usize, usize),
    RotateBasedOnPositionOfLetter(u8),
    InvertedRotateBasedOnPositionOfLetter(u8),
}

impl Operation {
    fn invert(&self) -> Self {
        use Operation::*;
        match self {
            MovePosition(from, to) => MovePosition(*to, *from),
            SwapPosition(from, to) => SwapPosition(*from, *to),
            SwapLetter(x, y) => SwapLetter(*x, *y),
            RotateRight(pos) => RotateLeft(*pos),
            RotateLeft(pos) => RotateRight(*pos),
            ReversePositions(from, to) => ReversePositions(*from, *to),
            RotateBasedOnPositionOfLetter(c) => InvertedRotateBasedOnPositionOfLetter(*c),
            InvertedRotateBasedOnPositionOfLetter(c) => RotateBasedOnPositionOfLetter(*c),
        }
    }
}

static ROTATE_POSITION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^rotate based on position of letter (\w)$").unwrap());

static MOVE_POSITION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^move position (\d) to position (\d)$").unwrap());

static SWAP_POSITION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^swap position (\d) with position (\d)$").unwrap());

static ROTATE_RIGHT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^rotate right (\d) steps?$").unwrap());

static ROTATE_LEFT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^rotate left (\d) steps?$").unwrap());

static REVERSE_POSITIONS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^reverse positions (\d) through (\d)$").unwrap());

static ROTATE_BASED_ON_POSITION_OF_LETTER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^rotate based on position of letter (\w)$").unwrap());

static SWAP_LETTER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^swap letter (\w) with letter (\w)$").unwrap());

#[derive(Debug)]
enum ParseOperationError {
    InvalidLine(String),
}

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cap) = ROTATE_POSITION_REGEX.captures(s) {
            let letter = cap[1].as_bytes().first().unwrap();
            Ok(Operation::RotateBasedOnPositionOfLetter(*letter))
        } else if let Some(cap) = MOVE_POSITION_REGEX.captures(s) {
            let from = cap[1].parse::<usize>().unwrap();
            let to = cap[2].parse::<usize>().unwrap();
            Ok(Operation::MovePosition(from, to))
        } else if let Some(cap) = SWAP_POSITION_REGEX.captures(s) {
            let from = cap[1].parse::<usize>().unwrap();
            let to = cap[2].parse::<usize>().unwrap();
            Ok(Operation::SwapPosition(from, to))
        } else if let Some(cap) = ROTATE_RIGHT_REGEX.captures(s) {
            let steps = cap[1].parse::<usize>().unwrap();
            Ok(Operation::RotateRight(steps))
        } else if let Some(cap) = ROTATE_LEFT_REGEX.captures(s) {
            let steps = cap[1].parse::<usize>().unwrap();
            Ok(Operation::RotateLeft(steps))
        } else if let Some(cap) = REVERSE_POSITIONS_REGEX.captures(s) {
            let from = cap[1].parse::<usize>().unwrap();
            let to = cap[2].parse::<usize>().unwrap();
            Ok(Operation::ReversePositions(from, to))
        } else if let Some(cap) = ROTATE_BASED_ON_POSITION_OF_LETTER_REGEX.captures(s) {
            let letter = cap[1].as_bytes().first().unwrap();
            Ok(Operation::RotateBasedOnPositionOfLetter(*letter))
        } else if let Some(cap) = SWAP_LETTER_REGEX.captures(s) {
            let from = cap[1].as_bytes().first().unwrap();
            let to = cap[2].as_bytes().first().unwrap();
            Ok(Operation::SwapLetter(*from, *to))
        } else {
            Err(ParseOperationError::InvalidLine(s.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_operations() {
        let mut scrambler = Scrambler::new("abcde");

        scrambler.swap_position(&4, &0);
        assert_eq!(scrambler.result(), "ebcda");

        scrambler.swap_letter(&b'd', &b'b');
        assert_eq!(scrambler.result(), "edcba");

        scrambler.reverse_positions(&4, &0);
        assert_eq!(scrambler.result(), "abcde");

        scrambler.rotate_left(&1);
        assert_eq!(scrambler.result(), "bcdea");

        scrambler.move_position(&1, &4);
        assert_eq!(scrambler.result(), "bdeac");

        scrambler.move_position(&3, &0);
        assert_eq!(scrambler.result(), "abdec");

        scrambler.rotate_based_on_position_of_letter(&b'b');
        assert_eq!(scrambler.result(), "ecabd");

        scrambler.rotate_based_on_position_of_letter(&b'd');
        assert_eq!(scrambler.result(), "decab");
    }

    #[test]
    fn test_example_part1() {
        use Operation::*;
        let operations = vec![
            SwapPosition(4, 0),
            SwapLetter(b'd', b'b'),
            ReversePositions(4, 0),
            RotateLeft(1),
            MovePosition(1, 4),
            MovePosition(3, 0),
            RotateBasedOnPositionOfLetter(b'b'),
            RotateBasedOnPositionOfLetter(b'd'),
        ];
        assert_eq!(part1(&operations, "abcde"), "decab");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&OPERATIONS, "abcdefgh"), "ghfacdbe");
    }

    #[test]
    fn test_another_example_part1() {
        assert_eq!(part1(&OPERATIONS, "fhgcdaeb"), "fbgdceah");
    }

    #[test]
    fn test_inverse_swap_position() {
        let mut scrambler = Scrambler::new("abcde");
        scrambler.swap_position(&4, &0);
        assert_eq!(scrambler.result(), "ebcda");
        scrambler.swap_position(&4, &0);
        assert_eq!(scrambler.result(), "abcde");
    }

    #[test]
    fn test_inverse_move_position() {
        let mut scrambler = Scrambler::new("bcdea");
        scrambler.move_position(&1, &4);
        assert_eq!(scrambler.result(), "bdeac");
        scrambler.move_position(&4, &1);
        assert_eq!(scrambler.result(), "bcdea");
    }

    #[test]
    fn test_inverse_swap_letter() {
        let mut scrambler = Scrambler::new("ebcda");
        scrambler.swap_letter(&b'd', &b'b');
        assert_eq!(scrambler.result(), "edcba");
        scrambler.swap_letter(&b'd', &b'b');
        assert_eq!(scrambler.result(), "ebcda");
    }

    #[test]
    fn test_inverse_rotate_left() {
        let mut scrambler = Scrambler::new("abcde");
        scrambler.rotate_left(&1);
        assert_eq!(scrambler.result(), "bcdea");
        scrambler.rotate_right(&1);
        assert_eq!(scrambler.result(), "abcde");
    }

    #[test]
    fn test_reverse_positions() {
        let mut scrambler = Scrambler::new("edcba");
        scrambler.reverse_positions(&4, &0);
        assert_eq!(scrambler.result(), "abcde");
        scrambler.reverse_positions(&4, &0);
        assert_eq!(scrambler.result(), "edcba");
    }

    #[test]
    fn test_rotate_based_on_position_of_letter1() {
        let mut scrambler = Scrambler::new("abdec");
        scrambler.rotate_based_on_position_of_letter(&b'b');
        assert_eq!(scrambler.result(), "ecabd");
        scrambler.inverted_rotate_based_on_position_of_letter(&b'b');
        assert_eq!(scrambler.result(), "abdec");
    }

    #[test]
    fn test_rotate_based_on_position_of_letter2() {
        let mut scrambler = Scrambler::new("ecabd");
        scrambler.rotate_based_on_position_of_letter(&b'd');
        assert_eq!(scrambler.result(), "decab");
        scrambler.inverted_rotate_based_on_position_of_letter(&b'd');
        assert_eq!(scrambler.result(), "ecabd");
    }

    #[test]
    fn test_generate_rotate_based_on_position_of_letter_for_size_eight() {
        let mut scrambler = Scrambler::new("abcdefgh");
        for c in b'a'..=b'h' {
            println!("{:?}", scrambler.result());
            eprintln!("On letter: {}", c.to_ascii_lowercase() as char);
            scrambler.rotate_based_on_position_of_letter(&c);
            eprintln!("{:?}", scrambler.result());
            scrambler.inverted_rotate_based_on_position_of_letter(&c);
            eprintln!("{:?}", scrambler.result());
            assert_eq!(scrambler.result(), "abcdefgh");
        }
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&OPERATIONS, "fbgdceah"), "fhgcdaeb");
    }

    #[test]
    fn test_another_example_part2() {
        let operations = invert(&OPERATIONS);
        assert_eq!(part2(&OPERATIONS, "ghfacdbe"), "abcdefgh");
    }
}
