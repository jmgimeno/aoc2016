use once_cell::sync::Lazy;

static INPUT: Lazy<String> = Lazy::new(|| common::read_file_as_string("data/day16.txt").unwrap());

fn main() {
    println!("Part 1: {}", checksum(&INPUT, 272));
    println!("Part 1: {}", checksum(&INPUT, 35651584));
}

fn checksum(input: &str, length: usize) -> String {
    let dragon = Dragon::new(input.to_string(), length);
    let size = window_size(length);
    let mut result = String::new();
    let mut buffer = vec!['0'; size];
    let mut iter = dragon.iter_forward().take(length);
    let num_windows = length / size;

    for _ in 0..num_windows {
        for i in 0..size {
            buffer[i] = iter.next().unwrap();
        }
        let mut n = size;
        while n > 1 {
            for i in 0..n / 2 {
                buffer[i] = if buffer[2 * i] == buffer[2 * i + 1] {
                    '1'
                } else {
                    '0'
                };
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

    fn iter_forward(&self) -> DragonForwardIter<'_> {
        DragonForwardIter::new(self)
    }

    fn iter_backward(&self) -> DragonBackwardIter<'_> {
        DragonBackwardIter::new(self)
    }
}

// Forward iterator
pub struct DragonForwardIter<'a> {
    stack: Vec<DragonForwardFrame<'a>>,
}

enum DragonForwardFrame<'a> {
    Base(&'a str, usize),
    Step {
        left: Box<DragonForwardIter<'a>>,
        middle: bool,
        right: Box<DragonBackwardIter<'a>>,
    },
}

impl<'a> DragonForwardIter<'a> {
    pub fn new(dragon: &'a Dragon) -> Self {
        let frame = match dragon {
            Dragon::Base(s) => DragonForwardFrame::Base(s, 0),
            Dragon::Step(d) => DragonForwardFrame::Step {
                left: Box::new(DragonForwardIter::new(d)),
                middle: false,
                right: Box::new(DragonBackwardIter::new(d)),
            },
        };
        Self { stack: vec![frame] }
    }
}

impl<'a> Iterator for DragonForwardIter<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        while let Some(frame) = self.stack.last_mut() {
            match frame {
                DragonForwardFrame::Base(s, pos) => {
                    if *pos < s.len() {
                        let c = s.as_bytes()[*pos];
                        *pos += 1;
                        return Some(c as char);
                    } else {
                        self.stack.pop();
                    }
                }
                DragonForwardFrame::Step { left, middle, right } => {
                    if let Some(c) = left.next() {
                        return Some(c);
                    }
                    if !*middle {
                        *middle = true;
                        return Some('0');
                    }
                    if let Some(c) = right.next() {
                        // flip bits for right half
                        return Some(if c == '0' { '1' } else { '0' });
                    }
                    self.stack.pop();
                }
            }
        }
        None
    }
}

// Backward iterator
pub struct DragonBackwardIter<'a> {
    stack: Vec<DragonBackwardFrame<'a>>,
}

enum DragonBackwardFrame<'a> {
    Base(&'a str, isize),
    Step {
        right: Box<DragonForwardIter<'a>>,
        middle: bool,
        left: Box<DragonBackwardIter<'a>>,
    },
}

impl<'a> DragonBackwardIter<'a> {
    pub fn new(dragon: &'a Dragon) -> Self {
        let frame = match dragon {
            Dragon::Base(s) => DragonBackwardFrame::Base(s, s.len() as isize - 1),
            Dragon::Step(d) => DragonBackwardFrame::Step {
                right: Box::new(DragonForwardIter::new(d)),
                middle: false,
                left: Box::new(DragonBackwardIter::new(d)),
            },
        };
        Self { stack: vec![frame] }
    }
}

impl<'a> Iterator for DragonBackwardIter<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        while let Some(frame) = self.stack.last_mut() {
            match frame {
                DragonBackwardFrame::Base(s, pos) => {
                    if *pos >= 0 {
                        let c = s.as_bytes()[*pos as usize];
                        *pos -= 1;
                        return Some(c as char);
                    } else {
                        self.stack.pop();
                    }
                }
                DragonBackwardFrame::Step { right, middle, left } => {
                    if let Some(c) = right.next() {
                        return Some(if c == '0' { '1' } else { '0' });
                    }
                    if !*middle {
                        *middle = true;
                        return Some('0');
                    }
                    if let Some(c) = left.next() {
                        return Some(c);
                    }
                    self.stack.pop();
                }
            }
        }
        None
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
    use crate::{Dragon, INPUT, checksum};

    #[test]
    fn test_iteration_example1() {
        let d1 = Dragon::Base("1".to_string());
        let r1 = d1.iter_forward().collect::<String>();
        assert_eq!(r1, "1".to_string());
        assert_eq!(d1.len(), r1.len());
    }

    #[test]
    fn test_iteration_example2() {
        let d2 = Dragon::Step(Box::new(Dragon::Base("0".to_string())));
        let r2 = d2.iter_forward().collect::<String>();
        assert_eq!(r2, "001".to_string());
        assert_eq!(d2.len(), r2.len());
    }

    #[test]
    fn test_generation() {
        let d = Dragon::new("10000".to_string(), 20);
        let r = d.iter_forward().take(20).collect::<String>();
        assert_eq!(r, "10000011110010000111".to_string());
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(checksum("10000", 20), "01100".to_string());
    }

    #[test]
    fn test_part1() {
        assert_eq!(checksum(&INPUT, 272), "11100111011101111".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(checksum(&INPUT, 35651584), "10001110010000110".to_string());
    }
}
