use std::collections::VecDeque;
use md5::{Digest, Md5};
use once_cell::sync::Lazy;

static SALT: Lazy<String> = Lazy::new(|| common::read_file_as_string("data/day14.txt").unwrap());

fn main() {
    println!("Part 1: {}", part1(&SALT));
    println!("Part 1: {}", part2(&SALT));
}

fn part1(salt: &str) -> usize {
    let cache = Cache::new(salt, |buffer| Md5::digest(buffer).into());
    part(cache)
}

fn part2(salt: &str) -> usize {
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

const CACHE_SIZE: usize = 1001;

struct CacheEntry {
    suffix: usize,
    first_triplet: u8,
    quintuplets: u16,
}

impl CacheEntry {
    fn new(suffix: usize, hash: &[u8]) -> Option<Self> {
        let mut first_triplet = None;
        let mut quintuplets = 0_u16;
        let mut prev = None;
        let mut count = 1;
        for &byte in hash {
            for &nibble in &[byte >> 4, byte & 0x0F] {
                if Some(nibble) == prev {
                    count += 1;
                    if count == 3 && first_triplet.is_none() {
                        first_triplet = Some(nibble);
                    }
                    if count == 5 {
                        quintuplets |= 1 << (nibble as u16);
                    }
                } else {
                    prev = Some(nibble);
                    count = 1;
                }
            }
        }
        first_triplet.map(|triplet| Self {
            suffix,
            first_triplet: triplet,
            quintuplets,
        })
    }
}

struct Cache<F>
where
    F: Fn(&[u8]) -> [u8; 16],
{
    salt_length: usize,
    salt_buffer: Vec<u8>,
    f: F,
    entries: VecDeque<CacheEntry>,
    next_suffix: usize,
}

impl<F> Cache<F>
where
    F: Fn(&[u8]) -> [u8; 16],
{
    fn new(salt: &str, f: F) -> Self {
        Self {
            salt_length: salt.len(),
            salt_buffer: salt.as_bytes().to_vec(),
            f,
            entries: VecDeque::with_capacity(CACHE_SIZE),
            next_suffix: 0,
        }
    }

    // Genera y añade la siguiente entrada con tripleta
    fn next_entry(&mut self) -> Option<&CacheEntry> {
        loop {
            self.salt_buffer.truncate(self.salt_length);
            self.salt_buffer.extend_from_slice(self.next_suffix.to_string().as_bytes());
            let result = (self.f)(&self.salt_buffer);
            if let Some(entry) = CacheEntry::new(self.next_suffix, &result) {
                if self.entries.len() == CACHE_SIZE {
                    self.entries.pop_front();
                }
                self.entries.push_back(entry);
                self.next_suffix += 1;
                return self.entries.back();
            }
            self.next_suffix += 1;
        }
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

    fn find_index(&mut self, from_idx: usize) -> usize {
        // Avanza hasta encontrar el sufijo >= from_idx
        while self.cache.entries.front().map_or(true, |e| e.suffix < from_idx) {
            self.cache.next_entry();
        }
        let mut idx = 0;
        while let Some(entry) = self.cache.entries.get(idx) {
            if entry.suffix < from_idx {
                idx += 1;
                continue;
            }
            let entry_suffix = entry.suffix;
            let triplet = entry.first_triplet;
            // Libera la referencia a entry antes de mutar la cache
            while self.cache.entries.back().map_or(0, |e| e.suffix) < entry_suffix + 1000 {
                self.cache.next_entry();
            }
            for next in self.cache.entries.iter().skip(idx + 1) {
                if next.suffix > entry_suffix + 1000 { break; }
                if (next.quintuplets & (1 << (triplet as u16))) != 0 {
                    return entry_suffix;
                }
            }
            idx += 1;
        }
        unreachable!("No valid index found");
    }
}

fn stretched_hash(s: &[u8]) -> [u8; 16] {
    let mut hash = Md5::digest(s).to_vec();
    let mut hex = [0u8; 32];
    for _ in 0..2016 {
        // Convert hash to lowercase hex
        for (i, byte) in hash.iter().enumerate() {
            hex[2 * i] = b"0123456789abcdef"[(byte >> 4) as usize];
            hex[2 * i + 1] = b"0123456789abcdef"[(byte & 0x0F) as usize];
        }
        hash = Md5::digest(&hex).to_vec();
    }
    Md5::digest(&hex).into()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_find() {
        let cache = Cache::new("abc", |buffer| Md5::digest(buffer).into());
        let mut finder = KeyFinder::new(cache);
        assert_eq!(finder.find_index(0), 39);
        assert_eq!(finder.find_index(40), 92);
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part1("abc"), 22728);
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
