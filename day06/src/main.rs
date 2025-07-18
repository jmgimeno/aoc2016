use once_cell::sync::Lazy;
use std::collections::HashMap;

static INPUT: Lazy<Vec<String>> =
    Lazy::new(|| common::read_file_as_lines("data/day06.txt").unwrap());

fn main() {
    println!("Part 1: {}", part1(&INPUT));
    println!("Part 2: {}", part2(&INPUT));
}

fn part1<T: AsRef<str>>(input: &[T]) -> String {
    count(input).maxs()
}

fn part2<T: AsRef<str>>(input: &[T]) -> String {
    count(input).mins()
}

fn count<T: AsRef<str>>(input: &[T]) -> ColumnCounter {
    let length = input[0].as_ref().len();
    let mut counter = ColumnCounter::new(length);
    for line in input {
        counter.add(line.as_ref());
    }
    counter
}

struct ColumnCounter(Vec<HashMap<char, usize>>);

impl ColumnCounter {
    fn new(length: usize) -> Self {
        Self(vec![HashMap::new(); length])
    }

    fn add(&mut self, word: &str) {
        for (i, c) in word.chars().enumerate() {
            *self.0[i].entry(c).or_insert(0) += 1;
        }
    }

    fn maxs(&self) -> String {
        self.select_by(|a, b| a.1.cmp(b.1))
    }

    fn mins(&self) -> String {
        self.select_by(|a, b| b.1.cmp(a.1))
    }

    fn select_by<F>(&self, cmp: F) -> String
    where
        F: Fn(&(&char, &usize), &(&char, &usize)) -> std::cmp::Ordering,
    {
        self.0
            .iter()
            .map(|map| {
                map.iter()
                    .max_by(|a, b| cmp(a, b))
                    .map(|(&ch, _)| ch)
                    .unwrap()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: [&'static str; 16] = [
        "eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv", "nssdts",
        "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar",
    ];

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(&EXAMPLE_INPUT), "easter");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), "qrqlznrl");
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(&EXAMPLE_INPUT), "advent");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), "kgzdfaon");
    }
}
