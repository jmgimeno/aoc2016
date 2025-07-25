use itertools::Itertools;
use once_cell::sync::Lazy;

static INPUT: Lazy<String> = Lazy::new(|| {
    common::read_file_as_string("data/day16.txt").unwrap()
});

fn main() {
    println!("Part 1: {}", checksum(&INPUT, 272));
    println!("Part 1: {}", checksum(&INPUT, 35651584));
}

fn checksum(input: &str, length: usize) -> String {
    let dragon = Dragon::new(input.to_string(), length);
    let size = window_size(length);
    let mut result = String::new();
    let mut buffer = vec!['0'; size];

    let mut iter = dragon.chars().take(length);
    let num_windows = length / size;

    for _ in 0..num_windows {
        for i in 0..size {
            buffer[i] = iter.next().unwrap();
        }
        let mut n = size;
        while n > 1 {
            for i in 0..n / 2 {
                buffer[i] = if buffer[2 * i] == buffer[2 * i + 1] { '1' } else { '0' };
            }
            n /= 2;
        }
        result.push(buffer[0]);
    }
    result
}

enum Dragon {
    Base(String),
    Step(Box<Dragon>),
}

impl Dragon {

    fn new(s: String, min_length: usize) -> Self {
        let mut new_dragon = Self::Base(s);
        while new_dragon.len() < min_length {
            new_dragon = Self::Step(Box::new(new_dragon));
        }
        new_dragon
    }

    fn len(&self) -> usize {
        match self {
            Dragon::Base(s) => s.len(),
            Dragon::Step(d) => 2 * d.len() + 1,
        }
    }

    fn chars(&self) -> DragonIter<'_> {
        DragonIter::new(self)
    }
}

struct DragonIter<'a> {
    stack: Vec<Box<dyn Iterator<Item = char> + 'a>>,
}

impl<'a> DragonIter<'a> {
    fn new(dragon: &'a Dragon) -> Self {
        let mut stack = Vec::new();
        stack.push(Box::new(dragon.iter_chars()) as Box<dyn Iterator<Item = char> + 'a>);
        DragonIter { stack }
    }
}

impl<'a> Iterator for DragonIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(top) = self.stack.last_mut() {
            if let Some(c) = top.next() {
                return Some(c);
            } else {
                self.stack.pop();
            }
        }
        None
    }
}

trait DragonCharIter<'a> {
    fn iter_chars(&'a self) -> Box<dyn Iterator<Item = char> + 'a>;
}

impl<'a> DragonCharIter<'a> for Dragon {
    fn iter_chars(&'a self) -> Box<dyn Iterator<Item = char> + 'a> {
        match self {
            Dragon::Base(s) => Box::new(s.chars()),
            Dragon::Step(d) => {
                let left = d.iter_chars();
                let middle = std::iter::once('0');
                let right = d
                    .iter_chars()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|c| if c == '0' { '1' } else { '0' });
                Box::new(left.chain(middle).chain(right))
            }
        }
    }
}

fn window_size(mut length: usize) -> usize {
    let mut size = 1;
    while length % 2 == 0 && length > 0 {
        size *= 2;
        length /= 2;
    }
    size
}

#[cfg(test)]
mod tests {
    use crate::{checksum, Dragon};

    #[test]
    fn test_iteration_example1() {
        let d1 = Dragon::Base("1".to_string());
        let r1 = d1.chars().collect::<String>();
        assert_eq!(r1, "1".to_string());
        assert_eq!(d1.len(), r1.len());
    }

    #[test]
    fn test_iteration_example2() {
        let d2 = Dragon::Step(Box::new(Dragon::Base("0".to_string())));
        let r2 = d2.chars().collect::<String>();
        assert_eq!(r2, "001".to_string());
        assert_eq!(d2.len(), r2.len());
    }

    #[test]
    fn test_generation() {
        let d = Dragon::new("10000".to_string(), 20);
        let r = d.chars().take(20).collect::<String>();
        assert_eq!(r, "10000011110010000111".to_string());
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(checksum("10000", 20), "01100".to_string());
    }
}
