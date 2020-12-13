const INPUT: &str = include_str!("./input");

fn parse_input(input: &str) -> (u32, Vec<Option<u32>>) {
    let mut lines = input.trim_end().lines();
    let start_timestamp = lines.next().unwrap().parse().unwrap();
    (
        start_timestamp,
        lines
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse().ok())
            .collect(),
    )
}

fn part_1(input: &str) -> u32 {
    let (start_timestamp, buses) = parse_input(input);
    let (id, min_timestamp) = buses
        .iter()
        .filter_map(|&id| id)
        .map(|id| {
            let interval = start_timestamp.div_euclid(id);
            let interval = if start_timestamp.rem_euclid(id) == 0 {
                interval
            } else {
                interval + 1
            };
            (id, id * interval)
        })
        .min_by_key(|(_, a)| *a)
        .unwrap();
    id * (min_timestamp - start_timestamp)
}

// from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

fn part_2(input: &str) -> i64 {
    let (_, buses) = parse_input(input);
    let buses = buses
        .iter()
        .enumerate()
        .filter_map(|(i, &id)| id.map(|v| (i, v as i64)))
        .collect::<Vec<_>>();
    let mods = buses.iter().map(|&(_, b)| b).collect::<Vec<_>>();
    let res = buses
        .iter()
        .map(|&(i, b)| (b - i as i64))
        .collect::<Vec<_>>();
    chinese_remainder(&res, &mods).unwrap()
}

const EXAMPLE: &str = r#"939
7,13,x,x,59,x,31,19"#;

#[test]
fn test_part_1() {
    assert_eq!(part_1(EXAMPLE), 295);
    assert_eq!(part_1(INPUT), 5946);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(EXAMPLE), 1068781);
    assert_eq!(part_2(INPUT), 645338524823718);
}
