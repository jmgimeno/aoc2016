use once_cell::sync::Lazy;

static NUM_ELVES: Lazy<usize> = Lazy::new(|| {
    common::read_file_as_string("data/day19.txt")
        .unwrap()
        .parse()
        .unwrap()
});

fn main() {
    println!("Part 1: {}", part1(&NUM_ELVES));
    println!("Part 2: {}", part2(&NUM_ELVES));
}

fn part1(num_elves: &usize) -> usize {
    let mut ring = ElfRing::new(*num_elves);
    let mut elf_idx = 0;
    while ring.size > 1 {
        let to_remove = ring.next_elf(elf_idx);
        ring.remove_elf(to_remove);
        elf_idx = ring.next_elf(elf_idx);
    }
    elf_idx + 1
}

fn part2(num_elves: &usize) -> usize {
    let mut ring = ElfRing::new(*num_elves);
    let mut elf_idx = 0;
    while ring.size > 1 {
        let mut to_remove = ring.next_elf(elf_idx);
        for _ in 1..ring.size / 2 {
            to_remove = ring.next_elf(to_remove);
        }
        ring.remove_elf(to_remove);
        elf_idx = ring.next_elf(elf_idx);
    }
    elf_idx + 1
}

struct Elf {
    next_idx: usize,
    previous_idx: usize,
}

struct ElfRing {
    size: usize,
    elves: Vec<Elf>,
}

impl ElfRing {
    fn new(num_elves: usize) -> Self {
        let mut elves = Vec::with_capacity(num_elves);
        for i in 0..num_elves {
            elves.push(Elf {
                next_idx: (i + 1) % num_elves,
                previous_idx: (i + num_elves - 1) % num_elves,
            })
        }
        Self {
            size: num_elves,
            elves
        }
    }

    fn next_elf(&self, idx: usize) -> usize {
        self.elves[idx].next_idx
    }

    fn remove_elf(&mut self, idx: usize) {
        let previous_idx = self.elves[idx].previous_idx;
        self.elves[previous_idx].next_idx = self.elves[idx].next_idx;
        let next_idx = self.elves[idx].next_idx;
        self.elves[next_idx].previous_idx = self.elves[idx].previous_idx;
        self.size -= 1;
    }
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

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(&5), 2);
    }
}
