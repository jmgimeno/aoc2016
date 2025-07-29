use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::ops::Index;
use std::str::FromStr;

static NODES: Lazy<Vec<Node>> = Lazy::new(|| parse_file("data/day22.txt"));

fn main() {
    println!("Part 1: {}", part1(&NODES));
    println!("Part 2: {}", part2(&NODES));
}

fn part1(grid: &[Node]) -> usize {
    let mut viable_pairs = 0;
    for (i, a) in grid.iter().enumerate() {
        if a.used == 0 {
            continue;
        }
        for (j, b) in grid.iter().enumerate() {
            if i == j {
                continue;
            }
            if a.used <= b.avail {
                viable_pairs += 1;
            }
        }
    }
    viable_pairs
}

fn part2(nodes: &[Node]) -> usize {
    let grid = Grid::new(nodes);
    let initial_state = State {
        empty_node: grid.get_empty_node(),
        goal: Position {
            x: grid.max_x,
            y: 0,
        },
    };
    let mut queue = VecDeque::new();
    queue.push_back((0, initial_state.clone()));
    let mut visited = HashSet::new();
    visited.insert(initial_state);
    while let Some((depth, state)) = queue.pop_front() {
        for transfer in grid.transfers_to_empty(state.empty_node) {
            let new_goal = if transfer.from == state.goal {
                transfer.to
            } else {
                state.goal
            };
            if new_goal.x == 0 && new_goal.y == 0 {
                return depth + 1;
            }
            let next_state = State {
                empty_node: transfer.from,
                goal: new_goal,
            };
            if !visited.contains(&next_state) {
                visited.insert(next_state.clone());
                queue.push_back((depth + 1, next_state));
            }
        }
    }
    unreachable!("No solution found")
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    empty_node: Position,
    goal: Position,
}

#[derive(Debug)]
struct Grid {
    max_x: usize,
    max_y: usize,
    grid: Vec<Node>,
}

impl Grid {
    fn new(nodes: &[Node]) -> Self {
        let max_x = nodes.iter().map(|n| n.x).max().unwrap();
        let max_y = nodes.iter().map(|n| n.y).max().unwrap();
        let mut grid = vec![Node::default(); (max_x + 1) * (max_y + 1)];
        for node in nodes {
            let idx = node.y * (max_x + 1) + node.x;
            grid[idx] = node.clone();
        }
        Self { max_x, max_y, grid }
    }
    
    fn get_empty_node(&self) -> Position {
        (0..=self.max_x)
            .flat_map(|x| (0..=self.max_y).map(move |y| Position { x, y }))
            .find(|&pos| self[pos].used == 0)
            .expect("No empty node found")
    }

    fn pos_to_index(&self, pos: Position) -> usize {
        pos.y * (self.max_x + 1) + pos.x
    }

    fn transfers_to_empty(&self, empty_node: Position) -> Vec<Transfer> {
        let empty_size = self[empty_node].size;
        DIRECTIONS
            .iter()
            .filter_map(|d| {
                let from = Position {
                    x: empty_node.x.wrapping_add(d.x),
                    y: empty_node.y.wrapping_add(d.y),
                };
                if from.x <= self.max_x && from.y <= self.max_y && self[from].used <= empty_size {
                    Some(Transfer {
                        from,
                        to: empty_node,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Index<Position> for Grid {
    type Output = Node;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.grid[self.pos_to_index(pos)]
    }
}

#[derive(Debug)]
struct Transfer {
    from: Position,
    to: Position,
}

fn parse_file(filename: &str) -> Vec<Node> {
    common::read_file_as_lines(filename)
        .unwrap()
        .iter()
        .skip(2)
        .map(|line| line.parse::<Node>().unwrap())
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

const DIRECTIONS: [Position; 4] = [
    Position { x: 0, y: 1 },
    Position { x: 1, y: 0 },
    Position {
        x: 0,
        y: usize::MAX,
    }, // -1 with wrapping
    Position {
        x: usize::MAX,
        y: 0,
    }, // -1 with wrapping
];

static NODE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$").unwrap()
});

#[derive(Clone, Default, Debug)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = NODE_REGEX.captures(s) {
            let x = caps[1].parse::<usize>().unwrap();
            let y = caps[2].parse::<usize>().unwrap();
            let size = caps[3].parse::<usize>().unwrap();
            let used = caps[4].parse::<usize>().unwrap();
            let avail = caps[5].parse::<usize>().unwrap();
            Ok(Node {
                x,
                y,
                size,
                used,
                avail,
            })
        } else {
            eprintln!("Failed to parse node: {}", s);
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&NODES), 1038);
    }

    fn all_transfers(grid: &Grid) -> Vec<Transfer> {
        let mut transfers = Vec::new();
        for x in 0..=grid.max_x {
            for y in 0..=grid.max_y {
                let pos = Position { x, y };
                let node = &grid[pos];
                if node.used == 0 {
                    continue;
                }
                for d in &DIRECTIONS {
                    let to = Position {
                        x: pos.x.wrapping_add(d.x),
                        y: pos.y.wrapping_add(d.y),
                    };
                    if to.x <= grid.max_x
                        && to.y <= grid.max_y
                        && grid[to].used == 0
                        && node.used <= grid[to].avail
                    {
                        transfers.push(Transfer { from: pos, to });
                    }
                }
            }
        }
        transfers
    }

    #[test]
    fn all_transfers_are_to_the_empty_node() {
        let grid = Grid::new(&NODES);
        for transfer in all_transfers(&grid) {
            let used_at_destination = grid[transfer.to].used;
            assert_eq!(used_at_destination, 0, "Transfer to a non-empty node");
        }
    }

    #[test]
    fn test_example_part2() {
        let lines = vec![
            "/dev/grid/node-x0-y0   10T    8T     2T   80%",
            "/dev/grid/node-x0-y1   11T    6T     5T   54%",
            "/dev/grid/node-x0-y2   32T   28T     4T   87%",
            "/dev/grid/node-x1-y0    9T    7T     2T   77%",
            "/dev/grid/node-x1-y1    8T    0T     8T    0%",
            "/dev/grid/node-x1-y2   11T    7T     4T   63%",
            "/dev/grid/node-x2-y0   10T    6T     4T   60%",
            "/dev/grid/node-x2-y1    9T    8T     1T   88%",
            "/dev/grid/node-x2-y2    9T    6T     3T   66%",
        ];
        let nodes: Vec<_> = lines.iter().map(|l| l.parse::<Node>().unwrap()).collect();
        assert_eq!(part2(&nodes), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&NODES), 252);
    }
}
