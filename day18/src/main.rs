use once_cell::sync::Lazy;

static FIRST_ROW: Lazy<String> =
    Lazy::new(|| common::read_file_as_string("data/day18.txt").unwrap());

fn main() {
    println!("Part 1: {}", part(&FIRST_ROW, 40));
    println!("Part 1: {}", part(&FIRST_ROW, 400000));
}

fn part(input: &str, num_steps: usize) -> usize {
    let width = input.len();
    let mut row = input.as_bytes().to_vec();
    let mut total_safe = row.iter().filter(|&&c| c == b'.').count();

    let mut next_row = vec![b'.'; width];
    for _ in 1..num_steps {
        for i in 0..width {
            let left = if i == 0 { b'.' } else { row[i - 1] };
            let center = row[i];
            let right = if i == width - 1 { b'.' } else { row[i + 1] };
            next_row[i] = if (left == b'^' && center == b'^' && right != b'^')
                || (left != b'^' && center == b'^' && right == b'^')
                || (left == b'^' && center != b'^' && right != b'^')
                || (left != b'^' && center != b'^' && right == b'^')
            {
                b'^'
            } else {
                b'.'
            };
        }
        total_safe += next_row.iter().filter(|&&c| c == b'.').count();
        std::mem::swap(&mut row, &mut next_row);
    }
    total_safe
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
