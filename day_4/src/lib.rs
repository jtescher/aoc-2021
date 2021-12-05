type Boards = Vec<Board>;
type Board = Vec<Vec<(u8, bool)>>;

pub fn part_one(input: &str) -> Option<u32> {
    let (mut feed, mut boards) = parse_sections(input)?;

    while let Some(next_number) = feed.next() {
        if let Some(winning_board) = mark_boards(&mut boards, next_number) {
            return Some(sum_unmarked(winning_board) * next_number as u32);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut feed, mut boards) = parse_sections(input)?;

    while let Some(next_number) = feed.next() {
        while let Some(latest_winner) = mark_boards(&mut boards, next_number) {
            if boards.is_empty() {
                return Some(sum_unmarked(latest_winner) * next_number as u32);
            }
        }
    }

    None
}

fn parse_sections(input: &str) -> Option<(impl Iterator<Item = u8> + '_, Boards)> {
    let mut parts = input.split_terminator("\n\n");
    let feed = parts
        .next()?
        .split_terminator(',')
        .filter_map(|num| num.parse().ok());

    let boards = parts
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_terminator(' ')
                        .filter_map(|num| num.parse().ok().map(|n| (n, false)))
                        .collect()
                })
                .collect()
        })
        .collect();

    Some((feed, boards))
}

fn mark_boards(boards: &mut Boards, next: u8) -> Option<Board> {
    let mut winner = None;

    for (b_idx, board) in boards.iter_mut().enumerate() {
        if let Some((r_idx, c_idx)) = board.iter().enumerate().find_map(|(r_idx, row)| {
            row.iter()
                .position(|x| x.0 == next)
                .map(|c_idx| (r_idx, c_idx))
        }) {
            board[r_idx][c_idx].1 = true;

            if board[r_idx].iter().all(|x| x.1) || board.iter().all(|row| row[c_idx].1) {
                winner = Some(b_idx)
            }
        }
    }

    winner.map(|idx| boards.swap_remove(idx))
}

fn sum_unmarked(board: Board) -> u32 {
    board
        .iter()
        .flat_map(|row| {
            row.iter()
                .filter_map(|x| if !x.1 { Some(x.0 as u32) } else { None })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part_one_works() {
        assert_eq!(4512, part_one(TEST_INPUT).unwrap());
        assert_eq!(35711, part_one(include_str!("./input.txt")).unwrap());
    }

    #[test]
    fn part_two_works() {
        assert_eq!(1924, part_two(TEST_INPUT).unwrap());
        assert_eq!(5586, part_two(include_str!("./input.txt")).unwrap());
    }
}
