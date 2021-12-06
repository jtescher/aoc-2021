pub fn part_one(input: &str) -> usize {
    count_fish(input, 80)
}

pub fn part_two(input: &str) -> usize {
    count_fish(input, 256)
}

fn count_fish(input: &str, days: usize) -> usize {
    let mut per_day_counts = input
        .split_terminator(',')
        .filter_map(|num| num.trim().parse().ok())
        .fold([0; 9], |mut acc, num: usize| {
            acc[num] += 1;
            acc
        });

    (1..days).for_each(|n| per_day_counts[(n + 7) % 9] += per_day_counts[n % 9]);

    per_day_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part_one_works() {
        assert_eq!(5934, part_one(TEST_INPUT));
        assert_eq!(395627, part_one(include_str!("./input.txt")));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(26984457539, part_two(TEST_INPUT));
        assert_eq!(1767323539209, part_two(include_str!("./input.txt")));
    }
}
