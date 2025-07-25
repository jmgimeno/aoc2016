use once_cell::sync::Lazy;
use regex::{Error, Regex};
use std::str::FromStr;

static DISKS: Lazy<Vec<Disk>> = Lazy::new(||
    common::read_file_as_elements("data/day15.txt").unwrap()
);

fn main() {
    println!("Part 1: {}", part1(&DISKS));
}

fn part1(disks: &[Disk]) -> u32 {
    let eqns = disks.iter().map(|d| d.into()).collect::<Vec<_>>();
    let result = solve_many(&eqns);
    println!("{:?}", result);
    result.remainder as u32
}

#[derive(Clone, Debug)]
struct Disk {
    number: u32,
    number_of_positions: u32,
    position_at_zero: u32,
}

impl Disk {
    fn new(number: u32, number_of_positions: u32, position_at_zero: u32) -> Self {
        Self {
            number,
            number_of_positions,
            position_at_zero,
        }
    }
}

#[derive(Debug)]
enum DiskParseError {
    RegexError(regex::Error),
    IntError(std::num::ParseIntError),
    InvalidInput,
}

impl From<regex::Error> for DiskParseError {
    fn from(value: Error) -> Self {
        DiskParseError::RegexError(value)
    }
}

impl From<std::num::ParseIntError> for DiskParseError {
    fn from(value: std::num::ParseIntError) -> Self {
        DiskParseError::IntError(value)
    }
}

impl FromStr for Disk {
    type Err = DiskParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regexp = Regex::new(r"^Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).$")?;
        if let Some(caps) = regexp.captures(s) {
            let number = caps[1].parse::<u32>()?;
            let number_of_positions = caps[2].parse::<u32>()?;
            let position_at_zero = caps[3].parse::<u32>()?;
            Ok(Self::new(number, number_of_positions, position_at_zero))
        } else {
            Err(DiskParseError::InvalidInput)
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct CongruenceEquation {
    remainder: i32,
    modulus: u32,
}

impl From<&Disk> for CongruenceEquation {
    // Disk #n has P positions, at t=0 is at p0
    // - The ball arrives at t' = t + n
    // - The position will be p = (p0 + t'= % P = (p0 + t + n) % P
    // - We want p = 0, so we need (p0 + t + n) % P = 0
    //      => so t = -(p0 + n) % P
    fn from(value: &Disk) -> Self {
        Self {
            remainder: - (value.position_at_zero as i32 + value.number as i32),
            modulus: value.number_of_positions,
        }.normalize()
    }
}

impl CongruenceEquation {
    fn normalize(&self) -> Self {
        let x = self.remainder % self.modulus as i32;
        let x = if x >= 0 { x } else { x + self.modulus as i32 };
        Self { remainder: x, modulus: self.modulus }
    }
}

fn solve_two(eq1: CongruenceEquation, eq2: CongruenceEquation) -> CongruenceEquation {
    let CongruenceEquation { remainder: a_1, modulus: n_1 } = eq1;
    let CongruenceEquation { remainder: a_2, modulus: n_2 } = eq2;
    let (gcd, m_1, m_2) = extended_euclidean_algorithm(n_1, n_2);
    assert_eq!(gcd, 1);
    assert_eq!(m_1 * n_1 as i32 + m_2 * n_2 as i32, 1);
    let x = a_1 * m_2 * n_2 as i32 + a_2 * m_1 * n_1 as i32;
    let y = n_1 * n_2;
    CongruenceEquation { remainder: x, modulus: y }
}

fn solve_many(eqns: &[CongruenceEquation]) -> CongruenceEquation {
    let mut result = eqns[0];
    for eqn in eqns.iter().skip(1) {
        result = solve_two(result, *eqn).normalize();
    }
    result
}

fn extended_euclidean_algorithm(a: u32, b: u32) -> (u32, i32, i32) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1_i32;
    let mut s = 0_i32;
    let mut old_t = 0_i32;
    let mut t = 1_i32;
    while r != 0 {
        let quotient = old_r / r;
        let new_r = old_r - quotient * r;
        old_r = r;
        r = new_r;
        let new_s = old_s - quotient as i32 * s;
        old_s = s;
        s = new_s;
        let new_t = old_t - quotient as i32 * t;
        old_t = t;
        t = new_t;
    }
    (old_r, old_s, old_t)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_solve_two() {
        let disk1 = Disk::new(1, 5, 4);
        let disk2 = Disk::new(2, 2, 1);
        let eqn1 = CongruenceEquation::from(&disk1);
        let eqn2 = CongruenceEquation::from(&disk2);
        let result = solve_two(eqn1, eqn2).normalize();
        assert_eq!(result.remainder, 5);
        assert_eq!(result.modulus, 10);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&DISKS), 121834);
    }

}
