const INPUT: &str = include_str!("./input");

fn parse_policy(policy: &str) -> (usize, usize, char) {
    let mut iter = policy.split_whitespace();
    let range = iter.next().unwrap();
    let c = iter.next().unwrap();

    let mut iter = range.split('-');
    let min = iter.next().map(|v| v.parse().unwrap());
    let max = iter.next().map(|v| v.parse().unwrap());
    (min.unwrap(), max.unwrap(), c.parse().unwrap())
}

fn is_valid(policy: &str, password: &str) -> bool {
    let (min, max, c) = parse_policy(policy);
    let count = password.chars().filter(|v| *v == c).count();
    count >= min && count <= max
}

fn is_valid_2(policy: &str, password: &str) -> bool {
    let (min, max, c) = parse_policy(policy);
    let mut iter = password.chars();
    let min_c = iter.nth(min - 1);
    let max_c = iter.nth(max - min - 1);
    match (min_c, max_c) {
        (Some(a), Some(b)) => (a == c && b != c) || (a != c && b == c),
        _ => false,
    }
}

fn part_1() -> usize {
    INPUT
        .trim_end()
        .lines()
        .filter(|line| {
            let mut iter = line.split(':');
            let policy = iter.next().unwrap();
            let password = iter.next().unwrap().trim();
            is_valid(policy, password)
        })
        .count()
}

fn part_2() -> usize {
    INPUT
        .trim_end()
        .lines()
        .filter(|line| {
            let mut iter = line.split(':');
            let policy = iter.next().unwrap();
            let password = iter.next().unwrap().trim();
            is_valid_2(policy, password)
        })
        .count()
}

#[test]
fn test_part_1() {
    assert!(is_valid("1-3 a", "abcde"));
    assert!(!is_valid("1-3 b", "cdefg"));
    assert!(is_valid("2-9 c", "ccccccccc"));
    assert_eq!(part_1(), 396);
}

#[test]
fn test_part_2() {
    assert!(is_valid_2("1-3 a", "abcde"));
    assert!(!is_valid_2("1-3 b", "cdefg"));
    assert!(!is_valid_2("2-9 c", "ccccccccc"));
    assert_eq!(part_2(), 428);
}
