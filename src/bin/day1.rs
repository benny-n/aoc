use aoc::Aoc;

struct Day1;

const EXAMPLE: &'static str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

fn lists(input: String) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(l, r)| {
            (
                l.trim().parse::<usize>().unwrap(),
                r.trim().parse::<usize>().unwrap(),
            )
        })
        .unzip()
}

impl Aoc for Day1 {
    const YEAR: usize = 2024;
    const DAY: usize = 1;
    // fn input() -> String {
    //     EXAMPLE.into()
    // }
    fn part1(input: String) -> impl std::fmt::Display {
        let (mut left, mut right) = lists(input);

        left.sort();
        right.sort();

        left.into_iter()
            .zip(right)
            .fold(0, |acc: usize, (l, r)| acc + l.abs_diff(r))
    }
    fn part2(input: String) -> impl std::fmt::Display {
        let (left, right) = lists(input);
        left.into_iter().fold(0, |acc, target| {
            acc + target * right.iter().filter(|&&x| x == target).count()
        })
    }
}

fn main() {
    let input = Day1::input();
    println!("Part 1: {}", Day1::part1(input.to_string()));
    println!("Part 2: {}", Day1::part2(input.to_string()));
}
