fn main() {
    let input = include_str!("day4.txt");
    println!("PART 1: {}", solve_part1(input));
    println!("PART 2: {}", solve_part2(input));
}

#[test]
fn test() {
    let input = include_str!("day4_test.txt");
    assert_eq!(solve_part1(input), 18);
    assert_eq!(solve_part2(input), 9);
}

fn is_chr(chars: &[&[u8]], x: isize, y: isize, chr: u8) -> bool {
    chars
        .get(y as usize)
        .and_then(|row| row.get(x as usize))
        .is_some_and(|&c| c == chr)
}

fn solve_part1(input: &str) -> usize {
    let chars: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    (0..(chars.len() as isize))
        .map(|y| (0..(chars[0].len() as isize)).map(move |x| (x, y)))
        .flatten()
        .map(|(x, y)| {
            [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ]
            .into_iter()
            .filter(|(dx, dy)| {
                is_chr(&chars, x, y, b'X')
                    & is_chr(&chars, x + dx, y + dy, b'M')
                    & is_chr(&chars, x + dx * 2, y + dy * 2, b'A')
                    & is_chr(&chars, x + dx * 3, y + dy * 3, b'S')
            })
            .count()
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let chars: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    (0..(chars.len() as isize))
        .map(|y| (0..(chars[0].len() as isize)).map(move |x| (x, y)))
        .flatten()
        .filter(|&(x, y)| {
            is_chr(&chars, x, y, b'A')
                && [b"MMSS", b"SMSM", b"MSMS", b"SSMM"].into_iter().any(|seq| {
                    is_chr(&chars, x - 1, y - 1, seq[0])
                        && is_chr(&chars, x + 1, y - 1, seq[1])
                        && is_chr(&chars, x - 1, y + 1, seq[2])
                        && is_chr(&chars, x + 1, y + 1, seq[3])
                })
        })
        .count()
}
