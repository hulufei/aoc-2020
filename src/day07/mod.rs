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
