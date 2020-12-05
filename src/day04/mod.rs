use std::collections::HashMap;

const INPUT: &str = include_str!("./input");

fn parse_passport(input: &'static str) -> Vec<HashMap<&'static str, &'static str>> {
    input
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

fn part_1(input: &'static str) -> usize {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    parse_passport(input)
        .iter()
        .filter(|passport| fields.iter().all(|k| passport.contains_key(k)))
        .count()
}

fn part_2(input: &'static str) -> usize {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    parse_passport(input)
        .iter()
        .filter(|&passport| {
            fields.iter().all(|&k| match passport.get_key_value(k) {
                Some((&"byr", v)) => (1920..=2002).contains(&v.parse().unwrap_or(0)),
                Some((&"iyr", v)) => (2010..=2020).contains(&v.parse().unwrap_or(0)),
                Some((&"eyr", v)) => (2020..=2030).contains(&v.parse().unwrap_or(0)),
                Some((&"hcl", v)) => {
                    v.starts_with('#')
                        && v.len() == 7
                        && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
                }
                Some((&"ecl", v)) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
                Some((&"pid", v)) => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
                Some((&"hgt", v)) => {
                    let height = v[0..(v.len() - 2)].parse().unwrap_or(0);
                    match &v[(v.len() - 2)..] {
                        "cm" => (150..=193).contains(&height),
                        "in" => (59..=76).contains(&height),
                        _ => false,
                    }
                }
                _ => false,
            })
        })
        .count()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(INPUT), 260);
}

#[test]
fn test_part_2() {
    assert_eq!(
        part_2(
            r#"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#
        ),
        4
    );
    assert_eq!(part_2(INPUT), 153);
}
