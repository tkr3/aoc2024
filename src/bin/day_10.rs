use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap_or(u32::MAX))
                .collect()
        })
        .collect();
    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                let set: HashSet<(usize, usize)> =
                    HashSet::from_iter(get_trailheads(&grid, (x, y), height).into_iter());
                sum += set.len();
            }
        }
    }
    sum
}

pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap_or(u32::MAX))
                .collect()
        })
        .collect();
    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                sum += get_trailheads(&grid, (x, y), &0).len();
            }
        }
    }
    sum
}

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (0, -1), (1, 0), (0, 1)];

fn get_trailheads(grid: &[Vec<u32>], start: (usize, usize), last: &u32) -> Vec<(usize, usize)> {
    if *last >= 9 {
        return vec![start];
    }
    DIRECTIONS
        .iter()
        .filter_map(|direction| {
            if let Ok((next, next_height)) = add_direction(&start, direction).and_then(|next| {
                grid.get(next.1)
                    .and_then(|x| x.get(next.0))
                    .map(|next_height| (next, next_height))
                    .ok_or(())
            }) {
                if *next_height == *last + 1 {
                    return Some(get_trailheads(grid, next, next_height));
                }
            }
            None
        })
        .flatten()
        .collect()
}

fn add_direction(point: &(usize, usize), direction: &(isize, isize)) -> Result<(usize, usize), ()> {
    Ok((
        point.0.checked_add_signed(direction.0).ok_or(())?,
        point.1.checked_add_signed(direction.1).ok_or(())?,
    ))
}

aoc2024::main!("../../inputs/day_10.txt");

aoc2024::test!(
    "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
    36,
    81
);
