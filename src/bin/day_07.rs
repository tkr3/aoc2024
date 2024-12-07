use core::panic;

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            let result: u64 = a.parse().unwrap();
            let operands: Vec<u64> = b
                .split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let ltr = LtR::new(&operands, 2, result);
            for res in ltr {
                if res == result {
                    return Some(result);
                }
            }
            None
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            let result: u64 = a.parse().unwrap();
            let operands: Vec<u64> = b
                .split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let ltr = LtR::new(&operands, 3, result);
            for res in ltr {
                if res == result {
                    return Some(result);
                }
            }
            None
        })
        .sum()
}

struct LtR<'a> {
    operands: &'a [u64],
    operator_count: usize,
    state: usize,
    max_state: usize,
    expected_result: u64,
}

impl<'a> LtR<'a> {
    fn new(operands: &'a [u64], operator_count: usize, expected_result: u64) -> Self {
        let max_state = operator_count.pow(operands.len() as u32 - 1);
        LtR {
            operands,
            operator_count,
            max_state,
            state: 0,
            expected_result,
        }
    }
}

impl Iterator for LtR<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state > self.max_state {
            return None;
        }
        let mut operands = self.operands.iter();
        let mut acc = *operands.next().unwrap();
        for (index, current) in operands.enumerate() {
            if self.expected_result < acc {
                return None;
            }
            let operation =
                self.state / (self.operator_count.pow(index as u32)) % self.operator_count;
            acc = match operation {
                2 => concat_numbers(acc, *current),
                1 => acc * current,
                0 => acc + current,
                _ => {
                    panic!("Invalid operation")
                }
            }
        }
        self.state += 1;
        Some(acc)
    }
}

fn concat_numbers(a: u64, b: u64) -> u64 {
    let mut b2 = b;
    let mut a = a;
    while b2 > 0 {
        b2 /= 10;
        a *= 10;
    }
    a + b
}

aoc2024::main!("../../inputs/day_07.txt");

aoc2024::test!(
    "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
",
    3749,
    11387
);
