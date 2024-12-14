use std::collections::HashSet;

use aoc::Aoc;
struct Day6;

const EXAMPLE: &'static str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn parse(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

impl Aoc for Day6 {
    const YEAR: usize = 2024;
    const DAY: usize = 6;
    // fn input() -> String {
    //     EXAMPLE.into()
    // }
    fn part1(input: String) -> impl std::fmt::Display {
        let board = parse(input);
        let (mut i, mut j) = board
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                (row.iter().enumerate().find(|(_, &cell)| cell == '^'))
                    .map(|(j, _)| (i as i32, j as i32))
            })
            .unwrap();
        let mut seen = HashSet::new();
        let mut dir_idx = 0;
        while i >= 0 && j >= 0 && i < board.len() as i32 && j < board[0].len() as i32 {
            let (di, dj) = DIR[dir_idx];
            if board[i as usize][j as usize] == '#' {
                dir_idx = (dir_idx + 1) % 4;
                i -= di;
                j -= dj;
                seen.remove(&(i, j));
                continue;
            }
            seen.insert((i, j));
            i += di;
            j += dj;
        }
        seen.len()
    }
    fn part2(input: String) -> impl std::fmt::Display {
        let mut board = parse(input);
        let clear_board = board.clone();
        let mut counter = 0;
        for y in 0..board.len() {
            'outer: for x in 0..board[0].len() {
                if board[y][x] == '.' {
                    board = clear_board.clone();
                    board[y][x] = '#';
                } else {
                    continue;
                }
                let (mut i, mut j) = board
                    .iter()
                    .enumerate()
                    .find_map(|(i, row)| {
                        (row.iter().enumerate().find(|(_, &cell)| cell == '^'))
                            .map(|(j, _)| (i as i32, j as i32))
                    })
                    .unwrap();
                let mut seen = HashSet::new();
                let mut dir_idx = 0;
                let mut steps = 0;
                while i >= 0 && j >= 0 && i < board.len() as i32 && j < board[0].len() as i32 {
                    steps += 1;
                    let (di, dj) = DIR[dir_idx];
                    if board[i as usize][j as usize] == '#' {
                        dir_idx = (dir_idx + 1) % 4;
                        i -= di;
                        j -= dj;
                        seen.remove(&(i, j));
                        if steps > board.len() * board[0].len() {
                            counter += 1;
                            continue 'outer;
                        }
                        continue;
                    }
                    seen.insert((i, j));
                    i += di;
                    j += dj;
                }
            }
        }
        counter
    }
}

fn main() {
    let input = Day6::input();
    println!("Part 1: {}", Day6::part1(input.to_string()));
    println!("Part 2: {}", Day6::part2(input.to_string()));
}
