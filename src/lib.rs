fn fetch_input(year: usize, day: usize) -> String {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let session = include_str!("../session.txt").trim();
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()
        .expect("request should succeed");
    resp.text().expect("response should be text")
}

#[test]
fn test_fetch_input() {
    let input = fetch_input(2024, 1);
    println!("{}", input);
}

pub trait Aoc {
    const YEAR: usize;
    const DAY: usize;
    fn input() -> String {
        match std::fs::read_to_string(format!("input/day{}.txt", Self::DAY)) {
            Ok(input) => input,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let input = fetch_input(Self::YEAR, Self::DAY);
                std::fs::write(format!("input/day{}.txt", Self::DAY), &input).ok();
                input
            }
            Err(e) => panic!("error reading input: {}", e),
        }
        .trim()
        .to_owned()
    }
    fn part1(input: String) -> impl std::fmt::Display;
    fn part2(input: String) -> impl std::fmt::Display;
}
