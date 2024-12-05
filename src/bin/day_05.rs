use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let (r, updates) = input.split_once("\n\n").unwrap();
    let rules: HashSet<(u32, u32)> = r
        .lines()
        .map(|line| {
            let mut numbers = line.split('|').filter_map(|n| n.parse().ok());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .collect();

    let mut mid_elements = Vec::new();
    'outer: for update in updates.lines() {
        let mut elements = Vec::new();
        for current in update.split(',').filter_map(|x| x.parse().ok()) {
            for previous in &elements {
                if rules.contains(&(current, *previous)) {
                    continue 'outer;
                }
            }
            elements.push(current);
        }
        mid_elements.push(elements[(elements.len() - 1) / 2])
    }

    mid_elements.into_iter().sum()
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

    let mut mid_elements = Vec::new();
    for update in updates
        .lines()
        .map(|update| update.split(',').filter_map(|x| x.parse().ok()))
    {
        let mut update: Vec<u32> = update.collect();
        let mut modified = false;
        'outer: loop {
            // Find next wrong element
            for (x, current) in update.iter().enumerate() {
                for (y, previous) in update[..x].iter().enumerate() {
                    if rules.contains(&(*current, *previous)) {
                        // Found wrong pair, swap them
                        update.swap(x, y);
                        modified = true;
                        continue 'outer;
                    }
                }
            }
            break;
        }
        // No more wrong elements
        if !modified {
            continue;
        }
        mid_elements.push(update[(update.len() - 1) / 2])
    }
    mid_elements.into_iter().sum()
}

const INPUT: &str = include_str!("../../inputs/day_05.txt");

fn main() {
    let p1 = part1(INPUT);
    println!("Part 1: {p1}");

    let p2 = part2(INPUT);
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "\
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
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 143);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 123);
    }
}
