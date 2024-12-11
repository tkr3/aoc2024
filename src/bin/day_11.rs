use std::collections::HashMap;

type Stone = u64;

pub fn part1(input: &str) -> usize {
    input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .map(|stone| blink(stone, 25))
        .sum()
}

fn blink(stone: Stone, count: u32) -> usize {
    if count == 0 {
        return 1;
    }

    if stone == 0 {
        blink(1, count - 1)
    } else if let Ok((left, right)) = try_split_stone(stone) {
        blink(left, count - 1) + blink(right, count - 1)
    } else {
        blink(stone * 2024, count - 1)
    }
}

fn try_split_stone(stone: Stone) -> Result<(Stone, Stone), ()> {
    let mut left = stone;
    let mut higher = 1;
    while left >= higher {
        left /= 10;
        higher *= 10;
    }
    if left * 10 < higher {
        return Err(());
    }
    Ok((left, stone % higher))
}

pub fn part2(input: &str) -> usize {
    let mut cached: HashMap<(Stone, u32), usize> = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .map(|stone| blink_cached(stone, 75, &mut cached))
        .sum()
}

fn blink_cached(stone: Stone, count: u32, cache: &mut HashMap<(Stone, u32), usize>) -> usize {
    if count == 0 {
        return 1;
    }

    if let Some(result) = cache.get(&(stone, count)) {
        return *result;
    }

    let result = if stone == 0 {
        blink_cached(1, count - 1, cache)
    } else if let Ok((left, right)) = try_split_stone(stone) {
        blink_cached(left, count - 1, cache) + blink_cached(right, count - 1, cache)
    } else {
        blink_cached(stone * 2024, count - 1, cache)
    };
    cache.insert((stone, count), result);
    result
}

aoc2024::main!("../../inputs/day_11.txt");

aoc2024::test!(
    "\
125 17
",
    55312,
    65601038650482
);

#[cfg(test)]
mod test {
    use super::try_split_stone;

    #[test]
    fn test_try_split() {
        assert_eq!(try_split_stone(1), Err(()));
        assert_eq!(try_split_stone(100), Err(()));
        assert_eq!(try_split_stone(999), Err(()));
        assert_eq!(try_split_stone(10000), Err(()));

        assert_eq!(try_split_stone(12), Ok((1, 2)));
        assert_eq!(try_split_stone(1234), Ok((12, 34)));
        assert_eq!(try_split_stone(1200), Ok((12, 0)));
        assert_eq!(try_split_stone(1000), Ok((10, 0)));
    }
}
