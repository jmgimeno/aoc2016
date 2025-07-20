use once_cell::sync::Lazy;

static INPUT: Lazy<String> = Lazy::new(|| common::read_file_as_string("data/day09.txt").unwrap());

fn main() {
    println!("Part 1: {}", part1(&INPUT));
    println!("Part 2: {}", part2(&INPUT));
}

fn part1(input: &str) -> usize {
    part(input.as_bytes(), |r| r.len() )
}

fn part2(input: &str) -> usize {
    part2_bytes(input.as_bytes())
}

fn part2_bytes(input: &[u8]) -> usize {
    part(input, |r| part2_bytes(r) )
}

fn part<F>(input: &[u8], count: F) -> usize
where
    F: Fn(&[u8]) -> usize,
{
    enum ParserState {
        Regular(usize),
        FirstNum(usize),
        SecondNum(usize, usize),
    }

    let mut rest = input;
    let mut result = 0;
    let mut state = ParserState::Regular(0);
    while !rest.is_empty() {
        let current = rest[0];
        rest = &rest[1..];
        match state {
            ParserState::Regular(x) => match current {
                b'(' => {
                    result += x;
                    state = ParserState::FirstNum(0);
                }
                _ => state = ParserState::Regular(x + 1),
            },
            ParserState::FirstNum(x) => match current {
                b'x' => {
                    state = ParserState::SecondNum(x, 0);
                }
                c => {
                    let digit = c - b'0';
                    state = ParserState::FirstNum(10 * x + digit as usize);
                }
            },
            ParserState::SecondNum(x, y) => match current {
                b')' => {
                    result += y * count(&rest[..x]);
                    state = ParserState::Regular(0);
                    rest = &rest[x..];
                }
                c => {
                    let digit = c - b'0';
                    state = ParserState::SecondNum(x, 10 * y + digit as usize);
                }
            },
        }
    }
    if let ParserState::Regular(x) = state {
        return result + x;
    }
    unreachable!("should have gotten a state");
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

    #[test]
    fn test_examples_part2() {
        assert_eq!(part2("ADVENT"), 6);
        assert_eq!(part1("A(1x5)BC"), 7);
        assert_eq!(part1("(3x3)XYZ"), 9);
        assert_eq!(part2("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(
            part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 10931789799);
    }
}
