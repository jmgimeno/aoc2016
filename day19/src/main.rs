use bitvec::prelude::*;
use once_cell::sync::Lazy;

static NUM_ELVES: Lazy<usize> = Lazy::new(|| {
    common::read_file_as_string("data/day19.txt")
        .unwrap()
        .parse()
        .unwrap()
});

fn main() {
   println!("Part 1: {}", part1(&NUM_ELVES));
}

fn part1(num_elves: &usize) -> usize {
    let mut elves = bitvec![u8, Lsb0; 1; *num_elves];
    let mut remaining = *num_elves;
    let mut idx = 0;
    while remaining > 1 {
        if elves[idx] {
            let mut next_idx = (idx + 1) % elves.len();
            while !elves[next_idx] {
                next_idx = (next_idx + 1) % elves.len();
            }
            elves.set(next_idx, false);
            remaining -= 1;
        }
        if remaining != 1 {
            idx = (idx + 1) % elves.len();
        }
    }
    idx + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(&5), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&NUM_ELVES), 1842613);
    }
}
