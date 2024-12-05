use std::{cmp::Ordering, collections::HashSet};

pub fn part1(input: &str) -> u32 {
    let (r, updates) = input.split_once("\n\n").unwrap();
    let rules: HashSet<(u32, u32)> = r
        .lines()
        .map(|line| {
            let mut numbers = line.split('|').filter_map(|n| n.parse().ok());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .collect();

    updates
        .lines()
        .map(|update| update.split(',').filter_map(|x| x.parse().ok()))
        .filter_map(|update| {
            let update: Vec<u32> = update.collect();
            if update.is_sorted_by(|a, b| rules.contains(&(*a, *b))) {
                return Some(update[update.len() / 2]);
            }
            None
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (r, updates) = input.split_once("\n\n").unwrap();
    let rules: HashSet<(u32, u32)> = r
        .lines()
        .map(|line| {
            let mut numbers = line.split('|').filter_map(|n| n.parse().ok());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .collect();

    updates
        .lines()
        .map(|update| update.split(',').filter_map(|x| x.parse().ok()))
        .filter_map(|update| {
            let mut update: Vec<u32> = update.collect();
            if update.is_sorted_by(|a, b| rules.contains(&(*a, *b))) {
                return None;
            }
            update.sort_unstable_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    return Ordering::Less;
                } else if rules.contains(&(*b, *a)) {
                    return Ordering::Greater;
                }
                Ordering::Equal
            });
            Some(update[(update.len() - 1) / 2])
        })
        .sum()
}

aoc2024::main!("../../inputs/day_05.txt");

aoc2024::test!(
    "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
    143,
    123
);
