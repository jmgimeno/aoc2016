use once_cell::sync::Lazy;

static INPUT: Lazy<String> = Lazy::new(|| common::read_file_as_string("data/day16.txt").unwrap());

fn main() {
    println!("Part 1: {}", fill_and_checksum(&INPUT, 272));
    println!("Part 2: {}", fill_and_checksum(&INPUT, 35651584));
}

fn fill_and_checksum(input: &str, min_size: usize) -> String {
    let mut dragon_iterator = create_dragon_iterator(input, min_size);
    checksum(&mut dragon_iterator, min_size)
}

fn create_dragon_iterator(input: &str, min_size: usize) -> impl Iterator<Item = char> {
    let levels = needed_levels(input, min_size);
    DragonIterator::new(input, levels).take(min_size)
}

fn checksum(dragon_iterator: &mut impl Iterator<Item = char>, size: usize) -> String {
    let window_size = window_size(size);
    let num_windows = size / window_size;
    let mut buffer = vec!['0'; window_size];
    let mut result = String::new();

    for _ in 0..num_windows {
        for i in 0..window_size {
            buffer[i] = dragon_iterator.next().unwrap();
        }
        let mut n = window_size;
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

struct DragonIterator<'a> {
    stack: Vec<(&'a str, usize, usize, bool)>,
}

impl<'a> DragonIterator<'a> {
    fn new(seed: &'a str, level: usize) -> Self {
        Self {
            stack: vec![(seed, level, 0, false)],
        }
    }
}

impl<'a> Iterator for DragonIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((s, level, pos, reverse_and_flip)) = self.stack.pop() {
            let len = s.len();
            if level == 0 {
                if pos < len {
                    let idx = if reverse_and_flip { len - 1 - pos } else { pos };
                    let c = s.as_bytes()[idx];
                    let ch = if reverse_and_flip {
                        if c == b'0' { '1' } else { '0' }
                    } else {
                        c as char
                    };
                    if pos + 1 < len {
                        self.stack.push((s, level, pos + 1, reverse_and_flip));
                    }
                    return Some(ch);
                }
            } else {
                // Push right, middle, left frames (in reverse order for stack)
                let sep = if reverse_and_flip { "1" } else { "0" };
                self.stack.push((s, level - 1, 0, true));
                self.stack.push((sep, 0, 0, false));
                self.stack.push((s, level - 1, 0, false));
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

fn needed_levels(s: &str, min_length: usize) -> usize {
    let mut level = 0;
    let mut size = s.len();
    while size < min_length {
        size = 2 * size + 1;
        level += 1;
    }
    level
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_iteration_example1() {
        let d1 = DragonIterator::new("1", 0);
        let r1 = d1.collect::<String>();
        assert_eq!(r1, "1".to_string());
    }

    #[test]
    fn test_iteration_example2() {
        let d2 = DragonIterator::new("0", 1);
        let r2 = d2.collect::<String>();
        assert_eq!(r2, "001".to_string());
    }

    #[test]
    fn test_generation() {
        let levels = needed_levels("10000", 20);
        let d = DragonIterator::new("10000", levels).take(20);
        let r = d.collect::<String>();
        assert_eq!(r, "10000011110010000111".to_string());
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(fill_and_checksum("10000", 20), "01100".to_string());
    }

    #[test]
    fn test_part1() {
        assert_eq!(fill_and_checksum(&INPUT, 272), "11100111011101111".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(fill_and_checksum(&INPUT, 35651584), "10001110010000110".to_string());
    }
}
