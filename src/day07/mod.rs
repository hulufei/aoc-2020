use std::collections::HashMap;

const INPUT: &str = include_str!("./input");

fn parse_line(line: &str) -> (&str, &str) {
    let mut iter = line.split(" bags contain ");
    (iter.next().unwrap(), iter.next().unwrap())
}

fn part_1(input: &str) -> usize {
    let mut bags = input
        .trim_end()
        .lines()
        .map(parse_line)
        .collect::<HashMap<_, _>>();
    let mut sub_bags = vec!["shiny gold"];
    let mut count = 0;
    while sub_bags.len() > 0 {
        let pair: (Vec<(&str, &str)>, Vec<(&str, &str)>) = bags
            .into_iter()
            .partition(|(_, v)| sub_bags.iter().any(|x| v.contains(x)));
        sub_bags = pair.0.iter().map(|x| x.0).collect();
        bags = pair.1.into_iter().collect();
        count += sub_bags.len();
    }
    count
}

fn parse_line_2(line: &str) -> (&str, Vec<(usize, String)>) {
    let mut iter = line.split(" bags contain ");
    let k = iter.next().unwrap();
    let bags = iter
        .next()
        .unwrap()
        .split(", ")
        .filter_map(|p| {
            let mut parts = p.split_whitespace();
            let count = parts.nth(0).and_then(|x| x.parse::<usize>().ok());
            parts.nth_back(0);
            let bag = parts.collect::<Vec<_>>().join(" ");
            count.map(|x| (x, bag))
        })
        .collect();
    (k, bags)
}

fn count_sum(bags: &HashMap<&str, Vec<(usize, String)>>, entry: &str) -> usize {
    1_usize
        + bags[entry]
            .iter()
            .map(|(count, next_entry)| count * count_sum(bags, next_entry))
            .sum::<usize>()
}

fn part_2(input: &str) -> usize {
    let bags: HashMap<&str, Vec<(usize, String)>> =
        input.trim_end().lines().map(parse_line_2).collect();
    count_sum(&bags, "shiny gold") - 1
}

#[test]
fn test_part_1() {
    assert_eq!(
        part_1(
            r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
            "#
        ),
        4
    );
    assert_eq!(part_1(INPUT), 121);
}

#[test]
fn test_parse_line_2() {
    assert_eq!(
        parse_line_2("light red bags contain 1 bright white bag, 2 muted yellow bags."),
        (
            "light red",
            vec![
                (1, "bright white".to_owned()),
                (2, "muted yellow".to_owned())
            ]
        )
    );
    assert_eq!(
        parse_line_2("faded blue bags contain no other bags."),
        ("faded blue", vec![])
    );
}

#[test]
fn test_part_2() {
    assert_eq!(
        part_2(
            r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
                "#
        ),
        32
    );
    assert_eq!(
        part_2(
            r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"#
        ),
        126
    );
    assert_eq!(part_2(INPUT), 3805);
}
