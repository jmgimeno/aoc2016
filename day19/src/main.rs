use once_cell::sync::Lazy;

static NUM_ELVES: Lazy<usize> = Lazy::new(|| {
    common::read_file_as_string("data/day19.txt")
        .unwrap()
        .parse()
        .unwrap()
});

fn main() {
    println!("Part 1: {}", part1(*NUM_ELVES));
    println!("Part 2: {}", part2(*NUM_ELVES));
}

fn part1(num_elves: usize) -> usize {
    // Josephus' problem:
    // n = 2^k + m (k largest) => 2 * m + 1 survives
    // p = 2^k, m = n - p = n - 2^k
    // 2 * m + 1 = 2 * (n - 2^k) + 1 = 2*n - 2^k+1 + 1
    let mut p = 1;
    while p < num_elves {
        p *= 2;
    }
    2 * num_elves - p + 1
}

fn part2(num_elves: usize) -> usize {
    // Run the program that computes this with linked-lists and find a pattern.
    // See file part2.txt
    // Given n elves, let p be the largest integer power of 3 that does not exceed n .
    // If n = p then the winning elf is in position n.
    // Between the powers of 3, we see that for every elf added, it will remove an elf from the
    // starting position onward.
    // This holds up to 2p elves incrementing the winning position by 1.
    // If between 2p and the next integer power of 3, the position will jump by 2.
    // The easy way to compute it is from the next power of p
    let mut p = 1;
    while p * 3 < num_elves {
        p *= 3;
    }
    if num_elves == p {
        p
    } else if num_elves <= 2 * p {
        num_elves - p
    } else {
        // (3 * p) is the next power of 3
        // (3 * p - num_elves) is the (negative) distance to the next power of three
        // For each element of distance we have to subtract 2 from 3 * p
        // 3 * p - (3 * p - num_elves) * 2 = 2 * num_elves - 3 * p
        2 * num_elves - 3 * p
    }
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
