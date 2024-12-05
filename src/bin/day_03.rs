use std::{fmt::Debug, iter::Peekable};

pub fn part1(input: &str) -> i32 {
    let mut chars = input.chars().peekable();
    let mut sum = 0;

    while let Some(char) = chars.peek() {
        if *char == 'm' {
            let parsed = parse_mul(&mut chars);
            if let Ok(result) = parsed {
                sum += result;
            } else {
                // eprintln!("{}", parsed.unwrap_err());
            }
            continue;
        }
        chars.next();
    }
    sum
}

pub fn part2(input: &str) -> i32 {
    let mut chars = input.chars().peekable();
    let mut sum = 0;
    let mut enabled = true;

    while let Some(char) = chars.peek() {
        match *char {
            'm' => {
                let parsed = parse_mul(&mut chars);
                if let Ok(result) = parsed {
                    if enabled {
                        sum += result;
                    }
                } else {
                    // eprintln!("{}", parsed.unwrap_err());
                }
            }
            'd' => {
                let parsed = parse_cond(&mut chars);
                if let Ok(result) = parsed {
                    enabled = result;
                } else {
                    // eprintln!("{}", parsed.unwrap_err());
                }
            }
            _ => {
                chars.next();
            }
        }
    }
    sum
}

fn parse_cond<I>(chars: &mut Peekable<I>) -> Result<bool, String>
where
    I: Iterator<Item = char> + Debug,
{
    let mut enable = true;
    parse_char(chars, 'd')?;
    parse_char(chars, 'o')?;

    if parse_char(chars, 'n').is_ok() {
        parse_char(chars, '\'')?;
        parse_char(chars, 't')?;
        enable = false;
    }

    parse_char(chars, '(')?;
    parse_char(chars, ')')?;

    Ok(enable)
}

fn parse_mul<I>(chars: &mut Peekable<I>) -> Result<i32, String>
where
    I: Iterator<Item = char> + Debug,
{
    parse_char(chars, 'm')?;
    parse_char(chars, 'u')?;
    parse_char(chars, 'l')?;
    parse_char(chars, '(')?;
    let a = parse_number(chars)?;
    parse_char(chars, ',')?;
    let b = parse_number(chars)?;
    parse_char(chars, ')')?;

    Ok(a * b)
}

fn parse_char<I>(chars: &mut Peekable<I>, expected: char) -> Result<(), String>
where
    I: Iterator<Item = char>,
{
    if chars.peek().is_some_and(|c| *c == expected) {
        chars.next();
        return Ok(());
    }
    Err(format!(
        "Invalid character: {:?}, expected: '{}'",
        chars.peek(),
        expected
    ))
}

fn parse_number<I>(chars: &mut Peekable<I>) -> Result<i32, String>
where
    I: Iterator<Item = char>,
{
    let mut number = String::new();
    let mut next = chars.peek();
    while next.is_some_and(|c| c.is_numeric()) {
        number.push(*next.unwrap());
        chars.next();
        next = chars.peek();
    }
    number
        .parse()
        .map_err(|_| format!("Invalid number: {:?}", number))
}

const INPUT: &str = include_str!("../../inputs/day_03.txt");

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
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 161);
    }

    const EXAMPLE_INPUT_2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 48);
    }
}
