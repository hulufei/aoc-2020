use std::iter::repeat_with;

const INPUT: &str = include_str!("./input");

type Grid = Vec<Vec<u8>>;

fn get_adjacent_seats(grid: &Grid, (row, col): (usize, usize)) -> [Option<&u8>; 8] {
    let prev_row = row.checked_sub(1).and_then(|r| grid.get(r));
    let prev_col = col.checked_sub(1).unwrap_or(usize::MAX);
    // Clockwise
    [
        prev_row.and_then(|r| r.get(prev_col)),
        prev_row.and_then(|r| r.get(col)),
        prev_row.and_then(|r| r.get(col + 1)),
        grid.get(row).and_then(|r| r.get(col + 1)),
        grid.get(row + 1).and_then(|r| r.get(col + 1)),
        grid.get(row + 1).and_then(|r| r.get(col)),
        grid.get(row + 1).and_then(|r| r.get(prev_col)),
        grid.get(row).and_then(|r| r.get(prev_col)),
    ]
}

fn evolve(grid: &Grid) -> Grid {
    grid.iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(move |(c, &seat)| {
                    let adjacent_seats = get_adjacent_seats(grid, (r, c));
                    match seat {
                        b'L' if adjacent_seats.iter().all(|v| v.copied() != Some(b'#')) => b'#',
                        b'#' if adjacent_seats
                            .iter()
                            .filter(|v| v.copied() == Some(b'#'))
                            .count()
                            >= 4 =>
                        {
                            b'L'
                        }
                        other => other,
                    }
                })
                .collect()
        })
        .collect()
}

fn parse_input(input: &str) -> Grid {
    input
        .trim_end()
        .lines()
        .map(|l| l.as_bytes().iter().copied().collect())
        .collect()
}

fn part_1(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut next_grid = evolve(&grid);
    while grid != next_grid {
        grid = next_grid;
        next_grid = evolve(&grid);
    }
    grid.iter()
        .map(|r| r.iter().filter(|&v| *v == b'#').count())
        .sum()
}

const TEST_INPUT: &str = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#;

#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT), 37);
    assert_eq!(part_1(INPUT), 2338);
}
