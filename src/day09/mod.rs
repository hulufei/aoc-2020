use crate::day01::two_sum;

const INPUT: &str = include_str!("./input");

fn attack(xs: &[i64], window_size: usize) -> i64 {
    xs.windows(window_size + 1)
        .find(|slice| {
            slice
                .split_last()
                .and_then(|(x, preamble)| two_sum(preamble, *x))
                .is_none()
        })
        .and_then(|win| win.last().copied())
        .unwrap()
}

fn encrypt(xs: &[i64], target: i64) -> i64 {
    let (mut i, mut j, mut sum) = (0, 0, 0);
    while sum != target {
        if sum < target {
            sum += xs[j];
            j += 1;
        } else {
            sum -= xs[i];
            i += 1;
        }
    }
    let min = xs[i..j].iter().min().unwrap();
    let max = xs[i..j].iter().max().unwrap();
    min + max
}

fn parse_numbers(input: &str) -> Vec<i64> {
    input
        .trim_end()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>()
}

fn part_1(input: &str, window_size: usize) -> i64 {
    let numbers = parse_numbers(input);
    attack(&numbers, window_size)
}

fn part_2(input: &str, window_size: usize) -> i64 {
    let numbers = parse_numbers(input);
    let target = attack(&numbers, window_size);
    encrypt(&numbers, target)
}

const TEST_INPUT: &str = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"#;

#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT, 5), 127);
    assert_eq!(part_1(INPUT, 25), 41682220);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(TEST_INPUT, 5), 62);
    assert_eq!(part_2(INPUT, 25), 5388976);
}
