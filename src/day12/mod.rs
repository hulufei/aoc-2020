use std::mem;
use std::ops::{Add, AddAssign, Mul};

const INPUT: &str = include_str!("./input");

type Instruction = (char, u32);

#[derive(Copy, Clone, Debug)]
struct Vector {
    x: i32,
    y: i32,
    face: char,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            ..self
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            face: self.face,
        };
    }
}

impl Mul<u32> for Vector {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        let rhs = rhs as i32;
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            ..self
        }
    }
}

impl Vector {
    fn new(x: i32, y: i32, face: char) -> Self {
        Self { x, y, face }
    }

    fn turn_clockwise(&mut self, degree: u32) {
        let clock = ['E', 'S', 'W', 'N'];
        let face_index = clock
            .iter()
            .enumerate()
            .find_map(|(i, &v)| if v == self.face { Some(i) } else { None })
            .unwrap();
        let degree = (face_index as u32 + (degree / 90)) % 4;
        self.face = clock[degree as usize];
    }

    fn rotate_clockwise(&mut self, degree: u32) {
        let degree = (degree / 90) % 4;
        match degree {
            1 => {
                mem::swap(&mut self.x, &mut self.y);
                self.y = -self.y;
            }
            3 => {
                mem::swap(&mut self.x, &mut self.y);
                self.x = -self.x;
            }
            2 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            _ => (),
        }
    }

    fn forward(&mut self, (dir, distance): Instruction) {
        let distance = distance as i32;
        match dir {
            'N' => self.y += distance,
            'S' => self.y -= distance,
            'E' => self.x += distance,
            'W' => self.x -= distance,
            _ => unreachable!(),
        }
    }
}

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

fn part_1(input: &str) -> i32 {
    let mut ship = Vector::new(0, 0, 'E');
    for (dir, v) in parse_input(input) {
        match dir {
            'F' => ship.forward((ship.face, v)),
            'L' => ship.turn_clockwise(360 - v),
            'R' => ship.turn_clockwise(v),
            _ => ship.forward((dir, v)),
        }
    }
    ship.x.abs() + ship.y.abs()
}

fn part_2(input: &str) -> i32 {
    let mut ship = Vector::new(0, 0, 'E');
    let mut waypoint = Vector::new(10, 1, 'E');
    for (dir, v) in parse_input(input) {
        // println!("instruction {:?}", (dir, v));
        match dir {
            'F' => ship += waypoint * v,
            'L' => waypoint.rotate_clockwise(360 - v),
            'R' => waypoint.rotate_clockwise(v),
            _ => waypoint.forward((dir, v)),
        }
        // println!("ship {:?}, waypoint {:?}", ship, waypoint);
    }
    ship.x.abs() + ship.y.abs()
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
    assert_eq!(part_2(INPUT), 126797);
}
