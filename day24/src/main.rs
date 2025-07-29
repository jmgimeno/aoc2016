use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Index};

static INPUT: Lazy<Vec<String>> = Lazy::new(||
    common::read_file_as_lines("data/day24.txt").unwrap()
);

fn main() {
    println!("Part 1: {}", part1(&INPUT));
    println!("Part 2: {}", part2(&INPUT));
}

fn part1<T : AsRef<str>>(input: &[T]) -> usize {
    let grid = Grid::new(input);
    grid.min_distance_all(false)
}

fn part2(input: &[String]) -> usize {
    let grid = Grid::new(input);
    grid.min_distance_all(true)
}

struct Grid {
    grid: Vec<Vec<bool>>,
    locations: HashMap<usize, Position>,
}

impl Grid {
    fn new<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut grid = Vec::new();
        let mut locations = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.as_ref().chars().enumerate() {
                row.push(c != '#');
                if c.is_digit(10) {
                    locations.insert(c.to_digit(10).unwrap() as usize, Position { x, y });
                }
            }
            grid.push(row);
        }
        Self { grid, locations }
    }

    fn get_neighbors(&self, pos: &Position) -> Vec<Position> {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut neighbors = Vec::new();
        for (dx, dy) in directions.iter() {
            let nx = (pos.x as isize + dx) as usize;
            let ny = (pos.y as isize + dy) as usize;
            if self.grid[ny][nx] {
                neighbors.push(Position { x: nx, y: ny });
            }
        }
        neighbors
    }

    fn min_distance(&self, from: &Position, to: &Position) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((0, from.clone()));
        let mut visited = HashSet::new();
        visited.insert(from.clone());
        while let Some((depth, pos)) = queue.pop_front() {
            for neighbor in self.get_neighbors(&pos) {
                if neighbor == *to {
                    return depth + 1;
                } else if !visited.contains(&neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back((depth + 1, neighbor));
                }
            }
        }
        unreachable!("No path found");
    }

    fn min_distance_all(&self, go_back: bool) -> usize {
        let others = (1..self.locations.len()).collect::<Vec<_>>();
        let mut min_distance = usize::MAX;
        let mut cached_distances = CachedDistances::new();
        for permutation in others.iter().permutations(others.len()) {
            // eprintln!("{:?}", permutation);
            let mut start = &0;
            let mut permutation_distance = 0;
            for next in permutation {
                let leg = if cached_distances.has_key(&(*start, *next)) {
                    cached_distances[&(*start, *next)]
                }  else {
                    let distance = self.min_distance(&self.locations[&start], &self.locations[&next]);
                    cached_distances.insert(&(*start, *next),  distance);
                    distance
                };
                permutation_distance += leg;
                start = next;
            }
            if go_back {
                let return_path = if cached_distances.has_key(&(*start, 0)) {
                    cached_distances[&(*start, 0)]
                } else {
                    self.min_distance(&self.locations[&start], &self.locations[&0])
                };
                permutation_distance += return_path;
            }
            min_distance = min_distance.min(permutation_distance);
        }
        min_distance
    }
}

impl Index<&Position> for Grid {
    type Output = bool;
    fn index(&self, pos: &Position) -> &Self::Output {
        &self.grid[pos.y][pos.x]
    }
}

#[derive(Debug)]
struct CachedDistances {
    distances: HashMap<(usize, usize), usize>,
}

impl CachedDistances {
    fn new() -> Self {
        Self { distances: HashMap::new() }
    }

    fn has_key(&self, key: &(usize, usize)) -> bool {
        self.distances.contains_key(&Self::normalize(key))
    }

    fn insert(&mut self, key: &(usize, usize), value: usize) {
        self.distances.insert(Self::normalize(key), value);
    }

    fn normalize(key: &(usize, usize)) -> (usize, usize) {
        let pair = if key.0 > key.1 { (key.1, key.0) } else { *key };
        if pair.0 > pair.1 {
            (pair.1, pair.0)
        } else {
            pair
        }
    }
}

impl Index<&(usize, usize)> for CachedDistances {
    type Output = usize;

    fn index(&self, key: &(usize, usize)) -> &Self::Output {
        &self.distances[&Self::normalize(key)]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distances_from_0() {
        let input = vec![
            "###########",
            "#0.1.....2#",
            "#.#######.#",
            "#4.......3#",
            "###########",
        ];
        let grid = Grid::new(&input);
        let distances =
            (1..grid.locations.len())
                .map(|i| grid.min_distance(&grid.locations[&0], &grid.locations[&i]))
                .collect::<Vec<_>>();
        assert_eq!(distances, vec![2, 8, 10, 2]);
    }

    #[test]
    fn test_example_part1() {
        let input = vec![
            "###########",
            "#0.1.....2#",
            "#.#######.#",
            "#4.......3#",
            "###########",
        ];
        assert_eq!(part1(&input), 14);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 470);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 720);
    }
}
