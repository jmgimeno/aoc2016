use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::str::FromStr;

static INPUT: Lazy<Vec<IPv7Address>> =
    Lazy::new(|| common::read_file_as_elements("data/day07.txt").unwrap());

fn main() {
    println!("Part 1: {}", part1(&INPUT));
    println!("Part 2: {}", part2(&INPUT));
}

fn part1(input: &[IPv7Address]) -> usize {
    input.iter().filter(|ip| ip.supports_tls()).count()
}

fn part2(input: &[IPv7Address]) -> usize {
    input.iter().filter(|ip| ip.supports_ssl()).count()
}

#[derive(Debug, Clone)]
struct IPv7Address {
    supernet: Vec<String>,
    hypernet: Vec<String>,
}

impl FromStr for IPv7Address {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut supernet = Vec::new();
        let mut hypernet = Vec::new();
        let mut current = String::new();
        for c in s.chars() {
            if c == '[' {
                if !current.is_empty() {
                    supernet.push(current);
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
            supernet.push(current);
        }
        Ok(Self { supernet, hypernet })
    }
}

impl IPv7Address {
    fn supports_tls(&self) -> bool {
        self.supernet.iter().any(|s| IPv7Address::is_abba(s))
            && self.hypernet.iter().all(|s| !IPv7Address::is_abba(s))
    }

    fn is_abba(s: &str) -> bool {
        s.as_bytes()
            .windows(4)
            .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
    }

    fn supports_ssl(&self) -> bool {
        let all_abas_supernet = IPv7Address::all_triplets(&self.supernet);
        let all_babs_hypernet = IPv7Address::all_triplets(&self.hypernet);
        let all_babs_supernet = IPv7Address::invert_triplets(&all_abas_supernet);
        all_babs_supernet.intersection(&all_babs_hypernet).count() > 0
    }

    fn all_triplets(input: &[String]) -> HashSet<(u8, u8, u8)> {
        input
            .iter()
            .flat_map(|s| IPv7Address::get_triplets(s))
            .collect()
    }

    fn get_triplets(s: &str) -> impl Iterator<Item = (u8, u8, u8)>  {
        s.as_bytes()
            .windows(3)
            .filter(|w| w[0] == w[2] && w[0] != w[1])
            .map(|w| (w[0], w[1], w[2]))
    }

    fn invert_triplets(abas: &HashSet<(u8, u8, u8)>) -> HashSet<(u8, u8, u8)> {
        abas.iter().map(|ab| (ab.1, ab.0, ab.1)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(s: &str) -> IPv7Address {
        s.parse().unwrap()
    }

    #[test]
    fn test_supports_tls() {
        assert!(ip("abba[mnop]qrst").supports_tls());
        assert!(!ip("abcd[bddb]xyyx").supports_tls());
        assert!(!ip("aaaa[qwer]tyui").supports_tls());
        assert!(ip("ioxxoj[asdfgh]zxcvbn").supports_tls());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 115);
    }

    #[test]
    fn test_supports_ssl() {
        assert!(ip("aba[bab]xyz").supports_ssl());
        assert!(!ip("xyx[xyx]xyx").supports_ssl());
        assert!(ip("aaa[kek]eke").supports_ssl());
        assert!(ip("zazbz[bzb]cdb").supports_ssl());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 231);
    }
}
