use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

static INPUT: Lazy<Vec<Room>> =
    Lazy::new(|| common::read_file_as_elements("data/day04.txt").unwrap());

fn main() {
    println!("Part 1 {}", part1(&INPUT));
    //explore_part2(&input);
    println!("Part 2 {}", part2(&INPUT));
}

fn part1(input: &[Room]) -> u32 {
    input
        .iter()
        .filter(|r| r.is_real())
        .map(|r| r.sector_id)
        .sum()
}

fn part2(input: &[Room]) -> u32 {
    input
        .iter()
        .find(|r| r.is_real() && r.descrypt() == "northpole object storage")
        .unwrap()
        .sector_id
}

#[allow(dead_code)]
fn explore_part2(input: &[Room]) {
    input
        .iter()
        .filter(|r| r.is_real())
        .for_each(|r| println!("{}", r.descrypt()));
}

#[derive(Debug)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn is_real(&self) -> bool {
        let counts = self.count_letters();
        let checksum = checksum(counts);
        checksum == self.checksum
    }

    fn count_letters(&self) -> HashMap<char, u32> {
        let mut counts = HashMap::new();
        for c in self.name.chars().filter(|c| c.is_alphabetic()) {
            *counts.entry(c).or_insert(0) += 1;
        }
        counts
    }

    fn descrypt(&self) -> String {
        let key = (self.sector_id % 26) as u8;
        self.name
            .chars()
            .map(|c| {
                if c == '-' {
                    ' '
                } else {
                    let shifted = ((c as u8 - b'a' + key) % 26) + b'a';
                    shifted as char
                }
            })
            .collect()
    }
}

fn checksum(counters: HashMap<char, u32>) -> String {
    let mut entries: Vec<(char, u32)> = counters.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    entries.into_iter().take(5).map(|e| e.0).collect()
}

impl FromStr for Room {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([a-z-]+)-(\d+)\[([a-z]+)]$").unwrap();
        let caps = re.captures(s).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let sector_id = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let checksum = caps.get(3).unwrap().as_str();
        Ok(Room {
            name: name.to_string(),
            sector_id,
            checksum: checksum.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_real() {
        let room1 = Room::from_str("aaaaa-bbb-z-y-x-123[abxyz]").unwrap();
        assert!(room1.is_real());
        let room2 = Room::from_str("a-b-c-d-e-f-g-h-987[abcde]").unwrap();
        assert!(room2.is_real());
        let room3 = Room::from_str("not-a-real-room-404[oarel]").unwrap();
        assert!(room3.is_real());
        let room4 = Room::from_str("totally-real-room-200[decoy]").unwrap();
        assert!(!room4.is_real());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 158835);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 993);
    }
}
