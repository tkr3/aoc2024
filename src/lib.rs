pub mod util;

#[macro_export]
macro_rules! main {
    ($input:expr) => {
        static INPUT: &str = include_str!($input);

        fn main() {
            println!("Part 1: {}", part1(INPUT));

            println!("Part 2: {}", part2(INPUT));
        }
    };
    ($input:expr, $($param:expr),+) => {
        static INPUT: &str = include_str!($input);

        fn main() {
            println!("Part 1: {}", part1(INPUT, $($param),*));

            println!("Part 2: {}", part2(INPUT, $($param),*));
        }
    };
}

#[macro_export]
macro_rules! test {
    ($input:expr, $part1:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part1() {
                assert_eq!(part1($input), $part1);
            }

            #[test]
            fn test_part2() {
                assert_eq!(part2($input), $part2);
            }
        }
    };
    ($input1:expr, $part1:expr, $input2:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part1() {
                assert_eq!(part1($input1), $part1);
            }

            #[test]
            fn test_part2() {
                assert_eq!(part2($input2), $part2);
            }
        }
    };
    ($input:expr $(,$param:expr),*; $part1:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part1() {
                assert_eq!(part1($input $(,$param),*), $part1);
            }

            #[test]
            fn test_part2() {
                assert_eq!(part2($input $(,$param),*), $part2);
            }
        }
    };
}

#[macro_export]
macro_rules! benches {
    ($($day:ident),*) => {
        $(
            aoc2024::bench!($day);
        )*

        criterion_group!(benchmarks, $($day::bench,)*);
    };
}

#[macro_export]
macro_rules! bench {
    ($day:ident) => {
        mod $day {
            use criterion::Criterion;

            include!(concat!("../src/bin/", stringify!($day), ".rs"));

            pub fn bench(c: &mut Criterion) {
                let mut group = c.benchmark_group(stringify!($day));
                group.bench_function("1", |b| b.iter(|| part1(INPUT)));
                group.bench_function("2", |b| b.iter(|| part2(INPUT)));
                group.finish();
            }
        }
    };
}
