use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
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
    let empty_node = grid.get_empty_node();
    let goal = (grid.max_x, 0);
    let initial_state = State { empty_node, goal };
    let mut queue = VecDeque::new();
    queue.push_back((0, initial_state.clone()));
    let mut visited = HashSet::new();
    visited.insert(initial_state);
    while let Some((depth, state)) = queue.pop_front() {
        for transfer in grid.transfers_to_empty(state.empty_node) {
            assert_eq!(transfer.to, state.empty_node, "Transfer to wrong node");
            let new_goal = if transfer.from == state.goal {
                transfer.to
            } else {
                state.goal
            };
            if new_goal == (0, 0) {
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
    empty_node: (usize, usize),
    goal: (usize, usize),
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

    fn get_empty_node(&self) -> (usize, usize) {
        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                let node = self.get(x, y);
                if node.used == 0 {
                    return (x, y);
                }
            }
        }
        unreachable!("No empty node found");
    }

    fn get(&self, x: usize, y: usize) -> &Node {
        &self.grid[y * (self.max_x + 1) + x]
    }

    fn transfers_to_empty(&self, empty_node: (usize, usize)) -> Vec<Transfer> {
        let mut transfers = Vec::new();
        let empty_size = self.get(empty_node.0, empty_node.1).size;
        if empty_node.0 > 0 && self.get(empty_node.0 - 1, empty_node.1).used <= empty_size {
            transfers.push(Transfer {
                from: (empty_node.0 - 1, empty_node.1),
                to: empty_node,
            });
        }
        if empty_node.1 > 0 && self.get(empty_node.0, empty_node.1 - 1).used <= empty_size {
            transfers.push(Transfer {
                from: (empty_node.0, empty_node.1 - 1),
                to: empty_node,
            });
        }
        if empty_node.0 < self.max_x && self.get(empty_node.0 + 1, empty_node.1).used <= empty_size
        {
            transfers.push(Transfer {
                from: (empty_node.0 + 1, empty_node.1),
                to: empty_node,
            });
        }
        if empty_node.1 < self.max_y && self.get(empty_node.0, empty_node.1 + 1).used <= empty_size
        {
            transfers.push(Transfer {
                from: (empty_node.0, empty_node.1 + 1),
                to: empty_node,
            });
        }
        transfers
    }
}

#[derive(Debug)]
struct Transfer {
    from: (usize, usize),
    to: (usize, usize),
}

fn parse_file(filename: &str) -> Vec<Node> {
    common::read_file_as_lines(filename)
        .unwrap()
        .iter()
        .skip(2)
        .map(|line| line.parse::<Node>().unwrap())
        .collect()
}

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
                let node = grid.get(x, y);
                if x > 0 && node.used > 0 && node.used <= grid.get(x - 1, y).avail {
                    transfers.push(Transfer {
                        from: (x, y),
                        to: (x - 1, y),
                    });
                }
                if y > 0 && node.used > 0 && node.used <= grid.get(x, y - 1).avail {
                    transfers.push(Transfer {
                        from: (x, y),
                        to: (x, y - 1),
                    });
                }
                if x < grid.max_x && node.used > 0 && node.used <= grid.get(x + 1, y).avail {
                    transfers.push(Transfer {
                        from: (x, y),
                        to: (x + 1, y),
                    });
                }
                if y < grid.max_y && node.used > 0 && node.used <= grid.get(x, y + 1).avail {
                    transfers.push(Transfer {
                        from: (x, y),
                        to: (x, y + 1),
                    });
                }
            }
        }
        transfers
    }
    
    #[test]
    fn all_transfers_are_to_the_empty_node() {
        let grid = Grid::new(&NODES);
        for transfer in all_transfers(&grid) {
            let used_at_destination = grid.get(transfer.to.0, transfer.to.1).used;
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
