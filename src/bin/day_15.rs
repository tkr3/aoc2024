use std::fmt::Display;

use aoc2024::util::{I2d, U2d};

pub fn part1(input: &str) -> usize {
    let (a, b) = input.split_once("\n\n").unwrap();
    let mut robot = Robot::default();
    let mut grid: Vec<Vec<Option<Block>>> = a
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, block)| {
                    block.try_into().map_or_else(
                        |_| {
                            match block {
                                b'.' => return None,
                                _ => {
                                    let position = U2d(x, y);
                                    robot = Robot(position);
                                }
                            }
                            None
                        },
                        Some,
                    )
                })
                .collect()
        })
        .collect();

    for direction in b
        .bytes()
        .filter(|b| !b.is_ascii_control())
        .map::<I2d, _>(|x| x.try_into().expect("Invalid direction"))
    {
        let target = (robot.0 + direction).expect("Robot should stay inside grid");

        match &grid[target.1][target.0] {
            Some(Block::Wall) => {
                // noop
            }
            Some(Block::Box) => {
                let mut push_target = target + direction;
                while let Ok(push_target_block) = push_target.map(|pos| &grid[pos.1][pos.0]) {
                    match push_target_block {
                        Some(Block::Wall) => break,
                        None => {
                            grid[push_target.unwrap().1][push_target.unwrap().0] = Some(Block::Box);
                            grid[target.1][target.0] = None;
                            robot.move_to(target);
                            break;
                        }
                        _ => {
                            push_target = push_target.unwrap() + direction;
                        }
                    }
                }
            }
            None => {
                robot.move_to(target);
            }
        }
    }

    #[cfg(debug_assertions)]
    for l in &grid {
        for c in l {
            if let Some(c) = c {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }

    grid.into_iter().enumerate().fold(0, |sum, (y, l)| {
        sum + l
            .into_iter()
            .enumerate()
            .filter_map(|(x, block)| matches!(block, Some(Block::Box)).then_some(y * 100 + x))
            .sum::<usize>()
    })
}

pub fn part2(input: &str) -> u32 {
    0
}

#[derive(Debug, Default)]
struct Robot(U2d);

impl Robot {
    fn move_to(&mut self, target: U2d) {
        self.0 = target;
    }
}

#[derive(Debug)]
enum Block {
    Wall,
    Box,
}

impl TryFrom<u8> for Block {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'#' => Ok(Self::Wall),
            b'O' => Ok(Self::Box),
            _ => Err(()),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Box => {
                write!(f, "O");
            }
            Block::Wall => {
                write!(f, "#");
            }
        }
        Ok(())
    }
}

aoc2024::main!("../../inputs/day_15.txt");

aoc2024::test!(
    "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
",
    10092,
    0
);
