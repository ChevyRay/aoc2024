fn main() {
    let input = include_str!("day1.txt");
    println!("PART 1: {}", solve_part1(input));
    println!("PART 2: {}", solve_part2(input));
}

#[test]
fn test() {
    let input = include_str!("day1_test.txt");
    assert_eq!(solve_part1(input), 11);
    assert_eq!(solve_part2(input), 31);
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .unzip()
}

fn solve_part1(input: &str) -> usize {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let (mut left, mut right) = parse(input);
    left.into_iter()
        .map(|a| a * right.iter().filter(|b| a.eq(b)).count())
        .sum()
}
