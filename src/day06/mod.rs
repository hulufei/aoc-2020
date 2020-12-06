use std::collections::HashSet;

const INPUT: &str = include_str!("./input");

fn parse_anyone_yes(input: &str) -> usize {
    input
        .trim_end()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|s| s.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn parse_everyone_yes(input: &str) -> usize {
    input
        .trim_end()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold(('a'..='z').collect::<HashSet<_>>(), |acc, b| {
                    acc.intersection(&b).cloned().collect()
                })
                .len()
        })
        .sum()
}

fn part_1() -> usize {
    parse_anyone_yes(INPUT)
}

fn part_2() -> usize {
    parse_everyone_yes(INPUT)
}

#[test]
fn test_parse_anyone_yes() {
    assert_eq!(
        parse_anyone_yes(
            r#"abc

a
b
c

ab
ac

a
a
a
a

b
"#
        ),
        11
    );
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 6521);
}

#[test]
fn test_parse_everyone_yes() {
    assert_eq!(
        parse_everyone_yes(
            r#"abc

a
b
c

ab
ac

a
a
a
a

b
"#
        ),
        6
    );
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 3305);
}
