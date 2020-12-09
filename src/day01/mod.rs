use std::{collections::HashSet, fs::read_to_string};

fn read_inputs() -> Vec<i64> {
    read_to_string("src/day01/input")
        .unwrap()
        .lines()
        .map(|line| line.parse::<_>().unwrap())
        .collect()
}

pub fn two_sum(input: &[i64], target: i64) -> Option<(i64, i64)> {
    let mut set = HashSet::new();
    for i in input {
        let j = target - i;
        if set.contains(i) {
            return Some((j, *i));
        }
        set.insert(j);
    }
    None
}

fn three_sum(input: &[i64], target: i64) -> Option<(i64, i64, i64)> {
    let len = input.len();
    for (i, v) in input.iter().enumerate().take(len - 1) {
        let remain_start = i + 1;
        let slice = &input[remain_start..];
        if let Some((a, b)) = two_sum(&slice, target - v) {
            return Some((*v, a, b));
        }
    }
    None
}

fn part_1() -> i64 {
    if let Some((a, b)) = two_sum(&read_inputs(), 2020) {
        a * b
    } else {
        0
    }
}

fn part_2() -> i64 {
    if let Some((a, b, c)) = three_sum(&read_inputs(), 2020) {
        a * b * c
    } else {
        0
    }
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 842016);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 9199664);
}

#[test]
fn test_two_sum() {
    assert_eq!(
        two_sum(&[1721, 979, 366, 299, 675, 1456,], 2020),
        Some((1721, 299))
    )
}

#[test]
fn test_three_sum() {
    assert_eq!(
        three_sum(&[1721, 979, 366, 299, 675, 1456,], 2020),
        Some((979, 366, 675))
    )
}
