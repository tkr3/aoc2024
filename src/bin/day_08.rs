use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let mut nodes: HashMap<u8, Vec<(usize, usize)>> = HashMap::with_capacity(20);

    let mut bottom_right = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.bytes().enumerate() {
            if char != b'.' {
                nodes.entry(char).or_default().push((x, y));
            }
            bottom_right = (x, y);
        }
    }

    let unique_antinodes =
        nodes
            .into_iter()
            .fold(HashSet::new(), |mut acc, (_frequency, antennas)| {
                for antinode in antennas
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index != antennas.len())
                    .flat_map(|(index, antenna)| {
                        antennas
                            .iter()
                            .skip(index + 1)
                            .flat_map(|other_antenna| {
                                [
                                    get_antinode(antenna, other_antenna),
                                    get_antinode(other_antenna, antenna),
                                ]
                            })
                            .filter_map(|x| x.ok())
                            .filter(|x| x.0 <= bottom_right.0 && x.1 <= bottom_right.1)
                    })
                {
                    acc.insert(antinode);
                }
                acc
            });

    #[cfg(debug_assertions)]
    {
        for y in 0..=bottom_right.1 {
            for x in 0..=bottom_right.0 {
                print!(
                    "{}",
                    if unique_antinodes.contains(&(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
    }

    unique_antinodes.len()
}

pub fn part2(input: &str) -> usize {
    let mut nodes: HashMap<u8, Vec<(usize, usize)>> = HashMap::with_capacity(20);

    let mut bottom_right = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.bytes().enumerate() {
            if char != b'.' {
                nodes.entry(char).or_default().push((x, y));
            }
            bottom_right = (x, y);
        }
    }

    let unique_antinodes =
        nodes
            .into_iter()
            .fold(HashSet::new(), |mut acc, (_frequency, antennas)| {
                for antinode in antennas
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index != antennas.len())
                    .flat_map(|(index, antenna)| {
                        antennas
                            .iter()
                            .skip(index + 1)
                            .flat_map(|other_antenna| {
                                [
                                    get_all_antinodes(antenna, other_antenna, &bottom_right),
                                    get_all_antinodes(other_antenna, antenna, &bottom_right),
                                ]
                                .into_iter()
                                .flatten_ok()
                            })
                            .filter_map(|x| x.ok())
                            .filter(|x| x.0 <= bottom_right.0 && x.1 <= bottom_right.1)
                    })
                {
                    acc.insert(antinode);
                }
                acc
            });

    #[cfg(debug_assertions)]
    {
        for y in 0..=bottom_right.1 {
            for x in 0..=bottom_right.0 {
                print!(
                    "{}",
                    if unique_antinodes.contains(&(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
    }

    unique_antinodes.len()
}

fn get_all_antinodes(
    ant1: &(usize, usize),
    ant2: &(usize, usize),
    max_position: &(usize, usize),
) -> Result<Vec<(usize, usize)>, ()> {
    debug_assert!(ant1 != ant2);

    let mut antinodes = Vec::new();

    let mut start = *ant1;
    let mut next = *ant2;
    antinodes.push(next);

    while let Ok(node) = get_antinode(&start, &next) {
        if node.0 > max_position.0 || node.1 > max_position.1 {
            break;
        }
        antinodes.push(node);
        start = next;
        next = node;
    }

    Ok(antinodes)
}

fn get_antinode(ant1: &(usize, usize), ant2: &(usize, usize)) -> Result<(usize, usize), ()> {
    debug_assert!(ant1 != ant2);
    Ok((
        ant2.0
            .checked_add_signed((ant2.0 as isize) - (ant1.0 as isize))
            .ok_or(())?,
        ant2.1
            .checked_add_signed((ant2.1 as isize) - (ant1.1 as isize))
            .ok_or(())?,
    ))
}

aoc2024::main!("../../inputs/day_08.txt");

aoc2024::test!(
    "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
    14,
    34
);
