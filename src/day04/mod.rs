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
        .filter(|passport| {
            fields.iter().all(|k| {
                passport.contains_key(k) && {
                    let v = passport.get(k).unwrap().to_string();
                    match *k {
                        "byr" => Byr(v).is_valid(),
                        "eyr" => Eyr(v).is_valid(),
                        "iyr" => Iyr(v).is_valid(),
                        "hgt" => Hgt(v).is_valid(),
                        "hcl" => Hcl(v).is_valid(),
                        "ecl" => Ecl(v).is_valid(),
                        "pid" => Pid(v).is_valid(),
                        _ => false,
                    }
                }
            })
        })
        .count()
}

trait Validation {
    fn is_valid(&self) -> bool;
}

struct Byr(String);

impl Validation for Byr {
    fn is_valid(&self) -> bool {
        parse_digits(&self.0).map_or(false, |n| n >= 1920 && n <= 2002)
    }
}

struct Iyr(String);

impl Validation for Iyr {
    fn is_valid(&self) -> bool {
        parse_digits(&self.0).map_or(false, |n| n >= 2010 && n <= 2020)
    }
}

struct Eyr(String);

impl Validation for Eyr {
    fn is_valid(&self) -> bool {
        parse_digits(&self.0).map_or(false, |n| n >= 2020 && n <= 2030)
    }
}

struct Hgt(String);

impl Validation for Hgt {
    fn is_valid(&self) -> bool {
        parse_digits(&self.0).map_or(false, |n| {
            let unit = &self
                .0
                .chars()
                .skip_while(|c| c.is_ascii_digit())
                .collect::<String>();
            (unit == "cm" && n >= 150 && n <= 193) || (unit == "in" && n >= 59 && n <= 76)
        })
    }
}

struct Hcl(String);

impl Validation for Hcl {
    fn is_valid(&self) -> bool {
        self.0.starts_with('#')
            && self.0.len() == 7
            && self
                .0
                .chars()
                .skip(1)
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    }
}

struct Ecl(String);

impl Validation for Ecl {
    fn is_valid(&self) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.0.as_str())
    }
}

struct Pid(String);

impl Validation for Pid {
    fn is_valid(&self) -> bool {
        self.0.len() == 9 && self.0.chars().all(|c| c.is_ascii_digit())
    }
}

fn parse_digits(s: &str) -> Option<u16> {
    s.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
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
