use once_cell::sync::Lazy;

static NUM_ELVES: Lazy<usize> = Lazy::new(||
    common::read_file_as_string("data/day19.txt").unwrap().parse().unwrap()
);

fn main() {
    println!("Part 1: {}", part1(*NUM_ELVES));
}

fn part1(num_elves: usize) -> usize {
    josephus(num_elves, 2)
}

fn part2(num_elves: usize) -> usize {
    unimplemented!()
}

fn josephus(num_elves: usize, num_to_skip: usize) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(5), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(*NUM_ELVES), 1842613);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(5), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(*NUM_ELVES), 1424135);
    }
}
