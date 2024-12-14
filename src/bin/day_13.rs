use std::{i64, str::FromStr};

pub fn part1(input: &str) -> i64 {
    input
        .split("\n\n")
        .filter_map(|block| block.parse::<ClawMachine>().unwrap().play(None).ok())
        .map(|(a, b)| a * 3 + b)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .split("\n\n")
        .filter_map(|block| {
            block
                .parse::<ClawMachine>()
                .unwrap()
                .play(Some(10000000000000))
                .ok()
        })
        .map(|(a, b)| a * 3 + b)
        .sum()
}

trait Play {
    fn play(&self, base: Option<i64>) -> Result<(i64, i64), ()>;
}

#[derive(Debug)]
struct ClawMachine(i64, i64, i64, i64, i64, i64);

impl Play for ClawMachine {
    fn play(&self, base: Option<i64>) -> Result<(i64, i64), ()> {
        let x = self.4 + base.unwrap_or_default();
        let y = self.5 + base.unwrap_or_default();
        let det = (self.0 * self.3) - (self.1 * self.2);

        if det == 0 {
            return Err(());
        }

        let a = (x * self.3 - y * self.2) / det;
        let b = (self.0 * y - self.1 * x) / det;

        if self.0 * a + self.2 * b == x && self.1 * a + self.3 * b == y {
            return Ok((a, b));
        }
        Err(())
    }
}

impl FromStr for ClawMachine {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.lines().flat_map(|line| {
            line.split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|x| x[2..].parse().unwrap())
        });

        Ok(ClawMachine(
            nums.next().ok_or(())?,
            nums.next().ok_or(())?,
            nums.next().ok_or(())?,
            nums.next().ok_or(())?,
            nums.next().ok_or(())?,
            nums.next().ok_or(())?,
        ))
    }
}

aoc2024::main!("../../inputs/day_13.txt");

aoc2024::test!(
    "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
",
    480,
    875318608908
);
