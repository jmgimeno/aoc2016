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
    // The input has only 5 different enerators and microchips
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Group {
    microchips: Set,
    generators: Set,
}

impl Group {
    fn new() -> Self {
        Self {
            microchips: Set::new(),
            generators: Set::new(),
        }
    }

    fn new_one_microchip(m: usize) -> Self {
        Self {
            microchips: Set::singleton(m),
            generators: Set::new(),
        }
    }

    fn new_one_generator(g: usize) -> Self {
        Self {
            microchips: Set::new(),
            generators: Set::singleton(g),
        }
    }

    fn new_two_microchips(m1: usize, m2: usize) -> Self {
        Self {
            microchips: Set::singleton(m1) | Set::singleton(m2),
            generators: Set::new(),
        }
    }

    fn new_two_generators(g1: usize, g2: usize) -> Self {
        Self {
            microchips: Set::new(),
            generators: Set::singleton(g1) | Set::singleton(g2),
        }
    }

    fn new_both(mg: usize) -> Self {
        Self {
            microchips: Set::singleton(mg),
            generators: Set::singleton(mg),
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

#[derive(Debug, Clone, Copy)]
struct Movemement {
    direction: Direction,
    group: Group,
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

    fn is_valid(&self) -> bool {
        self.floors.iter().all(|floor| floor.is_valid())
    }

    fn valid_directions(&self) -> Vec<Direction> {
        let mut directions = Vec::new();
        if self.elevator > 0 {
            directions.push(Direction::Down);
        }
        if self.elevator < 3 {
            directions.push(Direction::Up);
        }
        directions
    }

    fn valid_groups(&self) -> Vec<Group> {
        let floor = self.floors[self.elevator];
        let mut new_groups = Vec::new();
        // A single microchip is valid
        new_groups.extend(floor.microchips.iter().map(|m| Group::new_one_microchip(m)));
        // A single generator is valid
        new_groups.extend(floor.generators.iter().map(|g| Group::new_one_generator(g)));
        // A pair of microchips are valid as well
        new_groups.extend(
            floor
                .microchips
                .iter()
                .tuple_combinations()
                .map(|c: (usize, usize)| Group::new_two_microchips(c.0, c.1)),
        );
        // A pair of generators are also valid
        new_groups.extend(
            floor
                .generators
                .iter()
                .tuple_combinations()
                .map(|c: (usize, usize)| Group::new_two_generators(c.0, c.1)),
        );
        // A microchip only can travel with its generator
        new_groups.extend(
            (floor.microchips & floor.generators)
                .iter()
                .map(|mg| Group::new_both(mg)),
        );
        new_groups
    }

    fn possible_moves(&self) -> Vec<Movemement> {
        let mut moves = Vec::new();
        for direction in self.valid_directions() {
            for group in self.valid_groups() {
                moves.push(Movemement { direction, group });
            }
        }
        moves
    }

    fn next(&self, movement: Movemement) -> Self {
        let mut new_floors = self.floors.clone();
        new_floors[self.elevator] = new_floors[self.elevator].remove(movement.group);
        match movement.direction {
            Direction::Up => {
                new_floors[self.elevator + 1] = new_floors[self.elevator + 1].add(movement.group);
                Self {
                    elevator: self.elevator + 1,
                    floors: new_floors,
                }
            }
            Direction::Down => {
                new_floors[self.elevator - 1] = new_floors[self.elevator - 1].add(movement.group);
                Self {
                    elevator: self.elevator - 1,
                    floors: new_floors,
                }
            }
        }
    }

    fn expand(&self) -> Vec<Self> {
        let mut moves = Vec::new();
        for movement in self.possible_moves() {
            let next = self.next(movement);
            if next.is_valid() {
                moves.push(self.next(movement));
            }
        }
        moves
    }
}

fn parse_input<T: AsRef<str>>(input: &[T]) -> Configuration {
    let mut names = HashMap::new();
    let mut floors = [Group::new(); 4];
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
