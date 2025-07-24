use md5::{Digest, Md5};
use once_cell::sync::Lazy;

static SALT: Lazy<String> = Lazy::new(|| common::read_file_as_string("data/day14.txt").unwrap());

fn main() {
    println!("Part 1: {}", part1(&SALT));
}

fn part1(salt: &str) -> usize {
    let finder = KeyFinder::new(salt.to_string());
    let mut from = 0;
    for _ in 0..64 {
        from = finder.find_index(from);
        from += 1;
    }
    from - 1
}

struct KeyFinder {
    salt: String,
}

impl KeyFinder {
    fn new(salt: String) -> Self {
        Self { salt }
    }

    fn find_index(&self, from: usize) -> usize {
        let mut buffer = self.salt.as_bytes().to_vec();
        let mut suffix = from;
        loop {
            buffer.truncate(self.salt.len());
            let suffix_str = suffix.to_string();
            buffer.extend_from_slice(suffix_str.as_bytes());
            let hash: [u8; 16] = Md5::digest(&buffer).into();
            if let Some(byte) = KeyFinder::three_in_a_row(&hash) {
                if self.five_in_a_row_in_next_thousand(suffix + 1, byte) {
                    return suffix;
                }
            }
            suffix += 1;
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

    fn five_in_a_row_in_next_thousand(&self, from: usize, byte: u8) -> bool {
        let mut buffer = self.salt.as_bytes().to_vec();
        for i in from..from + 1000 {
            buffer.truncate(self.salt.len());
            buffer.extend_from_slice(i.to_string().as_bytes());
            let hash: [u8; 16] = Md5::digest(&buffer).into();
            if KeyFinder::five_in_a_row(&hash, byte) {
                return true;
            }
        }
        false
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
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_find() {
        let finder = KeyFinder::new("abc".to_string());
        assert_eq!(finder.find_index(0), 39);
        assert_eq!(finder.find_index(40), 92);
    }

    #[test]
    fn test_part1() {
        let finder = KeyFinder::new("abc".to_string());
        assert_eq!(part1("abc"), 22728);
    }
}
