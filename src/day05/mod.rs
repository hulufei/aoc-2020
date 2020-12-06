const INPUT: &str = include_str!("./input");

fn parse_seat_id(input: &str) -> i32 {
    let s = input
        .chars()
        .map(|c| match c {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            other => other,
        })
        .collect::<String>();
    i32::from_str_radix(&s, 2).unwrap()
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
