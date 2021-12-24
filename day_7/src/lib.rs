pub fn part_one(input: &str) -> i32 {
    let mut crabs = input
        .trim()
        .split_terminator(',')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<i32>>();

    let mid = crabs.len() / 2;
    let median = *crabs.select_nth_unstable(mid).1;

    crabs.iter().map(|i| (i - median).abs()).sum()
}

pub fn part_two(input: &str) -> i32 {
    let crabs = input
        .trim()
        .split_terminator(',')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<i32>>();

    let (min, max) = crabs.iter().fold((i32::MAX, i32::MIN), |(min, max), &x| {
        (min.min(x), max.max(x))
    });

    (min..=max)
        .map(|mid| {
            crabs
                .iter()
                .map(|i| {
                    let diff = (i - mid).abs();
                    diff * (diff + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part_one_works() {
        assert_eq!(37, part_one(TEST_INPUT));
        assert_eq!(336040, part_one(include_str!("./input.txt")));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(168, part_two(TEST_INPUT));
        assert_eq!(94813675, part_two(include_str!("./input.txt")));
    }
}
