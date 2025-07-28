use once_cell::sync::Lazy;
use std::num::ParseIntError;
use std::str::FromStr;

static IP_RANGES: Lazy<Vec<IPRange>> = Lazy::new(|| {
    let mut ip_ranges = common::read_file_as_elements("data/day20.txt").unwrap();
    ip_ranges.sort();
    ip_ranges
});

fn main() {
    println!("Part 1: {}", part1(&IP_RANGES));
    println!("Part 1: {}", part2(&IP_RANGES));
}

fn part1(ip_ranges: &[IPRange]) -> u32 {
    let mut possible_start = 0;
    for IPRange { begin, end } in ip_ranges {
        if possible_start < *begin {
            return possible_start;
        }
        possible_start = u32::max(possible_start, end.saturating_add(1));
    }
    unreachable!("Should never reach here")
}

fn part2(ip_ranges: &[IPRange]) -> u32 {
    let mut count_ips = 0;
    let mut possible_start = 0;
    for IPRange { begin, end } in ip_ranges {
        if possible_start < *begin {
            count_ips += *begin - possible_start;
        }
        possible_start = u32::max(possible_start, end.saturating_add(1));
    }
    count_ips += u32::MAX - possible_start;
    count_ips
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct IPRange {
    begin: u32,
    end: u32,
}

impl FromStr for IPRange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let begin = parts.next().unwrap().parse::<u32>()?;
        let end = parts.next().unwrap().parse::<u32>()?;
        Ok(IPRange { begin, end })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let mut ranges = vec![
            IPRange { begin: 5, end: 8 },
            IPRange { begin: 0, end: 2 },
            IPRange { begin: 4, end: 7 },
        ];
        ranges.sort();
        assert_eq!(part1(&ranges), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&IP_RANGES), 19449262);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&IP_RANGES), 119);
    }
}
