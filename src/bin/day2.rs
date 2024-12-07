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
        .map(|nums| nums.map(|n| n.parse::<usize>().unwrap()))
        .map(|nums| nums.collect())
        .collect()
}

fn all_valid(nums: &Vec<usize>) -> bool {
    nums.windows(2)
        .map(|n| (n[0], n[1]))
        .all(|(a, b)| (a < b) == (nums[0] < nums[1]) && (1..=3).contains(&a.abs_diff(b)))
}

fn solve_part1(input: &str) -> usize {
    parse(input).into_iter().filter(all_valid).count()
}

fn solve_part2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|nums| {
            (0..nums.len()).any(|i| {
                let mut nums = nums.clone();
                nums.remove(i);
                all_valid(&nums)
            })
        })
        .count()
}
