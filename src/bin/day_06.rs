use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    const DIRECTIONS: &[(isize, isize)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut start: Option<(isize, isize)> = None;
    let mut obstacles = HashSet::new();
    let grid = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, char)| ((x as isize, y as isize), char))
    });
    let mut size = (0, 0);
    for (pos, char) in grid {
        match char {
            '#' => {
                obstacles.insert(pos);
            }
            '^' => {
                start = Some(pos);
            }
            _ => {}
        }
        size = pos;
    }

    let mut guard_position = start.expect("No start found");
    let mut turns = 0;
    let mut visited_positions: HashSet<(isize, isize)> =
        HashSet::with_capacity((size.0 * size.1) as usize);

    while guard_position.0 >= 0
        && guard_position.1 >= 0
        && guard_position.0 <= size.0
        && guard_position.1 <= size.1
    {
        let current_direction = DIRECTIONS[turns % DIRECTIONS.len()];
        let next_position = (
            guard_position.0 + current_direction.0,
            guard_position.1 + current_direction.1,
        );

        if obstacles.contains(&next_position) {
            turns += 1;
            continue;
        }

        visited_positions.insert(guard_position);
        guard_position = next_position;
    }

    visited_positions.len()
}

pub fn part2(input: &str) -> usize {
    const DIRECTIONS: &[(isize, isize)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut start: Option<(isize, isize)> = None;
    let mut obstacles = HashSet::new();
    let grid = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, char)| ((x as isize, y as isize), char))
    });
    let mut size = (0, 0);
    for (pos, char) in grid {
        match char {
            '#' => {
                obstacles.insert(pos);
            }
            '^' => {
                start = Some(pos);
            }
            _ => {}
        }
        size = pos;
    }

    let mut guard_position = start.expect("No start found");
    let mut turns = 0;
    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    while guard_position.0 >= 0
        && guard_position.1 >= 0
        && guard_position.0 <= size.0
        && guard_position.1 <= size.1
    {
        let current_direction = DIRECTIONS[turns % DIRECTIONS.len()];
        let next_position = (
            guard_position.0 + current_direction.0,
            guard_position.1 + current_direction.1,
        );

        if obstacles.contains(&next_position) {
            turns += 1;
            continue;
        }

        visited_positions.insert(guard_position);
        guard_position = next_position;
    }

    let mut stored_position = HashSet::new();

    visited_positions
        .into_iter()
        .filter(|turn_position| {
            stored_position.clear();
            guard_position = start.unwrap();
            turns = 0;
            while guard_position.0 >= 0
                && guard_position.1 >= 0
                && guard_position.0 <= size.0
                && guard_position.1 <= size.1
            {
                let dir_index = turns % DIRECTIONS.len();
                let current_direction = DIRECTIONS[dir_index];
                let next_position = (
                    guard_position.0 + current_direction.0,
                    guard_position.1 + current_direction.1,
                );
                if obstacles.contains(&next_position) || next_position == *turn_position {
                    turns += 1;
                    continue;
                }
                guard_position = next_position;

                if !stored_position.insert((guard_position, dir_index)) {
                    return true;
                }
            }
            false
        })
        .count()
}

aoc2024::main!("../../inputs/day_06.txt");

aoc2024::test!(
    "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
    41,
    6
);
