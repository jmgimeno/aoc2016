use once_cell::sync::Lazy;

static INPUT: Lazy<String> = Lazy::new(|| common::read_file_as_string("data/day09.txt").unwrap());

fn main() {
    println!("Part 1: {}", part1(&INPUT));
}

fn part1(input: &str) -> usize {
    let mut rest = input.as_bytes();
    let mut result = 0;
    let mut state = State::Regular(0);
    while !rest.is_empty() {
        let current = rest[0];
        rest = &rest[1..];
        match state {
            State::Regular(x) => match current {
                b'(' => {
                    result += x;
                    state = State::FirstNum(0);
                }
                _ => state = State::Regular(x + 1),
            },
            State::FirstNum(x) => match current {
                b'x' => {
                    state = State::SecondNum(x, 0);
                }
                c => {
                    let digit = c - b'0';
                    state = State::FirstNum(10 * x + digit as usize);
                }
            },
            State::SecondNum(x, y) => match current {
                b')' => {
                    result += x * y;
                    state = State::Regular(0);
                    rest = &rest[x..];
                }
                c => {
                    let digit = c - b'0';
                    state = State::SecondNum(x, 10 * y + digit as usize);
                }
            },
        }
    }
    if let State::Regular(x) = state {
        return result + x;
    }
    unreachable!("should have gotten a state");
}

enum State {
    Regular(usize),
    FirstNum(usize),
    SecondNum(usize, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!(part1("ADVENT"), 6);
        assert_eq!(part1("A(1x5)BC"), 7);
        assert_eq!(part1("(3x3)XYZ"), 9);
        assert_eq!(part1("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(part1("(6x1)(1x3)A"), 6);
        assert_eq!(part1("X(8x2)(3x3)ABCY"), 18);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 112830);
    }
}
