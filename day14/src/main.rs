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

struct Cache<F>
where
    F: Fn(&[u8]) -> [u8; 16],
{
    salt_length: usize,
    salt_buffer: Vec<u8>,
    f: F,
    entries: Vec<Option<CacheEntry>>,
    start_suffix: usize,
}

impl<F> Cache<F>
where
    F: Fn(&[u8]) -> [u8; 16],
{
    fn new(salt: &str, f: F) -> Self {
        let mut entries = Vec::with_capacity(CACHE_SIZE);
        entries.resize_with(CACHE_SIZE, || None);
        Self {
            salt_length: salt.len(),
            salt_buffer: salt.as_bytes().to_vec(),
            f,
            entries,
            start_suffix: 0,
        }
    }

    fn apply(&mut self, suffix: usize) -> &CacheEntry {
        if suffix < self.start_suffix {
            panic!("Requested suffix {} is before the buffer start {}", suffix, self.start_suffix);
        }
        if suffix >= self.start_suffix + CACHE_SIZE {
            let idx = self.start_suffix % CACHE_SIZE;
            self.entries[idx] = None;
            self.start_suffix += 1;
        }
        let idx = suffix % CACHE_SIZE;
        if self.entries[idx].is_none() {
            self.salt_buffer.truncate(self.salt_length);
            self.salt_buffer.extend_from_slice(&suffix.to_string().as_bytes());
            let result = (self.f)(&self.salt_buffer);
            self.entries[idx] = Some(CacheEntry::new(&result));
        }
        self.entries[idx].as_ref().unwrap()
    }
}

struct CacheEntry {
    first_triplet: Option<u8>,
    quintuplets: u16,
}

impl CacheEntry {
    fn new(hash: &[u8]) -> Self {
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

        Self {
            first_triplet,
            quintuplets,
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

    fn find_index(&mut self, from: usize) -> usize {
        let mut suffix = from;
        loop {
            let first_triplet = self.cache.apply(suffix).first_triplet;
            if let Some(byte) = first_triplet {
                if self.five_in_a_row_in_next_thousand(suffix + 1, byte) {
                    return suffix;
                }
            }
            suffix += 1;
        }
    }

    fn five_in_a_row_in_next_thousand(&mut self, from: usize, byte: u8) -> bool {
        for i in from..from + 1000 {
            let quintuplets = &self.cache.apply(i).quintuplets;
            if (quintuplets & (1 << (byte as u16))) != 0 {
                return true;
            }
        }
        false
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
