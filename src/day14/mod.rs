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

fn gen_replaced_key(a: (u64, u64), b: (u64, u64)) -> Option<(u64, u64)> {
    // merge x
    let x_masks = a.1 | b.1;
    let counter = a.0 ^ b.0;
    // remain bit should be x masks
    if counter | x_masks == x_masks {
        return Some((counter, x_masks));
    }
    None
}

fn part_2(input: &str) -> u64 {
    let mut mem: HashMap<(u64, u64), u64> = HashMap::new();
    let mut lines = input.trim_end().lines();
    let (_, mut mask) = parse_line(lines.next().unwrap());
    for line in lines {
        let (left, right) = parse_line(line);
        if left == "mask" {
            mask = right;
        } else {
            let address = left[4..(left.len() - 1)].parse::<u64>().unwrap();
            let masked_address = apply_mask_2(mask, address);
            let value = right.parse().unwrap();
            let mut replaced = false;
            mem = mem
                .iter()
                .map(|(&k, &v)| {
                    if let Some(new_k) = gen_replaced_key(k, masked_address) {
                        replaced = true;
                        (new_k, value)
                    } else {
                        (k, v)
                    }
                })
                .collect();
            if !replaced {
                mem.insert(masked_address, value);
            }
        }
    }
    mem.iter()
        .inspect(|v| {
            println!(
                "{:?}, x count = {}, pow 2 = {}",
                v,
                v.1.count_ones(),
                2_u64.pow(v.1.count_ones())
            )
        })
        .map(|(&(_, x_masks), &v)| 2_u64.pow(x_masks.count_ones()) * v)
        .sum()
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

#[test]
fn test_part_2() {
    assert_eq!(
        part_2(
            r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#
        ),
        208
    );
}
