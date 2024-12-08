use aoc::Aoc;
use regex::Regex;

struct Day3;

const EXAMPLE: &'static str =
    r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

impl Aoc for Day3 {
    const YEAR: usize = 2024;
    const DAY: usize = 3;
    // fn input() -> String {
    //     EXAMPLE.into()
    // }
    fn part1(input: String) -> impl std::fmt::Display {
        let re = Regex::new(r#"mul\([0-9]*\,[0-9]*\)"#).unwrap();
        re.find_iter(&input)
            .map(|str| {
                let (left, right) = str.as_str().split_once(",").unwrap();
                println!("multing {left} * {right}");
                left["mul(".len()..].parse::<usize>().unwrap()
                    * right[..right.len() - 1].parse::<usize>().unwrap()
            })
            .sum::<usize>()
    }
    fn part2(input: String) -> impl std::fmt::Display {
        let re = Regex::new(r#"mul\([0-9]*\,[0-9]*\)|do\(\)|don\'t\(\)"#).unwrap();
        let mut r#do = true;
        re.find_iter(&input)
            .map(|str| {
                println!("{}", str.as_str());
                if str.as_str() == "do()" {
                    r#do = true;
                    return 0;
                } else if str.as_str() == "don't()" {
                    r#do = false;
                    return 0;
                }
                if !r#do {
                    return 0;
                }
                let (left, right) = str.as_str().split_once(",").unwrap();
                println!("multing {left} * {right}");
                left["mul(".len()..].parse::<usize>().unwrap()
                    * right[..right.len() - 1].parse::<usize>().unwrap()
            })
            .sum::<usize>()
    }
}

fn main() {
    let input = Day3::input();
    println!("Part 1: {}", Day3::part1(input.to_string()));
    println!("Part 2: {}", Day3::part2(input.to_string()));
}
