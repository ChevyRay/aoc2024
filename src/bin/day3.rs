use regex::{Match, Regex};

fn main() {
    let input = include_str!("day3.txt");
    println!("PART 1: {}", solve_part1(input));
    println!("PART 2: {}", solve_part2(input));
}

#[test]
fn test() {
    assert_eq!(
        solve_part1(r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
        161
    );
    assert_eq!(
        solve_part2(r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
        48
    );
}

fn solve_part1(input: &str) -> usize {
    Regex::new(r"mul\(\d{1,3},\d{1,3}\)")
        .unwrap()
        .find_iter(input)
        .map(|m| m.as_str())
        .map(|op| op[4..(op.len() - 1)].split_once(',').unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .map(|(a, b)| a * b)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)")
        .unwrap()
        .find_iter(input)
        .map(|op| op.as_str())
        .fold((true, 0), |(on, sum), op| match op {
            "do()" => (true, sum),
            "don't()" => (false, sum),
            op if on => {
                let (a, b) = op[4..(op.len() - 1)].split_once(',').unwrap();
                let (a, b) = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());
                (true, sum + a * b)
            }
            _ => (on, sum),
        })
        .1
}
