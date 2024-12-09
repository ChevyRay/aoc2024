use itertools::Itertools;

fn main() {
    let input = include_str!("day2.txt");
    println!("PART 1: {}", solve_part1(input));
    println!("PART 2: {}", solve_part2(input));
}

#[test]
fn test() {
    let input = include_str!("day2_test.txt");
    assert_eq!(solve_part1(input), 2);
    assert_eq!(solve_part2(input), 4);
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.split(' '))
        .map(|strs| strs.map(|n| n.parse().unwrap()))
        .map(|nums| nums.collect())
        .collect()
}

fn valid(report: &Vec<usize>) -> bool {
    let asc = report[0] < report[1];
    report
        .into_iter()
        .tuple_windows()
        .all(|(a, b)| (a < b) == asc && (1..=3).contains(&a.abs_diff(*b)))
}

fn solve_part1(input: &str) -> usize {
    parse(input).into_iter().filter(valid).count()
}

fn solve_part2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|nums| {
            (0..nums.len()).any(|i| {
                let mut nums = nums.clone();
                nums.remove(i);
                valid(&nums)
            })
        })
        .count()
}
