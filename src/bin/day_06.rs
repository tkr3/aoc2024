use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    const directions: &[(isize, isize)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut start: Option<(isize, isize)> = None;
    let mut obstacles = HashSet::new();
    let mut grid = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, char)| ((x as isize, y as isize), char))
    });
    let mut size = (0, 0);
    while let Some((pos, char)) = grid.next() {
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
        let current_direction = directions[turns % directions.len()];
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
    0
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
    0
);
