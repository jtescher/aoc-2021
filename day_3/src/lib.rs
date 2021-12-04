pub fn part_one(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let num_bits = lines.first().map(|row| row.len()).unwrap_or(0);

    let bit_counts = lines.iter().fold(vec![0; num_bits], |mut acc, num| {
        for (i, c) in num.chars().enumerate() {
            if c == '1' {
                acc[i] += 1;
            }
        }
        acc
    });

    let gamma = bit_counts
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, count)| ((count > lines.len() / 2) as u32) << i)
        .sum::<u32>();

    let epsilon = !gamma & ((1 << num_bits) - 1);

    gamma * epsilon
}

pub fn part_two(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();

    fn find_by(mut rows: Vec<&str>, col_idx: usize, retain_char: fn(bool) -> char) -> Option<u32> {
        if rows.len() <= 1 {
            return rows.pop().and_then(|num| u32::from_str_radix(num, 2).ok());
        }

        let one_most_frequent = rows
            .iter()
            .filter(|row| row.chars().nth(col_idx) == Some('1'))
            .count()
            >= (rows.len() + 1) / 2;
        let col_target = retain_char(one_most_frequent);
        rows.retain(|num| num.chars().nth(col_idx) == Some(col_target));

        find_by(rows, col_idx + 1, retain_char)
    }

    let o2 = find_by(lines.clone(), 0, |ones| if ones { '1' } else { '0' }).unwrap_or(0);
    let co2 = find_by(lines, 0, |ones| if ones { '0' } else { '1' }).unwrap_or(0);

    o2 * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part_one_works() {
        assert_eq!(198, part_one(TEST_INPUT));
        assert_eq!(3895776, part_one(include_str!("./input.txt")));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(230, part_two(TEST_INPUT));
        assert_eq!(7928162, part_two(include_str!("./input.txt")));
    }
}
