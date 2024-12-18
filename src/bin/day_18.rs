use aoc2024::util::{symmetric_grid, I2d, U2d};
use itertools::Itertools;
use pathfinding::matrix::directions::DIRECTIONS_4;

pub fn part1<const SIZE: usize>(
    input: &str,
    mut grid: [[bool; SIZE]; SIZE],
    max_bytes: usize,
) -> u32 {
    for (x, y) in input
        .lines()
        .map(|l| {
            l.splitn(2, ',')
                .map(|x| x.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .take(max_bytes)
    {
        grid[y][x] = true;
    }

    #[cfg(debug_assertions)]
    for y in grid {
        for x in y {
            if x {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let goal = U2d(SIZE - 1, SIZE - 1);
    let (_path, cost) = pathfinding::prelude::dijkstra(
        &U2d(0, 0),
        |pos| {
            let pos = *pos;
            DIRECTIONS_4
                .iter()
                .map(|&x| x.into())
                .filter_map(move |d: I2d| {
                    let n = (pos + d).ok()?;
                    grid.get(n.1)
                        .and_then(|g| g.get(n.0))
                        .and_then(|b| (!b).then_some((n, 1)))
                })
        },
        |&node| node == goal,
    )
    .expect("No path found");

    cost
}

pub fn part2<const SIZE: usize>(
    input: &str,
    mut grid: [[bool; SIZE]; SIZE],
    max_bytes: usize,
) -> U2d {
    let mut bytes = input.lines().map(|l| {
        l.splitn(2, ',')
            .map(|x| x.parse::<usize>().unwrap())
            .next_tuple()
            .unwrap()
    });

    for (x, y) in (0..max_bytes).map(|_| bytes.next().unwrap()) {
        grid[y][x] = true;
    }

    let goal = U2d(SIZE - 1, SIZE - 1);

    let mut last_byte = (0, 0);

    while let Some((path, _cost)) = pathfinding::prelude::dijkstra(
        &U2d(0, 0),
        |pos| {
            let pos = *pos;
            DIRECTIONS_4
                .iter()
                .map(|&x| x.into())
                .filter_map(move |d: I2d| {
                    let n = (pos + d).ok()?;
                    grid.get(n.1)
                        .and_then(|g| g.get(n.0))
                        .and_then(|b| (!b).then_some((n, 1)))
                })
        },
        |&node| node == goal,
    ) {
        loop {
            if let Some(byte) = bytes.next() {
                if path.contains(&byte.into()) {
                    last_byte = byte;
                    break;
                }
                grid[byte.1][byte.0] = true;
            } else {
                #[cfg(debug_assertions)]
                for (y, g) in grid.iter().enumerate() {
                    for (x, corrupt) in g.iter().enumerate() {
                        if *corrupt {
                            print!("#");
                        } else if (x, y) == last_byte {
                            print!("!");
                        } else if path.contains(&(x, y).into()) {
                            print!("O");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                return last_byte.into();
            }
        }
    }
    unreachable!();
}

aoc2024::main!("../../inputs/day_18.txt", symmetric_grid::<71>(), 1024);

aoc2024::test!(
    "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
",symmetric_grid::<7>(),12;
    22,
    U2d(6,1)
);
