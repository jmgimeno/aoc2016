use once_cell::sync::Lazy;
use std::str::FromStr;

static INPUT: Lazy<Vec<IPv7Address>> =
    Lazy::new(|| common::read_file_as_elements("data/day07.txt").unwrap());

fn main() {
    println!("Part 1: {}", part1(&INPUT));
}

fn part1(input: &[IPv7Address]) -> usize {
    input.iter().filter(|ip| ip.supports_tls()).count()
}

#[derive(Debug, Clone)]
struct IPv7Address {
    net: Vec<String>,
    hypernet: Vec<String>,
}

impl FromStr for IPv7Address {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut net = Vec::new();
        let mut hypernet = Vec::new();
        let mut current = String::new();
        for c in s.chars() {
            if c == '[' {
                if !current.is_empty() {
                    net.push(current);
                    current = String::new();
                }
            } else if c == ']' {
                if !current.is_empty() {
                    hypernet.push(current);
                    current = String::new();
                }
            } else {
                current.push(c);
            }
        }
        if !current.is_empty() {
            net.push(current);
        }
        Ok(Self { net, hypernet })
    }
}

impl IPv7Address {
    fn supports_tls(&self) -> bool {
        self.net.iter().any(|s| IPv7Address::is_abba(s))
            && self.hypernet.iter().all(|s| !IPv7Address::is_abba(s))
    }

    fn is_abba(s: &str) -> bool {
        s.as_bytes()
            .windows(4)
            .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let ip = "abba[mnop]qrst".parse::<IPv7Address>().unwrap();
        assert!(ip.supports_tls());
    }

    #[test]
    fn test_example2() {
        let ip = "abcd[bddb]xyyx".parse::<IPv7Address>().unwrap();
        assert!(!ip.supports_tls());
    }

    #[test]
    fn test_example3() {
        let ip = "aaaa[qwer]tyui".parse::<IPv7Address>().unwrap();
        assert!(!ip.supports_tls());
    }

    #[test]
    fn test_example4() {
        let ip = "ioxxoj[asdfgh]zxcvbn".parse::<IPv7Address>().unwrap();
        assert!(ip.supports_tls());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 115);
    }
}
