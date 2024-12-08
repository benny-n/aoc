use aoc::Aoc;

struct Day2;

const EXAMPLE: &'static str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

fn is_safe(levels: Vec<usize>) -> bool {
    println!("checking {:?}", levels);
    (levels.is_sorted() || levels.iter().rev().is_sorted())
        && (0..levels.len() - 1).all(|idx| {
            let diff = levels[idx].abs_diff(levels[idx + 1]);
            diff >= 1 && diff <= 3
        })
}

impl Aoc for Day2 {
    const YEAR: usize = 2024;
    const DAY: usize = 2;
    // fn input() -> String {
    //     EXAMPLE.into()
    // }
    fn part1(input: String) -> impl std::fmt::Display {
        input
            .lines()
            .filter(|line| {
                is_safe(
                    line.split_ascii_whitespace()
                        .filter_map(|level| level.parse().ok())
                        .collect(),
                )
            })
            .count()
    }
    fn part2(input: String) -> impl std::fmt::Display {
        input
            .lines()
            .filter(|line| {
                let levels: Vec<usize> = line
                    .split_ascii_whitespace()
                    .filter_map(|level| level.parse().ok())
                    .collect();
                is_safe(levels.clone())
                    || is_safe(levels[1..].to_vec())
                    || is_safe(levels[..levels.len() - 1].to_vec())
                    || (2..levels.len()).any(|idx| {
                        is_safe(
                            vec![levels[0..=(idx - 2)].to_vec(), levels[idx..].to_vec()]
                                .into_iter()
                                .flatten()
                                .collect(),
                        )
                    })
            })
            .count()
    }
}

fn main() {
    // 482 too high
    // 456 too low
    let input = Day2::input();
    println!("Part 1: {}", Day2::part1(input.to_string()));
    println!("Part 2: {}", Day2::part2(input.to_string()));
}
