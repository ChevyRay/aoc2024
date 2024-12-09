use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("day5.txt");
    println!("PART 1: {}", solve_part1(input));
    println!("PART 2: {}", solve_part2(input));
}

#[test]
fn test() {
    let input = include_str!("day5_test.txt");
    assert_eq!(solve_part1(input), 143);
    assert_eq!(solve_part2(input), 123);
}

fn parse(input: &str) -> (HashMap<(usize, usize), Ordering>, Vec<Vec<usize>>) {
    let input = input.replace('\r', "");
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: HashMap<(usize, usize), Ordering> = rules
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .map(|(a, b)| [((a, b), Ordering::Less), ((b, a), Ordering::Greater)])
        .flatten()
        .collect();
    let updates: Vec<Vec<usize>> = updates
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn solve_part1(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter(|pages| pages.is_sorted_by(|&a, &b| rules.get(&(a, b)) == Some(&Ordering::Less)))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter(|pages| !pages.is_sorted_by(|&a, &b| rules.get(&(a, b)) == Some(&Ordering::Less)))
        .map(|mut pages| {
            pages.sort_by(|&a, &b| *rules.get(&(a, b)).unwrap_or(&Ordering::Equal));
            pages[pages.len() / 2]
        })
        .sum()
}
