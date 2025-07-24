use std::str::from_utf8;
use md5::{Digest, Md5};
use once_cell::sync::Lazy;

static SALT: Lazy<String> = Lazy::new(|| common::read_file_as_string("data/day14.txt").unwrap());

fn main() {
    println!("Part 1: {}", part1(&SALT));
    println!("Part 1: {}", part2(&SALT));
}

fn part1(salt: &String) -> usize {
    let cache = Cache::new(salt, |buffer| Md5::digest(buffer).into());
    part(cache)
}

fn part2(salt: &String) -> usize {
    let cache = Cache::new(salt, |buffer| stretched_hash(buffer).into());
    part(cache)
}

fn part<F>(cache: Cache<F>) -> usize
where
    F: Fn(&[u8]) -> [u8; 16],
{
    let mut finder = KeyFinder::new(cache);
    let mut from = 0;
    for _ in 0..64 {
        from = finder.find_index(from);
        from += 1;
    }
    from - 1
}

struct Cache<F>
where
    F: Fn(&[u8]) -> [u8; 16],
{
    salt: String,
    f: F,
    cached: Vec<[u8; 16]>,
}

impl<F> Cache<F>
where
    F: Fn(&[u8]) -> [u8; 16],
{
    fn new(salt: &String, f: F) -> Self {
        Self { salt: salt.clone() , f , cached: Vec::new()}
    }

    fn apply(&mut self, suffix: usize) -> [u8; 16] {
        if suffix < self.cached.len() {
            return self.cached[suffix];
        }
        let result = (self.f)(format!("{}{}", self.salt, suffix).as_bytes());
        self.cached.push(result);
        result
    }
}

struct KeyFinder<F>
where
    F: Fn(&[u8]) -> [u8; 16],
{
    cache: Cache<F>,
}

impl<F> KeyFinder<F>
where
    F: Fn(&[u8]) -> [u8; 16],
{
    fn new(cache: Cache<F>) -> Self {
        Self { cache }
    }

    fn find_index(&mut self, from: usize) -> usize {
        let mut suffix = from;
        loop {
            let hash: [u8; 16] = self.cache.apply(suffix);
            if let Some(byte) = three_in_a_row(&hash) {
                if self.five_in_a_row_in_next_thousand(suffix + 1, byte) {
                    return suffix;
                }
            }
            suffix += 1;
        }
    }

    fn five_in_a_row_in_next_thousand(&mut self, from: usize, byte: u8) -> bool {
        for i in from..from + 1000 {
            let hash: [u8; 16] = self.cache.apply(i);
            if five_in_a_row(&hash, byte) {
                return true;
            }
        }
        false
    }
}


fn three_in_a_row(hash: &[u8]) -> Option<u8> {
    let mut prev = None;
    let mut count = 1;
    for &byte in hash {
        for &nibble in &[byte >> 4, byte & 0x0F] {
            if Some(nibble) == prev {
                count += 1;
                if count == 3 {
                    return Some(nibble);
                }
            } else {
                prev = Some(nibble);
                count = 1;
            }
        }
    }
    None
}

fn five_in_a_row(hash: &[u8], target: u8) -> bool {
    let mut prev = None;
    let mut count = 1;
    for &byte in hash {
        for &nibble in &[byte >> 4, byte & 0x0F] {
            if Some(nibble) == prev {
                count += 1;
                if count == 5 {
                    if nibble == target {
                        return true;
                    }
                }
            } else {
                prev = Some(nibble);
                count = 1;
            }
        }
    }
    false
}

fn stretched_hash(s: &[u8]) -> [u8; 16] {
    let mut current = from_utf8(s).unwrap().to_string();
    for _ in 0..2016 {
        let hash = Md5::digest(current.as_bytes());
        current = format!("{:x}", hash);
    }
    Md5::digest(current.as_bytes()).into()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_find() {
        let cache = Cache::new(&"abc".to_string(), |buffer| Md5::digest(buffer).into());
        let mut finder = KeyFinder::new(cache);
        assert_eq!(finder.find_index(0), 39);
        assert_eq!(finder.find_index(40), 92);
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(&"abc".to_string()), 22728);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SALT), 23890);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SALT), 22696);
    }
}
