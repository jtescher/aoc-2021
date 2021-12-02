pub fn part_one(input: &str) -> anyhow::Result<usize> {
    let measurements = input
        .split_terminator('\n')
        .map(|line| Ok(line.parse()?))
        .collect::<anyhow::Result<Vec<usize>>>()?;

    let count = measurements.windows(2).filter(|x| x[1] > x[0]).count();

    Ok(count)
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    let measurements = input
        .split_terminator('\n')
        .map(|line| Ok(line.parse()?))
        .collect::<anyhow::Result<Vec<usize>>>()?;

    let count = measurements
        .windows(3)
        .fold((0, usize::MAX), |(acc, prev), x| {
            let sum = x.iter().sum();
            if sum > prev {
                (acc + 1, sum)
            } else {
                (acc, sum)
            }
        })
        .0;

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part_one_works() {
        assert_eq!(7, part_one(TEST_INPUT).unwrap());
        assert_eq!(1292, part_one(include_str!("./part_one.txt")).unwrap());
    }

    #[test]
    fn part_two_works() {
        assert_eq!(5, part_two(TEST_INPUT).unwrap());
        assert_eq!(1262, part_two(include_str!("./part_two.txt")).unwrap());
    }
}
