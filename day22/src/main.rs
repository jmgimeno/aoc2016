use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

static NODES: Lazy<Vec<Node>> = Lazy::new(|| parse_file("data/day22.txt"));

fn main() {
    println!("{:?}", part1(&NODES));
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

fn parse_file(filename: &str) -> Vec<Node> {
    common::read_file_as_lines(filename)
        .unwrap()
        .iter()
        .skip(2)
        .map(|line| line.parse::<Node>().unwrap())
        .collect()
}

struct Grid {
    max_x: usize,
    max_y: usize,
    grid: Vec<GridNode>,
}

impl Grid {
    fn new(nodes: &[Node]) -> Self {
        let max_x = nodes.iter().map(|n| n.x).max().unwrap();
        let max_y = nodes.iter().map(|n| n.y).max().unwrap();
        let mut grid = vec![GridNode::default(); (max_x + 1) * (max_y + 1)];
        for node in nodes {
            let idx = node.y * (max_x + 1) + node.x;
            grid[idx].used = node.used as u8;
            grid[idx].avail = node.avail as u8;
        }
        Self { max_x, max_y, grid }
    }

    fn get(&self, x: usize, y: usize) -> &GridNode {
        &self.grid[y * (self.max_x + 1) + x]
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct GridNode {
    used: u8,
    avail: u8,
}

struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
}

static NODE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$").unwrap()
});

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = NODE_REGEX.captures(s) {
            let x = caps[1].parse::<usize>().unwrap();
            let y = caps[2].parse::<usize>().unwrap();
            let size = caps[3].parse::<usize>().unwrap();
            let used = caps[4].parse::<usize>().unwrap();
            let avail = caps[5].parse::<usize>().unwrap();
            let percent = caps[6].parse::<usize>().unwrap();
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
}
