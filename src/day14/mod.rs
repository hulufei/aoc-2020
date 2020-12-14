use std::collections::HashMap;

const INPUT: &str = include_str!("./input");

fn apply_mask(mask: &str, n: u64) -> u64 {
    let n = n | u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
    n & u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap()
}

fn parse_line(line: &str) -> (&str, &str) {
    let mut iter = line.split(" = ");
    (iter.next().unwrap(), iter.next().unwrap())
}

fn part_1(input: &str) -> u64 {
    let mut mem = HashMap::new();
    let mut lines = input.trim_end().lines();
    let (_, mut mask) = parse_line(lines.next().unwrap());
    for line in lines {
        let (left, right) = parse_line(line);
        if left == "mask" {
            mask = right;
        } else {
            let address = left[4..(left.len() - 1)].parse::<u64>().unwrap();
            mem.insert(address, apply_mask(mask, right.parse().unwrap()));
        }
    }
    mem.values().sum()
}

fn apply_mask_2(mask: &str, n: u64) -> (u64, u64) {
    // masked address
    let n = n | u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
    // x masks
    let x = n & u64::from_str_radix(&mask.replace("1", "0").replace("X", "1"), 2).unwrap();
    (n, x)
}

fn hash_equal(a: (u64, u64), b: (u64, u64)) -> bool {
    // remain bit should be x masks
    a.0 ^ b.0 == a.1 & b.1
}

fn part_2(input: &str) -> u64 {
    let mut mem = HashMap::new();
    let mut lines = input.trim_end().lines();
    let (_, mut mask) = parse_line(lines.next().unwrap());
    for line in lines {
        let (left, right) = parse_line(line);
        if left == "mask" {
            mask = right;
        } else {
            let address = left[4..(left.len() - 1)].parse::<u64>().unwrap();
            let (masked_address, x_masks) = apply_mask_2(mask, address);
            mem.insert(apply_mask_2(mask, address), right.parse().unwrap());
        }
    }
    mem.values().sum()
}

#[test]
fn test_apply_mask() {
    assert_eq!(apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 11), 73);
    assert_eq!(apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 101), 101);
    assert_eq!(apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 0), 64);
}

const EXAMPLE: &str = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
"#;

#[test]
fn test_part_1() {
    assert_eq!(part_1(EXAMPLE), 165);
    assert_eq!(part_1(INPUT), 18630548206046);
}
