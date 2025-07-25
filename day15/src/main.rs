use once_cell::sync::Lazy;
use regex::{Error, Regex};
use std::str::FromStr;

static DISKS: Lazy<Vec<Disk>> = Lazy::new(||
    common::read_file_as_elements("data/day15.txt").unwrap()
);

fn main() {
    println!("Part 1: {}", part1(&DISKS));
    println!("Part 2: {}", part2(&DISKS));
}

fn part1(disks: &[Disk]) -> u64 {
    let eqns = disks.iter().map(|d| d.into()).collect::<Vec<_>>();
    let result = solve_many(&eqns);
    result.remainder as u64
}

fn part2(disks: &[Disk]) -> u64 {
    let mut eqns = disks.iter().map(|d| d.into()).collect::<Vec<_>>();
    let new_disk = Disk::new(eqns.len() as u64 + 1, 11, 0);
    eqns.push((&new_disk).into());
    let result = solve_many(&eqns);
    result.remainder as u64
}

#[derive(Clone, Debug)]
struct Disk {
    number: u64,
    number_of_positions: u64,
    position_at_zero: u64,
}

impl Disk {
    fn new(number: u64, number_of_positions: u64, position_at_zero: u64) -> Self {
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

static DISK_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).$").unwrap()
});

impl FromStr for Disk {
    type Err = DiskParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = DISK_REGEX.captures(s) {
            let number = caps.get(1).unwrap().as_str().parse::<u64>()?;
            let number_of_positions = caps.get(2).unwrap().as_str().parse::<u64>()?;
            let position_at_zero = caps.get(3).unwrap().as_str().parse::<u64>()?;
            Ok(Self::new(number, number_of_positions, position_at_zero))
        } else {
            Err(DiskParseError::InvalidInput)
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct CongruenceEquation {
    remainder: i64,
    modulus: u64,
}

impl From<&Disk> for CongruenceEquation {
    // Disk #n has P positions, at t=0 is at p0
    // - The ball arrives at t' = t + n
    // - The position will be p = (p0 + t'= % P = (p0 + t + n) % P
    // - We want p = 0, so we need (p0 + t + n) % P = 0
    //      => so t = -(p0 + n) % P
    fn from(value: &Disk) -> Self {
        Self {
            remainder: - (value.position_at_zero as i64 + value.number as i64),
            modulus: value.number_of_positions,
        }.normalize()
    }
}

impl CongruenceEquation {
    fn normalize(&self) -> Self {
        let x = self.remainder % self.modulus as i64;
        let x = if x >= 0 { x } else { x + self.modulus as i64 };
        Self { remainder: x, modulus: self.modulus }
    }
}

fn solve_two(eq1: CongruenceEquation, eq2: CongruenceEquation) -> CongruenceEquation {
    let CongruenceEquation { remainder: a_1, modulus: n_1 } = eq1;
    let CongruenceEquation { remainder: a_2, modulus: n_2 } = eq2;
    let (gcd, m_1, m_2) = extended_euclidean_algorithm(n_1, n_2);
    assert_eq!(gcd, 1, "GCD of {} and {} is not 1", n_1, n_2);
    let x = a_1 * m_2 * n_2 as i64 + a_2 * m_1 * n_1 as i64;
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

fn extended_euclidean_algorithm(a: u64, b: u64) -> (u64, i64, i64) {
    let (mut old_r, mut r) = (a as i64, b as i64);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    (old_r as u64, old_s, old_t)
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&DISKS), 3208099);
    }
}
