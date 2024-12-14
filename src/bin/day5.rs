use std::collections::HashMap;

use aoc::Aoc;
struct Day5;

const EXAMPLE: &'static str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

fn parse(input: String) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let (rules, orders) = input.split_once("\n\n").unwrap();
    let mut ordering = HashMap::new();
    for (k, v) in rules
        .lines()
        .filter_map(|line| line.split_once("|"))
        .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
    {
        ordering
            .entry(k)
            .and_modify(|vc: &mut Vec<i32>| {
                vc.push(v);
            })
            .or_insert(vec![v]);
    }
    let orders: Vec<Vec<i32>> = orders
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (ordering, orders)
}

impl Aoc for Day5 {
    const YEAR: usize = 2024;
    const DAY: usize = 5;
    // fn input() -> String {
    //     EXAMPLE.into()
    // }
    fn part1(input: String) -> impl std::fmt::Display {
        let (ordering, orders) = parse(input);
        orders
            .into_iter()
            .filter(|order| {
                order.iter().enumerate().all(|(idx, x)| {
                    idx == order.len() - 1
                        || order[idx + 1..]
                            .iter()
                            .all(|y| ordering.get(x).is_some_and(|x| x.contains(y)))
                })
            })
            .map(|order| order[(order.len() - 1) / 2])
            .sum::<i32>()
    }
    fn part2(input: String) -> impl std::fmt::Display {
        let (ordering, orders) = parse(input);
        orders
            .into_iter()
            .filter(|order| {
                !order.iter().enumerate().all(|(idx, x)| {
                    idx == order.len() - 1
                        || order[idx + 1..]
                            .iter()
                            .all(|y| ordering.get(x).is_some_and(|x| x.contains(y)))
                })
            })
            .filter_map(|order| {
                order.clone().into_iter().find(|&x| {
                    order
                        .iter()
                        .filter(|y| ordering.get(&y).is_some_and(|y| y.contains(&x)))
                        .count()
                        == (order.len() - 1) / 2
                })
            })
            .sum::<i32>()
    }
}

fn main() {
    let input = Day5::input();
    println!("Part 1: {}", Day5::part1(input.to_string()));
    println!("Part 2: {}", Day5::part2(input.to_string()));
}
