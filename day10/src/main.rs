use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

static PARSED_INPUT: Lazy<(Vec<Bot>, Vec<Move>)> = Lazy::new(|| {
    let input = common::read_file_as_lines("data/day10.txt").unwrap();
    parse_input(&input)
});

fn main() {
    let mut factory = Factory::new(&PARSED_INPUT.0);
    let part1 = part1(&mut factory, &PARSED_INPUT.1, 17, 61);
    println!("Part 1: {:?}", part1.unwrap());
    println!("Part 2; {:?}", part2(&factory));
}

fn part1(factory: &mut Factory, moves: &[Move], min_target: u32, max_target: u32) -> Option<u32> {
    let mut queue = VecDeque::from(moves.to_vec());
    let mut comparer_bot = None;
    while !queue.is_empty() {
        let move_ = queue.pop_front().unwrap();
        let next = factory.step(&move_);
        if next.len() == 2 && next[0].value == min_target && next[1].value == max_target {
            comparer_bot = Some(move_.bot);
        }
        queue.extend(next.into_iter());
    }
    comparer_bot
}

fn part2(factory: &Factory) -> u32 {
    factory.output[&0] * factory.output[&1] * factory.output[&2]
}

struct Factory {
    bots: HashMap<u32, Bot>,
    holding: HashMap<u32, Option<u32>>,
    output: HashMap<u32, u32>,
}

impl Factory {
    fn new(bots: &[Bot]) -> Self {
        Self {
            bots: bots.iter().map(|b| (b.id, b.clone())).collect(),
            holding: bots.iter().map(|b| (b.id, None)).collect(),
            output: HashMap::new(),
        }
    }

    fn step(&mut self, move_: &Move) -> Vec<Move> {
        let Move { value, bot: bot_id } = move_;
        let held = &self.holding[bot_id];
        match held {
            None => {
                self.holding.insert(*bot_id, Some(*value));
                Vec::new()
            }
            Some(other) => {
                let min = min(*value, *other);
                let max = max(*value, *other);
                self.holding.insert(*bot_id, None);
                let Bot { id: _, low, high } = self.bots[bot_id];
                let mut moves = Vec::new();
                match low {
                    Receiver::Bot(output_id) => {
                        moves.push(Move {
                            value: min,
                            bot: output_id,
                        });
                    }
                    Receiver::Output(output_id) => {
                        self.output.insert(output_id, min);
                    }
                }
                match high {
                    Receiver::Bot(output_id) => {
                        moves.push(Move {
                            value: max,
                            bot: output_id,
                        });
                    }
                    Receiver::Output(output_id) => {
                        self.output.insert(output_id, max);
                    }
                }
                moves
            }
        }
    }
}

fn parse_input<T: AsRef<str>>(lines: &[T]) -> (Vec<Bot>, Vec<Move>) {
    let mut bots = Vec::new();
    let mut moves = Vec::new();
    for line in lines {
        if let Ok(bot) = Bot::from_str(line.as_ref()) {
            bots.push(bot);
        } else if let Ok(move_) = Move::from_str(line.as_ref()) {
            moves.push(move_);
        }
    }
    (bots, moves)
}

#[derive(Debug, Clone)]
struct Move {
    value: u32,
    bot: u32,
}

static MOVE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap());

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cap) = MOVE_REGEX.captures(s) {
            Ok(Self {
                value: cap[1].parse().unwrap(),
                bot: cap[2].parse().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Receiver {
    Bot(u32),
    Output(u32),
}

static RECEIVER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(bot|output) (\d+)$").unwrap());

impl FromStr for Receiver {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cap) = RECEIVER_REGEX.captures(s) {
            match &cap[1] {
                "bot" => Ok(Receiver::Bot(cap[2].parse().unwrap())),
                "output" => Ok(Receiver::Output(cap[2].parse().unwrap())),
                _ => unreachable!(),
            }
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Bot {
    id: u32,
    low: Receiver,
    high: Receiver,
}

static BOT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^bot (\d+) gives low to (\w+ \d+) and high to (\w+ \d+)$").unwrap());

impl FromStr for Bot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cap) = BOT_REGEX.captures(s) {
            Ok(Self {
                id: cap[1].parse().unwrap(),
                low: cap[2].parse().unwrap(),
                high: cap[3].parse().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = vec![
            "value 5 goes to bot 2",
            "bot 2 gives low to bot 1 and high to bot 0",
            "value 3 goes to bot 1",
            "bot 1 gives low to output 1 and high to bot 0",
            "bot 0 gives low to output 2 and high to output 0",
            "value 2 goes to bot 2",
        ];
        let (bots, moves) = parse_input(&input);
        let mut factory = Factory::new(&bots);
        let part1 = part1(&mut factory, &moves, 2, 5);
        assert_eq!(factory.output[&0], 5);
        assert_eq!(factory.output[&1], 2);
        assert_eq!(factory.output[&2], 3);
        assert_eq!(part1, Some(2));
    }

    #[test]
    fn test_parts() {
        let mut factory = Factory::new(&PARSED_INPUT.0);
        let part1 = part1(&mut factory, &PARSED_INPUT.1, 17, 61);
        assert_eq!(part1, Some(98));
        assert_eq!(part2(&factory), 4042);
    }
}
