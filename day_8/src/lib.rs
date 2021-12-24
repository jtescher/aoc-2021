pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            line.split_once('|').map(|(_, output)| {
                output
                    .split_whitespace()
                    .filter(|x| [2, 3, 4, 7].iter().any(|&len| len == x.len()))
                    .count()
            })
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            line.split_once('|').map(|(input, output)| {
                let mut inputs = input.split_whitespace();
                let one = inputs.clone().find(|i| i.len() == 2).unwrap_or_default();
                let four = inputs.find(|i| i.len() == 4).unwrap_or_default();

                output
                    .split_whitespace()
                    .map(|o| match o.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        7 => 8,
                        len => match (
                            len,
                            o.chars().filter(|&x| one.contains(x)).count(),
                            o.chars().filter(|&x| four.contains(x)).count(),
                        ) {
                            (5, 1, 3) => 5,
                            (5, 2, 3) => 3,
                            (5, _, 2) => 2,
                            (6, 1, _) => 6,
                            (6, _, 3) => 0,
                            (6, _, 4) => 9,
                            _ => 0,
                        },
                    })
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (i, n)| acc + n * 10_usize.pow(i as u32))
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part_one_works() {
        assert_eq!(26, part_one(TEST_INPUT));
        assert_eq!(409, part_one(include_str!("./input.txt")));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(61229, part_two(TEST_INPUT));
        assert_eq!(1024649, part_two(include_str!("./input.txt")));
    }
}
