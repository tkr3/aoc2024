use std::collections::HashMap;

use anyhow::{anyhow, Context};

pub fn part1(input: &str) -> usize {
    let (a, b) = input.split_once("\n\n").unwrap();

    let mut available: HashMap<char, Vec<&str>> = HashMap::new();

    for towel in a.split(", ") {
        available
            .entry(towel.chars().next().unwrap())
            .or_default()
            .push(towel);
    }

    b.lines()
        .filter(|design| check_towel(design, &available).is_ok())
        .count()
}

pub fn part2(input: &str) -> u64 {
    let (a, b) = input.split_once("\n\n").unwrap();

    let mut available: HashMap<char, Vec<&str>> = HashMap::new();

    for towel in a.split(", ") {
        available
            .entry(towel.chars().next().unwrap())
            .or_default()
            .push(towel);
    }

    let mut map: HashMap<&str, u64> = HashMap::new();

    b.lines()
        .filter_map(|design| check_towel_sum(design, &available, &mut map).ok())
        .sum()
}

fn check_towel(substr: &str, towels: &HashMap<char, Vec<&str>>) -> anyhow::Result<()> {
    if substr.is_empty() {
        return Ok(());
    }
    towels
        .get(&substr.chars().next().unwrap())
        .ok_or(anyhow!("No possible towels found"))
        .and_then(|x| {
            x.iter()
                .filter_map(|towel| {
                    substr
                        .strip_prefix(towel)
                        .and_then(|s| check_towel(s, towels).ok())
                })
                .next()
                .with_context(|| "")
        })
}

fn check_towel_sum<'a>(
    substr: &'a str,
    towels: &HashMap<char, Vec<&str>>,
    map: &mut HashMap<&'a str, u64>,
) -> anyhow::Result<u64> {
    if let Some(i) = map.get(substr) {
        return Ok(*i);
    }
    if substr.is_empty() {
        return Ok(1);
    }
    towels
        .get(&substr.chars().next().unwrap())
        .ok_or(anyhow!("No possible towels found"))
        .map(|x| {
            let x = x
                .iter()
                .filter_map(|towel| {
                    substr
                        .strip_prefix(towel)
                        .and_then(|s| check_towel_sum(s, towels, map).ok())
                })
                .sum();
            map.insert(substr, x);
            x
        })
}

aoc2024::main!("../../inputs/day_19.txt");

aoc2024::test!(
    "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
",
    6,
    16
);
