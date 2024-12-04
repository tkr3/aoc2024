pub fn part1(input: &str) -> usize {
    let mut xs: Vec<(usize, usize)> = Vec::new();
    let board: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .inspect(|(x, c)| {
                    if *c == 'X' {
                        xs.push((*x, y));
                    }
                })
                .map(|(_, c): (usize, char)| c)
                .collect()
        })
        .collect();

    xs.into_iter()
        .filter_map(|pos| check_position(&board, "XMAS", pos, None).ok())
        .sum()
}

fn check_position(
    board: &Vec<Vec<char>>,
    word: &str,
    check: (usize, usize),
    next_direction: Option<(isize, isize)>,
) -> Result<usize, ()> {
    let expected = word
        .chars()
        .next()
        .expect("Expected at least one character");
    if *try_get(board, check)? != expected {
        return Err(());
    }
    if word.len() <= 1 {
        return Ok(1);
    }
    if let Some(direction) = next_direction {
        check_position(
            board,
            &word[1..],
            get_next_pos(check, direction)?,
            next_direction,
        )
    } else {
        let directions = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        return Ok(directions
            .iter()
            .filter_map(|direction| {
                get_next_pos(check, *direction)
                    .and_then(|next| check_position(board, &word[1..], next, Some(*direction)))
                    .ok()
            })
            .sum());
    }
}

pub fn part2(input: &str) -> usize {
    let mut a_positions: Vec<(usize, usize)> = Vec::new();
    let board: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .inspect(|(x, c)| {
                    if *c == 'A' {
                        a_positions.push((*x, y));
                    }
                })
                .map(|(_, c): (usize, char)| c)
                .collect()
        })
        .collect();

    a_positions
        .into_iter()
        .filter_map(|pos| check_single_position(&board, pos).ok())
        .count()
}

fn check_single_position(board: &[Vec<char>], check: (usize, usize)) -> Result<(), ()> {
    for (a, b) in [((-1, -1), (1, 1)), ((-1, 1), (1, -1))] {
        let &x = try_get(board, get_next_pos(check, a)?)?;
        if x == 'M' && *try_get(board, get_next_pos(check, b)?)? != 'S'
            || x == 'S' && *try_get(board, get_next_pos(check, b)?)? != 'M'
            || x == 'A'
            || x == 'X'
        {
            return Err(());
        }
    }
    Ok(())
}

fn try_get<T>(board: &[Vec<T>], position: (usize, usize)) -> Result<&T, ()> {
    board
        .get(position.1)
        .and_then(|x| x.get(position.0))
        .ok_or(())
}

fn get_next_pos(current: (usize, usize), direction: (isize, isize)) -> Result<(usize, usize), ()> {
    let next = (
        current.0 as isize + direction.0,
        current.1 as isize + direction.1,
    );
    if next.0 < 0 || next.1 < 0 {
        return Err(());
    }
    Ok((next.0 as usize, next.1 as usize))
}

const INPUT: &str = include_str!("../../inputs/day_04.txt");

fn main() {
    let p1 = part1(INPUT);
    println!("Part 1: {p1}");

    let p2 = part2(INPUT);
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 18);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 9);
    }
}
