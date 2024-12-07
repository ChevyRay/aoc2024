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

fn all_valid(levels: &[usize]) -> bool {
    let mut gt = None;
    levels.windows(2).map(|n| (n[0], n[1])).all(|(a, b)| {
        if gt.is_some_and(|gt| gt != (b > a)) {
            return false;
        }
        gt = Some(b > a);
        let diff = a.abs_diff(b);
        diff > 0 && diff < 4
    })
}

fn solve_part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|levels| all_valid(&levels))
        .count()
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
