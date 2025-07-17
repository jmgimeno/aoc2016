use md5::{Digest, Md5};
use once_cell::sync::Lazy;

static INPUT: Lazy<String> = Lazy::new(|| common::read_file_as_string("data/day05.txt").unwrap());

fn main() {
    println!("Part 1: {}", part1(&INPUT));
    println!("Part 2: {}", part2(&INPUT));
}

fn part1(input: &str) -> String {
    let mut result = String::new();
    let mut buffer = input.as_bytes().to_vec();
    let mut suffix = 0;
    while result.len() < 8 {
        let suffix_str = suffix.to_string();
        buffer.truncate(input.len());
        buffer.extend_from_slice(suffix_str.as_bytes());
        let hash = Md5::digest(&buffer);
        if hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0 {
            result.push(hex_digit_char(hash[2] & 0x0F));
        }
        suffix += 1;
    }
    result
}

fn part2(input: &str) -> String {
    let mut found = [false; 8];
    let mut result = [0u8; 8];
    let mut buffer = input.as_bytes().to_vec();
    let mut suffix = 0;
    let mut counter = 0;
    while counter < 8 {
        let suffix_str = suffix.to_string();
        buffer.truncate(input.len());
        buffer.extend_from_slice(suffix_str.as_bytes());
        let hash = Md5::digest(&buffer);
        if hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0 {
            let index = (hash[2] & 0x0F) as usize;
            if index < 8 && !found[index] {
                found[index] = true;
                result[index] = (hash[3] & 0xF0) >> 4;
                counter += 1;
            }
        }
        suffix += 1;
    }
    result
        .iter()
        .map(|x| hex_digit_char(*x))
        .collect::<String>()
}

fn hex_digit_char(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + (n - 10)) as char,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(part1("abc"), "18f47a30");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), "f97c354d");
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2("abc"), "05ace8e3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), "863dde27");
    }
}
