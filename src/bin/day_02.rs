pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut levels = line.split_ascii_whitespace().map(|l| l.parse().unwrap());
            let mut first: i32 = levels.next().unwrap();
            let mut increasing: Option<bool> = None;
            for second in levels {
                let change = second - first;
                if change == 0 || change.abs() > 3 {
                    return false;
                }

                match increasing {
                    Some(false) if change.is_positive() => {
                        return false;
                    }
                    Some(true) if change.is_negative() => {
                        return false;
                    }
                    None => {
                        increasing.replace(change.is_positive());
                    }
                    _ => {}
                };

                first = second;
            }
            true
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|l| l.parse().unwrap())
                .collect();

            for index in 0..levels.len() {
                let excluded = exclude_index(&levels, index);
                if check_levels(&excluded).is_none() {
                    return true;
                }
            }
            false
        })
        .count()
}

fn check_levels(levels: &[i32]) -> Option<usize> {
    let mut levels = levels.iter();
    let mut first: i32 = *levels.next().unwrap();
    let mut increasing: Option<bool> = None;
    for (index, second) in levels.enumerate() {
        let change = second - first;
        if change == 0 || change.abs() > 3 {
            return Some(index + 1);
        }

        match increasing {
            Some(false) if change.is_positive() => {
                return Some(index + 1);
            }
            Some(true) if change.is_negative() => {
                return Some(index + 1);
            }
            None => {
                increasing.replace(change.is_positive());
            }
            _ => {}
        };

        first = *second;
    }
    None
}

fn exclude_index(vec: &[i32], index: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(vec.len() - 1);
    result.extend_from_slice(&vec[..index]);
    result.extend_from_slice(&vec[index + 1..]);
    result
}

const INPUT: &str = include_str!("../../inputs/day_02.txt");

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
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 2);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 4);
    }
}
