use once_cell::sync::Lazy;
use std::str::FromStr;

static INPUT: Lazy<Vec<Triplet>> =
    Lazy::new(|| common::read_file_as_elements("data/day03.txt").unwrap());

fn main() {
    println!("Part1 : {}", part1(&INPUT));
    println!("Part1 : {}", part2(&INPUT));
}

fn part1(input: &[Triplet]) -> usize {
    input.iter().filter(|t| t.is_triangle()).count()
}

fn part2(input: &[Triplet]) -> usize {
    let transposed = transpose_group_of_three(&input);
    part1(&transposed)
}

fn transpose_group_of_three(input: &[Triplet]) -> Vec<Triplet> {
    let mut result = Vec::with_capacity(input.len());
    for chunk in input.chunks(3) {
        if chunk.len() == 3 {
            let (a, b, c) = (&chunk[0], &chunk[1], &chunk[2]);
            result.push(Triplet(a.0, b.0, c.0));
            result.push(Triplet(a.1, b.1, c.1));
            result.push(Triplet(a.2, b.2, c.2));
        }
    }
    result
}

#[derive(Debug)]
struct Triplet(u32, u32, u32);

impl Triplet {
    fn is_triangle(&self) -> bool {
        self.0 + self.1 > self.2 && self.1 + self.2 > self.0 && self.0 + self.2 > self.1
    }
}

impl FromStr for Triplet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //  775  785  361
        let mut parts = s.split_whitespace();
        Ok(Self(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 1032);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 1838);
    }
}
