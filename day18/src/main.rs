use once_cell::sync::Lazy;

static FIRST_ROW: Lazy<String> =
    Lazy::new(|| common::read_file_as_string("data/day18.txt").unwrap()
);

fn main() {
    println!("Part 1: {}", part(&FIRST_ROW, 40));
    println!("Part 1: {}", part(&FIRST_ROW, 400000));
}

fn part(input: &str, num_steps: usize) -> usize {
    let mut embed = embed(input);
    let mut total_safe = count_safe(&embed);
    for _ in 1..num_steps {
        let mut new_embed = embed.clone();
        for i in 1..embed.len() - 1 {
            let left = embed[i - 1];
            let center = embed[i];
            let right = embed[i + 1];
            if (left == b'^' && center == b'^' && right != b'^') ||
                (left != b'^' && center == b'^' && right == b'^') ||
                (left == b'^' && center != b'^' && right != b'^') ||
                (left != b'^' && center != b'^' && right == b'^') {
                new_embed[i] = b'^';
            } else {
                new_embed[i] = b'.';
            }
        }
        total_safe += count_safe(&new_embed);
        embed = new_embed;
    }
    total_safe
}

fn embed(input: &str) -> Vec<u8> {
    let mut row = Vec::with_capacity(input.len() + 2);
    row.push(b'.');
    row.extend(input.as_bytes());
    row.push(b'.');
    row
}

fn count_safe(embed: &[u8]) -> usize {
    embed.iter().filter(|&&c| c == b'.').count() - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!(part("..^^.", 3), 6);
        assert_eq!(part(".^^.^.^^^^", 10), 38);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part(&FIRST_ROW, 40), 1963);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part(&FIRST_ROW, 400000), 20009568);
    }
}