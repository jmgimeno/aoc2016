use once_cell::sync::Lazy;
use std::collections::{HashSet, VecDeque};

static KEY: Lazy<u32> = Lazy::new(|| {
    common::read_file_as_string("data/day13.txt")
        .unwrap()
        .parse::<u32>()
        .unwrap()
});

fn main() {
    println!("Part 1: {}", part1(*KEY, &Position::new(31, 39)));
    println!("Part 2: {}", part2(*KEY, 50));
}

fn part1(key: u32, target: &Position) -> usize {
    let map = Map::new(key);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((0, Position::new(1, 1)));
    visited.insert(Position::new(0, 0));
    while let Some((depth, current)) = queue.pop_front() {
        for next in map.expand(&current) {
            if &next == target {
                return depth + 1;
            }
            if !visited.contains(&next) {
                queue.push_back((depth + 1, next));
                visited.insert(next);
            }
        }
    }
    unreachable!("Solution not found");
}

fn part2(key: u32, max_steps: usize) -> usize {
    let map = Map::new(key);
    let start = Position::new(1, 1);
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut total_visited = visited.clone();
    find(&map, max_steps, start, &mut visited, &mut total_visited);
    total_visited.len()
}

fn find(
    map: &Map,
    max_steps: usize,
    current: Position,
    visited_in_path: &mut HashSet<Position>,
    total_visited: &mut HashSet<Position>,
) {
    if max_steps == 0 {
        return;
    }
    for next in map.expand(&current) {
        if !visited_in_path.contains(&next) {
            total_visited.insert(next);
            visited_in_path.insert(next);
            find(map, max_steps - 1, next, visited_in_path, total_visited);
            visited_in_path.remove(&next);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position(u32, u32);

impl Position {
    fn new(x: u32, y: u32) -> Self {
        Self(x, y)
    }

    fn neighbours(&self) -> Vec<Position> {
        let mut result = Vec::new();
        if self.0 > 0 {
            result.push(Position(self.0 - 1, self.1));
        }
        if self.1 > 0 {
            result.push(Position(self.0, self.1 - 1));
        }
        result.push(Position(self.0 + 1, self.1));
        result.push(Position(self.0, self.1 + 1));
        result
    }
}

struct Map {
    key: u32,
}

impl Map {
    fn new(key: u32) -> Self {
        Self { key }
    }

    fn is_open(&self, p: &Position) -> bool {
        let v = p.0 * p.0 + 3 * p.0 + 2 * p.0 * p.1 + p.1 + p.1 * p.1 + self.key;
        v.count_ones() % 2 == 0
    }

    fn expand(&self, pos: &Position) -> Vec<Position> {
        pos.neighbours()
            .into_iter()
            .filter(|p| self.is_open(p))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example_map() {
        let expected = vec![
            ".#.####.##",
            "..#..#...#",
            "#....##...",
            "###.#.###.",
            ".##..#..#.",
            "..##....#.",
            "#...##.###",
        ];
        let map = Map::new(10);
        for (y, line) in expected.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                assert_eq!(map.is_open(&Position::new(x as u32, y as u32)), c == '.');
            }
        }
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(10, &Position::new(7, 4)), 11);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(*KEY, &Position::new(31, 39)), 82);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(*KEY, 50), 138);
    }
}
