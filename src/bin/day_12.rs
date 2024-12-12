pub fn part1(input: &str) -> u32 {
    let mut plots: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let mut next_queue = vec![(0, 0)];
    let mut total_price = 0;
    while let Some(next) = next_queue.pop() {
        if let Some(&plot_id) = plots.get(next.1).and_then(|l| l.get(next.0)) {
            if plot_id >= b'a' {
                continue;
            }
            let mut plot_queue = vec![Ok(next)];
            let mut regions = 0;
            let mut fences = 0;
            while let Some(next_position) = plot_queue.pop() {
                if let Ok(next_position @ (x, y)) = next_position {
                    match plots.get(y).and_then(|l| l.get(x)) {
                        Some(id) if *id == plot_id => {
                            // same region
                            regions += 1;
                            plot_queue.extend(get_directions(x, y));
                            plots[y][x] = plot_id.to_ascii_lowercase();
                        }
                        Some(id) if *id == plot_id.to_ascii_lowercase() => {}
                        _ => {
                            // other region / outside
                            fences += 1;
                            next_queue.push(next_position);
                        }
                    }
                } else {
                    // outside
                    fences += 1;
                }
            }
            total_price += regions * fences;
            #[cfg(debug_assertions)]
            eprintln!(
                "Region {}, area = {}, perimeter = {}",
                char::from(plot_id),
                regions,
                fences
            );
        }
    }
    total_price
}

pub fn part2(input: &str) -> u32 {
    0
}

const DIRECTIONS: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];
fn get_directions(x: usize, y: usize) -> impl Iterator<Item = Result<(usize, usize), ()>> {
    DIRECTIONS.iter().map(move |(x1, y1)| {
        Ok((
            x.checked_add_signed(*x1).ok_or(())?,
            y.checked_add_signed(*y1).ok_or(())?,
        ))
    })
}

aoc2024::main!("../../inputs/day_12.txt");

aoc2024::test!(
    "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
    1930,
    1206
);
