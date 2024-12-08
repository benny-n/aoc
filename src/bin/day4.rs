use aoc::Aoc;
struct Day4;

const EXAMPLE: &'static str = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;

fn parse(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn search(matrix: &[Vec<char>], (mut y, mut x): (i32, i32), (dx, dy): (i32, i32)) -> bool {
    for c in ['M', 'A', 'S'] {
        x += dx;
        y += dy;
        if x < 0
            || y < 0
            || x >= matrix[0].len() as i32
            || y >= matrix.len() as i32
            || matrix[y as usize][x as usize] != c
        {
            return false;
        }
    }
    true
}

fn search2(matrix: &[Vec<char>], (oy, ox): (i32, i32), seq: &str) -> bool {
    for (c, (dx, dy)) in seq.chars().zip([(1, 1), (1, -1), (-1, -1), (-1, 1)].iter()) {
        let x = ox + dx;
        let y = oy + dy;
        if x < 0
            || y < 0
            || x >= matrix[0].len() as i32
            || y >= matrix.len() as i32
            || matrix[y as usize][x as usize] != c
        {
            return false;
        }
    }
    true
}

impl Aoc for Day4 {
    const YEAR: usize = 2024;
    const DAY: usize = 4;
    // fn input() -> String {
    //     EXAMPLE.into()
    // }
    fn part1(input: String) -> impl std::fmt::Display {
        let matrix = parse(input);
        let mut counter = 0;
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                for (dx, dy) in [
                    (0, 1),
                    (1, 0),
                    (0, -1),
                    (-1, 0),
                    (1, 1),
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                ] {
                    if matrix[y][x] == 'X' && search(&matrix, (y as i32, x as i32), (dx, dy)) {
                        counter += 1
                    }
                }
            }
        }
        counter
    }
    fn part2(input: String) -> impl std::fmt::Display {
        let matrix = parse(input);
        let mut counter = 0;
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                for seq in ["MMSS", "SMMS", "SSMM", "MSSM"] {
                    if matrix[y][x] == 'A' && search2(&matrix, (y as i32, x as i32), seq) {
                        counter += 1;
                        break;
                    }
                }
            }
        }
        counter
    }
}

fn main() {
    let input = Day4::input();
    println!("Part 1: {}", Day4::part1(input.to_string()));
    println!("Part 2: {}", Day4::part2(input.to_string()));
}
