fn main() {
    let input = include_str!("day1_test.txt");
    println!("PART 1: {}", solve_part1(input));
    println!("PART 2: {}", solve_part2(input));
}

#[test]
fn test() {
    let input = include_str!("day1_test.txt");
    assert_eq!(solve_part1(input), 11);
    //assert_eq!(solve_part2(input), Ok(11));
}

fn solve_part1(input: &str) -> i32 {
    let pairs: Vec<(i32, i32)> = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .collect();

    todo!()
}

fn solve_part2(input: &str) -> i32 {
    todo!()
}
