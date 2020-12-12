const INPUT: &str = include_str!("./input");

type Instruction = (char, u32);

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let (dir, distance) = line.split_at(1);
            (dir.parse().unwrap(), distance.parse().unwrap())
        })
        .collect()
}

fn transform((x, y): (i32, i32), (dir, distance): Instruction) -> (i32, i32) {
    // println!("from {:?} to {:?}", (x, y), (dir, distance));
    let distance = distance as i32;
    match dir {
        'N' => (x, y - distance),
        'S' => (x, y + distance),
        'E' => (x + distance, y),
        'W' => (x - distance, y),
        _ => (x, y),
    }
}

type FaceIndex = u32;

fn turn_face(face_index: FaceIndex, (dir, distance): Instruction) -> FaceIndex {
    match dir {
        'L' => (face_index + (360 - distance) / 90) % 4,
        'R' => (face_index + distance / 90) % 4,
        _ => unreachable!(),
    }
}

const CLOCKWISE: [char; 4] = ['E', 'S', 'W', 'N'];

#[derive(Debug)]
struct Waypoint {
    x: (FaceIndex, u32),
    y: (FaceIndex, u32),
}

impl Waypoint {
    fn new(x: (FaceIndex, u32), y: (FaceIndex, u32)) -> Self {
        Self { x, y }
    }

    fn turn(&mut self, instruction: Instruction) {
        self.x.0 = turn_face(self.x.0, instruction);
        self.y.0 = turn_face(self.y.0, instruction);
        println!("turn waypoint to {:?}", self);
    }

    fn transform(&mut self, (dir, distance): Instruction) {
        let ((face_x, distance_x), (face_y, distance_y)) = (self.x, self.y);
        let dir_x = CLOCKWISE[face_x as usize];
        let dir_y = CLOCKWISE[face_y as usize];
        println!(
            "transform waypoint from {:?} to {:?}",
            ((dir_x, distance_x), (dir_y, distance_y)),
            (dir, distance)
        );
        let (x, y) = match dir {
            'N' | 'S' => {
                let face = if dir == dir_y {
                    (face_y, distance + distance_y)
                } else if distance_y < distance {
                    ((face_y + 2) % 4, distance - distance_y)
                } else {
                    (face_y, distance_y - distance)
                };
                ((face_x, distance_x), face)
            }
            'E' | 'W' => {
                let face = if dir == dir_x {
                    (face_x, distance + distance_x)
                } else if distance_x < distance {
                    ((face_x + 2) % 4, distance - distance_x)
                } else {
                    (face_x, distance_x - distance)
                };
                (face, (face_y, distance_y))
            }
            _ => unreachable!(),
        };
        self.x = x;
        self.y = y;
    }
}

fn part_1(input: &str) -> i32 {
    let mut face_index = 0;
    let (x, y, _) = parse_input(input)
        .iter()
        .fold((0, 0, 'E'), |(x, y, face), (dir, distance)| match dir {
            'L' | 'R' => {
                face_index = turn_face(face_index, (*dir, *distance));
                let turn_to = CLOCKWISE[face_index as usize];
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

fn part_2(input: &str) -> u32 {
    let mut waypoint = Waypoint::new((0, 10), (3, 1));
    let mut ship = Waypoint::new((0, 0), (3, 0));
    for (dir, distance) in parse_input(input) {
        match dir {
            'L' | 'R' => waypoint.turn((dir, distance)),
            'F' => {
                println!("transform ship:");
                ship.transform((CLOCKWISE[waypoint.x.0 as usize], distance * waypoint.x.1));
                ship.transform((CLOCKWISE[waypoint.y.0 as usize], distance * waypoint.y.1))
            }
            _ => waypoint.transform((dir, distance)),
        }
    }
    ship.x.1 + ship.y.1
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

#[test]
fn test_part_2() {
    assert_eq!(part_2(EXAMPLE), 286);
    assert_eq!(part_2(INPUT), 19051);
}
