use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet, VecDeque};
use tinybitset::TinyBitSet;

// In both the example and the input, 8 bits are enough
type Set = TinyBitSet<u8, 1>;

static PART1: Lazy<Configuration> = Lazy::new(|| {
    let input = common::read_file_as_lines("data/day11.txt").unwrap();
    parse_input(&input)
});

static PART2: Lazy<Configuration> = Lazy::new(|| {
    // The input has only 5 different generators and microchips
    let mut part2 = PART1.clone();
    part2.floors[0].generators.insert(6);
    part2.floors[0].generators.insert(7);
    part2.floors[0].microchips.insert(6);
    part2.floors[0].microchips.insert(7);
    part2
});

fn main() {
    println!("Part 1: {}", part(&PART1));
    println!("Part 2: {}", part(&PART2));
}

fn part(initial: &Configuration) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, *initial));
    visited.insert(*initial);
    while let Some((depth, configuration)) = queue.pop_front() {
        let expanded = configuration.expand();
        for configuration in expanded {
            if configuration.is_solution() {
                return depth + 1;
            } else if !visited.contains(&configuration) {
                queue.push_back((depth + 1, configuration));
                visited.insert(configuration);
            }
        }
    }
    unreachable!()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
struct Group {
    microchips: Set,
    generators: Set,
}

impl Group {
    fn new(microchips: Set, generators: Set) -> Self {
        Self {
            microchips,
            generators,
        }
    }

    fn is_empty(&self) -> bool {
        self.microchips.is_empty() && self.generators.is_empty()
    }

    fn is_valid(&self) -> bool {
        self.generators.is_empty() || self.microchips.iter().all(|i| self.generators[i])
    }

    fn add(self, other: Self) -> Self {
        Self {
            microchips: self.microchips | other.microchips,
            generators: self.generators | other.generators,
        }
    }

    fn remove(self, other: Self) -> Self {
        Self {
            microchips: self.microchips & !other.microchips,
            generators: self.generators & !other.generators,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Configuration {
    elevator: usize,
    floors: [Group; 4],
}

impl Configuration {
    fn is_solution(&self) -> bool {
        self.elevator == 3 && (0..3).all(|i| self.floors[i].is_empty())
    }

    fn valid_directions(&self) -> impl Iterator<Item = Direction> {
        [Some(Direction::Down), Some(Direction::Up)]
            .into_iter()
            .enumerate()
            .filter_map(move |(i, dir)| match i {
                0 if self.elevator > 0 => dir,
                1 if self.elevator < 3 => dir,
                _ => None,
            })
    }

    fn valid_groups(&self) -> impl Iterator<Item = Group> + '_ {
        let floor = self.floors[self.elevator];
        itertools::chain!(
            floor.microchips.iter().map(|m| Group::new(Set::singleton(m), Set::new())),
            floor.generators.iter().map(|g| Group::new(Set::new(), Set::singleton(g))),
            floor
                .microchips
                .iter()
                .tuple_combinations()
                .map(|(m1, m2)| Group::new(Set::singleton(m1) | Set::singleton(m2), Set::new())),
            floor
                .generators
                .iter()
                .tuple_combinations()
                .map(|(g1, g2)| Group::new(Set::new(), Set::singleton(g1) | Set::singleton(g2))),
            (floor.microchips & floor.generators)
                .iter()
                .map(|mg| Group::new(Set::singleton(mg), Set::singleton(mg)))
        )
    }

    fn expand(&self) -> Vec<Self> {
        let mut configurations = Vec::new();
        let current_floor = self.elevator;
        let current_group = &self.floors[current_floor];
        for group in self.valid_groups() {
            let new_current_group = current_group.remove(group);
            if !new_current_group.is_valid() { continue; }
            for direction in self.valid_directions() {
                let new_floor = match direction {
                    Direction::Up => current_floor + 1,
                    Direction::Down => current_floor - 1,
                };
                let new_floor_group = &self.floors[new_floor];
                let new_floor_grup = new_floor_group.add(group);
                if !new_floor_grup.is_valid() { continue; }
                let mut new_floors = self.floors;
                new_floors[current_floor] = new_current_group;
                new_floors[new_floor] = new_floor_grup;
                configurations.push(Self {
                    elevator: new_floor,
                    floors: new_floors,
                });
            }
        }
        configurations
    }
}

fn parse_input<T: AsRef<str>>(input: &[T]) -> Configuration {
    let mut names = HashMap::new();
    let mut floors = [Group::default(); 4];
    for (i, line) in input.iter().enumerate() {
        let microchip_regex = regex::Regex::new(r"(\w+)-compatible microchip").unwrap();
        let generator_regex = regex::Regex::new(r"(\w+) generator").unwrap();
        for microchip in microchip_regex.captures_iter(line.as_ref()) {
            let name = microchip[1].to_string();
            let num_names = names.len();
            let id = names.entry(name).or_insert_with(|| num_names);
            floors[i].microchips.insert(*id);
        }
        for generator in generator_regex.captures_iter(line.as_ref()) {
            let name = generator[1].to_string();
            let num_names = names.len();
            let id = names.entry(name).or_insert_with(|| num_names);
            floors[i].generators.insert(*id);
        }
    }
    Configuration {
        elevator: 0,
        floors,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = vec![
            "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.",
            "The second floor contains a hydrogen generator.",
            "The third floor contains a lithium generator.",
            "The fourth floor contains nothing relevant.",
        ];
        assert_eq!(part(&parse_input(&example)), 11);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part(&PART1), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part(&PART2), 57);
    }
}
