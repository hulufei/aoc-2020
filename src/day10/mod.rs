const INPUT: &str = include_str!("./input");

fn part_1(input: &str) -> usize {
    let sorted_arr = sort_input(input);
    let d_1 = sorted_arr.windows(2).filter(|&w| w[1] - w[0] == 1).count();
    let d_3 = sorted_arr.windows(2).filter(|&w| w[1] - w[0] == 3).count();
    d_1 * d_3
}

fn sort_input(input: &str) -> Vec<u32> {
    let mut sorted_arr = vec![0];
    let numbers = input
        .trim_end()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u32>>();
    sorted_arr.extend(numbers);
    sorted_arr.sort();
    sorted_arr.push(sorted_arr.last().unwrap() + 3);
    sorted_arr
}

fn total_arrange(sorted_arr: &[u32]) -> usize {
    let len = sorted_arr.len();
    if len == 1 {
        return 1;
    }
    let start = sorted_arr[0];
    sorted_arr
        .iter()
        .enumerate()
        .skip(1)
        .take_while(|(_, x)| (1..=3).contains(&(*x - start)))
        .map(|(i, _)| total_arrange(&sorted_arr[i..]))
        .sum::<usize>()
}

fn part_2(input: &str) -> usize {
    total_arrange(&sort_input(input))
}

const TEST_INPUT_1: &str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

const TEST_INPUT_2: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;

#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT_1), 35);
    assert_eq!(part_1(TEST_INPUT_2), 220);
    assert_eq!(part_1(INPUT), 1914);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(TEST_INPUT_1), 8);
    assert_eq!(part_2(TEST_INPUT_2), 19208);
    // assert_eq!(part_2(INPUT), 1914);
}
