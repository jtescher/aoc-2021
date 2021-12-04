pub fn part_one(input: &str) -> anyhow::Result<usize> {
    let measurements = input
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect::<anyhow::Result<Vec<u32>>>()?;

    let count = measurements.windows(2).filter(|x| x[1] > x[0]).count();

    Ok(count)
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    let measurements = input
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect::<anyhow::Result<Vec<u32>>>()?;

    let count = measurements.windows(4).filter(|x| x[0] < x[3]).count();

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
        assert_eq!(1292, part_one(include_str!("./input_1.txt")).unwrap());
    }

    #[test]
    fn part_two_works() {
        assert_eq!(5, part_two(TEST_INPUT).unwrap());
        assert_eq!(1262, part_two(include_str!("./input_2.txt")).unwrap());
    }
}
