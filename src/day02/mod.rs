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
    let first_match = iter.nth(min - 1).unwrap() == c;
    let second_match = iter.nth(max - min - 1).unwrap() == c;
    first_match ^ second_match
}

fn get_valid_count<F>(check: F) -> usize
where
    F: Fn(&str, &str) -> bool,
{
    INPUT
        .trim_end()
        .lines()
        .filter(|line| {
            let mut iter = line.split(':');
            let policy = iter.next().unwrap();
            let password = iter.next().unwrap().trim();
            check(policy, password)
        })
        .count()
}

fn part_1() -> usize {
    get_valid_count(is_valid)
}

fn part_2() -> usize {
    get_valid_count(is_valid_2)
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
