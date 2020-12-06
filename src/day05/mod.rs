const INPUT: &str = include_str!("./input");

fn binary_locate(input: &str, acc: (i32, i32), lower_char: char, upper_char: char) -> i32 {
    input
        .chars()
        .fold(acc, |(lo, hi), c| {
            if lower_char == c {
                return (lo, lo + ((hi - lo) >> 1));
            }
            if upper_char == c {
                return (lo + ((hi - lo) >> 1) + 1, hi);
            }
            unreachable!()
        })
        .0
}

fn parse_seat_id(input: &str) -> i32 {
    let row = binary_locate(&input[0..(input.len() - 3)], (0, 127), 'F', 'B');
    let col = binary_locate(&input[(input.len() - 3)..], (0, 7), 'L', 'R');
    row * 8 + col
}

fn part_1() -> Option<i32> {
    INPUT.lines().map(parse_seat_id).max()
}

fn part_2() -> Option<i32> {
    let mut iter = INPUT.lines().map(parse_seat_id).collect::<Vec<_>>();
    iter.sort();
    iter.windows(2)
        .filter(|x| x[1] - x[0] == 2)
        .map(|x| x[0] + 1)
        .next()
}

#[test]
fn test_seat_id() {
    assert_eq!(parse_seat_id("FBFBBFFRLR"), 357);
    assert_eq!(parse_seat_id("BFFFBBFRRR"), 567);
    assert_eq!(parse_seat_id("FFFBBBFRRR"), 119);
    assert_eq!(parse_seat_id("BBFFBBFRLL"), 820);
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), Some(850));
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), Some(599));
}
