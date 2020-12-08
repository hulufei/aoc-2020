use std::collections::HashSet;

const INPUT: &str = include_str!("./input");

type Operation = String;

type Argument = i32;

type Instruction = (Operation, Argument);

type Address = usize;

type Program = Vec<Instruction>;

fn exec(program: &Program, start: Address, visited: &mut HashSet<Address>, acc: i32) -> i32 {
    if visited.contains(&start) {
        return acc;
    }
    if let Some((operation, argument)) = program.get(start) {
        visited.insert(start);
        match operation.as_str() {
            "nop" => exec(program, start + 1, visited, acc),
            "jmp" => exec(program, ((start as i32) + *argument) as usize, visited, acc),
            "acc" => exec(program, start + 1, visited, acc + argument),
            _ => panic!("Unknown operation {}", operation),
        }
    } else {
        unreachable!();
    }
}

fn exec_terminate(
    program: &Program,
    start: Address,
    visited: &mut HashSet<Address>,
    switched: bool,
    acc: i32,
) -> (i32, bool) {
    if start == program.len() {
        return (acc, true);
    }
    if visited.contains(&start) {
        return (acc, false);
    }
    if let Some((operation, argument)) = program.get(start) {
        visited.insert(start);
        match operation.as_str() {
            "nop" => match exec_terminate(program, start + 1, visited, switched, acc) {
                (_, false) if !switched => exec_terminate(
                    program,
                    ((start as i32) + *argument) as usize,
                    visited,
                    true,
                    acc,
                ),
                other => other,
            },
            "jmp" => {
                match exec_terminate(
                    program,
                    ((start as i32) + *argument) as usize,
                    visited,
                    switched,
                    acc,
                ) {
                    (_, false) if !switched => {
                        exec_terminate(program, start + 1, visited, true, acc)
                    }
                    other => other,
                }
            }
            "acc" => exec_terminate(program, start + 1, visited, switched, acc + argument),
            _ => panic!("Unknown operation {}", operation),
        }
    } else {
        unreachable!();
    }
}

fn parse_program(input: &str) -> Program {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let operation = iter.next().unwrap();
            let argument = iter.next().unwrap().parse().unwrap();
            (operation.to_owned(), argument)
        })
        .collect()
}

fn part_1(input: &str) -> i32 {
    exec(&parse_program(input), 0, &mut HashSet::new(), 0)
}

fn part_2(input: &str) -> i32 {
    exec_terminate(&parse_program(input), 0, &mut HashSet::new(), false, 0).0
}

#[test]
fn test_part_1() {
    assert_eq!(
        part_1(
            r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"#
        ),
        5
    );
    assert_eq!(part_1(INPUT), 1928);
}

#[test]
fn test_part_2() {
    assert_eq!(
        part_2(
            r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"#
        ),
        8
    );
    assert_eq!(part_2(INPUT), 1319);
}
