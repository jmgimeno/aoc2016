use md5::{Digest, Md5};
use once_cell::sync::Lazy;
use std::collections::VecDeque;

static PASSCODE: Lazy<String> =
    Lazy::new(|| common::read_file_as_string("data/day17.txt").unwrap());

const GRID_WIDTH: usize = 4;
const GRID_HEIGHT: usize = 4;

fn main() {
    println!("Part 1: {}", part1(&PASSCODE));
    println!("Part 2: {}", part2(&PASSCODE));
}

fn part1(passcode: &str) -> String {
    let mut queue = VecDeque::new();
    let initial = Step {
        path: passcode.as_bytes().to_vec(),
        x: 1,
        y: 1,
    };
    queue.push_back(initial);
    while let Some(step) = queue.pop_front() {
        if step.x == GRID_WIDTH && step.y == GRID_HEIGHT {
            return std::str::from_utf8(&step.path[passcode.len()..])
                .unwrap()
                .to_string();
        }
        queue.extend(step.expand());
    }
    unreachable!("No path found");
}

fn part2(passcode: &str) -> usize {
    let mut queue = Vec::new();
    let mut longest = 0;
    let initial = Step {
        path: passcode.as_bytes().to_vec(),
        x: 1,
        y: 1,
    };
    queue.push(initial);
    while let Some(step) = queue.pop() {
        if step.x == GRID_WIDTH && step.y == GRID_HEIGHT {
            let new_length = step.path.len() - passcode.len();
            if longest < new_length {
                longest = new_length;
            }
        } else {
            queue.extend(step.expand());
        }
    }
    longest
}

struct Step {
    path: Vec<u8>,
    x: usize,
    y: usize,
}

impl Step {
    fn expand(&self) -> Vec<Step> {
        const ALL_DIRECTIONS: [u8; 4] = [b'U', b'D', b'L', b'R'];
        let hash = Md5::digest(self.path.as_slice());
        let nibbles = [hash[0] >> 4, hash[0] & 0x0F, hash[1] >> 4, hash[1] & 0x0F];
        let mut steps = Vec::new();
        for (i, nibble) in nibbles.iter().enumerate() {
            if *nibble < 11 {
                continue;
            }
            match ALL_DIRECTIONS[i] {
                b'U' if self.y > 1 => {
                    let mut new_path = self.path.clone();
                    new_path.push(b'U');
                    steps.push(Step {
                        path: new_path,
                        x: self.x,
                        y: self.y - 1,
                    });
                }
                b'D' if self.y < GRID_HEIGHT => {
                    let mut new_path = self.path.clone();
                    new_path.push(b'D');
                    steps.push(Step {
                        path: new_path,
                        x: self.x,
                        y: self.y + 1,
                    });
                }
                b'L' if self.x > 1 => {
                    let mut new_path = self.path.clone();
                    new_path.push(b'L');
                    steps.push(Step {
                        path: new_path,
                        x: self.x - 1,
                        y: self.y,
                    });
                }
                b'R' if self.x < GRID_WIDTH => {
                    let mut new_path = self.path.clone();
                    new_path.push(b'R');
                    steps.push(Step {
                        path: new_path,
                        x: self.x + 1,
                        y: self.y,
                    });
                }
                _ => continue,
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!(part1("ihgpwlah"), "DDRRRD");
        assert_eq!(part1("kglvqrro"), "DDUDRLRRUDRD");
        assert_eq!(part1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&PASSCODE), "DDRRULRDRD");
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(part2("ihgpwlah"), 370);
        assert_eq!(part2("kglvqrro"), 492);
        assert_eq!(part2("ulqzkmiv"), 830);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&PASSCODE), 536);
    }
}
