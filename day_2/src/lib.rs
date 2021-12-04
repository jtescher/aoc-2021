pub fn part_one(input: &str) -> anyhow::Result<i32> {
    let (horiz, depth) = input.lines().try_fold((0, 0), |(horiz, depth), line| {
        if let Some(forward) = line.strip_prefix("forward ") {
            Ok((horiz + forward.parse::<i32>()?, depth))
        } else if let Some(up) = line.strip_prefix("up ") {
            Ok((horiz, depth - up.parse::<i32>()?))
        } else if let Some(down) = line.strip_prefix("down ") {
            Ok((horiz, depth + down.parse::<i32>()?))
        } else {
            anyhow::bail!("expected a direction, got {}", line)
        }
    })?;

    Ok(horiz * depth)
}

pub fn part_two(input: &str) -> anyhow::Result<i32> {
    let (horiz, depth, _aim) = input
        .lines()
        .try_fold((0, 0, 0), |(horiz, depth, aim), line| {
            if let Some(forward) = line.strip_prefix("forward ") {
                let x = forward.parse::<i32>()?;
                Ok((horiz + x, depth + (x * aim), aim))
            } else if let Some(up) = line.strip_prefix("up ") {
                Ok((horiz, depth, aim - up.parse::<i32>()?))
            } else if let Some(down) = line.strip_prefix("down ") {
                Ok((horiz, depth, aim + down.parse::<i32>()?))
            } else {
                anyhow::bail!("expected a direction, got {}", line)
            }
        })?;

    Ok(horiz * depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part_one_works() {
        assert_eq!(150, part_one(TEST_INPUT).unwrap());
        assert_eq!(1727835, part_one(include_str!("./part_one.txt")).unwrap());
    }

    #[test]
    fn part_two_works() {
        assert_eq!(900, part_two(TEST_INPUT).unwrap());
        assert_eq!(
            1544000595,
            part_two(include_str!("./part_one.txt")).unwrap()
        );
    }
}
