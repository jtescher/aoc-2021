use std::collections::HashMap;
use std::iter::successors;

type Segment = [(isize, isize); 2];

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter_map(parse_segment)
        .filter(|[(ax, ay), (bx, by)]| ax == bx || ay == by)
        .fold(HashMap::new(), build_intersections)
        .values()
        .filter(|x| **x > 1)
        .count()
}

pub fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter_map(parse_segment)
        .fold(HashMap::new(), build_intersections)
        .values()
        .filter(|x| **x > 1)
        .count()
}

fn parse_segment(input_line: &str) -> Option<Segment> {
    let (a, b) = input_line.split_once(" -> ")?;
    let ((ax, ay), (bx, by)) = (a.split_once(',')?, b.split_once(',')?);

    Some([
        (ax.parse().ok()?, ay.parse().ok()?),
        (bx.parse().ok()?, by.parse().ok()?),
    ])
}

fn build_intersections(
    mut acc: HashMap<(isize, isize), usize>,
    [(ax, ay), (bx, by)]: Segment,
) -> HashMap<(isize, isize), usize> {
    let range = |a: isize, b: isize| successors(Some(a), move |n| Some(n + (b - a).signum()));

    range(ax, bx)
        .cycle()
        .zip(range(ay, by).cycle())
        .take((ax - bx).abs().max((ay - by).abs()) as usize + 1)
        .for_each(|(x, y)| *acc.entry((x, y)).or_insert(0) += 1);

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part_one_works() {
        assert_eq!(5, part_one(TEST_INPUT));
        assert_eq!(7414, part_one(include_str!("./input.txt")));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(12, part_two(TEST_INPUT));
        assert_eq!(19676, part_two(include_str!("./input.txt")));
    }
}
