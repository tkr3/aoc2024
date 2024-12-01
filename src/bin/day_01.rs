use std::collections::{BinaryHeap, HashMap};

pub fn part1(input: &str) -> u32 {
    let mut left: BinaryHeap<u32> = BinaryHeap::new();
    let mut right: BinaryHeap<u32> = BinaryHeap::new();

    input.lines().for_each(|l| {
        let mut line = l.split_ascii_whitespace();
        left.push(line.next().map(str::parse).unwrap().unwrap());
        right.push(line.next().map(str::parse).unwrap().unwrap());
    });

    left.into_sorted_vec()
        .into_iter()
        .zip(right.into_sorted_vec())
        .map(|(l, r)| (l.abs_diff(r)))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();

    input.lines().for_each(|l| {
        let mut line = l.split_ascii_whitespace();
        left.push(line.next().map(str::parse).unwrap().unwrap());
        let right_value = line.next().map(str::parse).unwrap().unwrap();
        right.insert(right_value, right.get(&right_value).unwrap_or(&0) + 1);
    });

    left.into_iter()
        .map(|l| l * right.get(&l).unwrap_or(&0))
        .sum()
}

const INPUT: &str = include_str!("../../inputs/day_01.txt");

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
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 11);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 31);
    }
}
