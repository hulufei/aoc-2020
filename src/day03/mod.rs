const INPUT: &str = include_str!("./input");

fn part_1() -> usize {
    count_by_slope(3, 1)
}

fn part_2() -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| count_by_slope(*x, *y))
        .product()
}

fn count_by_slope(step_x: usize, step_y: usize) -> usize {
    let mut col = 0;
    INPUT
        .trim_end()
        .lines()
        .step_by(step_y)
        .filter(|line| {
            let i = col % line.len();
            col += step_x;
            line.chars().nth(i).map(|c| c == '#').unwrap()
        })
        .count()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 280);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 4355551200);
}
