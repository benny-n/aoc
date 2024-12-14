use aoc::Aoc;
struct Day7;

const EXAMPLE: &'static str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

fn parse(input: String) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            let (sum, parts) = line.split_once(":").unwrap();
            let mut vec = vec![sum.parse().unwrap()];
            vec.append(
                &mut parts
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            );
            vec
        })
        .collect()
}

impl Aoc for Day7 {
    const YEAR: usize = 2024;
    const DAY: usize = 7;
    // fn input() -> String {
    //     EXAMPLE.into()
    // }
    fn part1(input: String) -> impl std::fmt::Display {
        let calibs = parse(input);
        let mut total = 0;
        'calibs: for cal in calibs {
            let combinations = 2u32.pow(cal.len() as u32 - 2);
            for c in 0..combinations {
                let mut res = cal[1];
                for (i, &x) in cal[2..].iter().enumerate() {
                    if (c >> i) & 1 == 0 {
                        res += x;
                    } else {
                        res *= x;
                    }
                }
                if res == cal[0] {
                    total += res;
                    continue 'calibs;
                }
            }
        }
        total
    }
    fn part2(input: String) -> impl std::fmt::Display {
        let calibs = parse(input);
        let mut total = 0;
        'calibs: for cal in calibs {
            let combinations = 3u32.pow(cal.len() as u32 - 2);
            'combos: for c in 0..combinations {
                let mut res = cal[1];
                for (i, &x) in cal[2..].iter().enumerate() {
                    let r = (c / 3u32.pow(i as u32)) % 3;
                    if r == 0 {
                        res += x;
                    } else if r == 1 {
                        res *= x;
                    } else {
                        let concatenated = res.to_string() + x.to_string().as_str();
                        let Ok(concatenated) = concatenated.parse() else {
                            continue 'combos;
                        };
                        res = concatenated
                    }
                }
                if res == cal[0] {
                    total += res;
                    continue 'calibs;
                }
            }
        }
        total
    }
}

fn main() {
    let input = Day7::input();
    println!("Part 1: {}", Day7::part1(input.to_string()));
    println!("Part 2: {}", Day7::part2(input.to_string()));
}
