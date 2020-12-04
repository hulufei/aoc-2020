use std::collections::HashMap;

const INPUT: &str = include_str!("./input");

fn parse_passport() -> Vec<HashMap<&'static str, &'static str>> {
    INPUT
        .split_terminator("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .map(|pair| {
                    let mut iter = pair.split(':');
                    (iter.next().unwrap(), iter.next().unwrap())
                })
                .collect()
        })
        .collect()
}

fn part_1() -> usize {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    parse_passport()
        .iter()
        .filter(|passport| fields.iter().all(|k| passport.contains_key(k)))
        .count()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 260);
}
