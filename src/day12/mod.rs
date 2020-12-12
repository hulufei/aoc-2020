const INPUT: &str = include_str!("./input");

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let (dir, distance) = line.split_at(1);
            (dir.parse().unwrap(), distance.parse().unwrap())
        })
        .collect()
}

fn transform((x, y): (i32, i32), (dir, distance): (char, i32)) -> (i32, i32) {
    // println!("from {:?} to {:?}", (x, y), (dir, distance));
    match dir {
        'N' => (x, y - distance),
        'S' => (x, y + distance),
        'E' => (x + distance, y),
        'W' => (x - distance, y),
        _ => (x, y),
    }
}

fn part_1(input: &str) -> i32 {
    let clockwise = ['E', 'S', 'W', 'N'];
    let mut face_index = 0;
    let (x, y, _) = parse_input(input)
        .iter()
        .fold((0, 0, 'E'), |(x, y, face), (dir, distance)| match dir {
            'L' => {
                face_index = (face_index + (360 - distance) / 90) % 4;
                let turn_to = clockwise[face_index as usize];
                (x, y, turn_to)
            }
            'R' => {
                face_index = (face_index + distance / 90) % 4;
                let turn_to = clockwise[face_index as usize];
                (x, y, turn_to)
            }
            'F' => {
                let (x, y) = transform((x, y), (face, *distance));
                (x, y, face)
            }
            _ => {
                let (x, y) = transform((x, y), (*dir, *distance));
                (x, y, face)
            }
        });
    x.abs() + y.abs()
}

const EXAMPLE: &str = r#"F10
N3
F7
R90
F11
"#;

#[test]
fn test_part_1() {
    assert_eq!(part_1(EXAMPLE), 25);
    assert_eq!(part_1(INPUT), 1956);
}
