use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

pub fn part1(input: &str, size: (usize, usize)) -> u32 {
    let mut bucket_collection = [0, 0, 0, 0];
    for pos in input
        .lines()
        .map(|l| l.parse::<Robot>().unwrap().walk(100, size))
    {
        if pos.0 != size.0 / 2 && pos.1 != size.1 / 2 {
            let bucket = match (pos.0 < size.0 / 2, pos.1 < size.1 / 2) {
                (true, true) => 0,
                (true, false) => 1,
                (false, true) => 2,
                (false, false) => 3,
            };
            bucket_collection[bucket] += 1;
        }
    }
    bucket_collection.into_iter().product::<u32>()
}

pub fn part2(input: &str, size: (usize, usize)) -> usize {
    let result = (0..(size.0 * size.1))
        .min_by_key(|seconds| {
            let mut bucket_collection = [0, 0, 0, 0];
            let positions: HashSet<_> = input
                .lines()
                .map(|l| l.parse::<Robot>().unwrap().walk(*seconds as i32, size))
                .collect();
            for pos in positions.iter() {
                if pos.0 != size.0 / 2 && pos.1 != size.1 / 2 {
                    let bucket = match (pos.0 < size.0 / 2, pos.1 < size.1 / 2) {
                        (true, true) => 0,
                        (true, false) => 1,
                        (false, true) => 2,
                        (false, false) => 3,
                    };
                    bucket_collection[bucket] += 1;
                }
            }
            bucket_collection.into_iter().product::<u32>()
        })
        .expect("No min found");

    #[cfg(debug_assertions)]
    {
        let positions: HashSet<_> = input
            .lines()
            .map(|l| l.parse::<Robot>().unwrap().walk(result as i32, size))
            .collect();
        for y in 0..size.1 {
            for x in 0..size.0 {
                if positions.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
    result
}

#[derive(Debug)]
struct Robot {
    position: (usize, usize),
    velocity: (i32, i32),
}

impl Robot {
    fn walk(&self, seconds: i32, walls: (usize, usize)) -> (usize, usize) {
        (
            (self.position.0 as i32 + self.velocity.0 * seconds).rem_euclid(walls.0 as i32)
                as usize,
            (self.position.1 as i32 + self.velocity.1 * seconds).rem_euclid(walls.1 as i32)
                as usize,
        )
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(' ').ok_or(())?;
        let mut p = p[2..].split(',').filter_map(|s| s.parse().ok());
        let mut v = v[2..].split(',').filter_map(|s| s.parse().ok());
        Ok(Robot {
            position: p.next_tuple().ok_or(())?,
            velocity: v.next_tuple().ok_or(())?,
        })
    }
}

aoc2024::main!("../../inputs/day_14.txt", (101, 103));

aoc2024::test!(
    "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
", (11, 7);
    12,
    0
);
