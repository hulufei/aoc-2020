const INPUT: &str = include_str!("./input");

type Grid = Vec<Vec<u8>>;

const DIR: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_adjacent_seats(grid: &Grid, (row, col): (usize, usize)) -> Vec<&u8> {
    DIR.iter()
        .map(|&(dy, dx)| (dy + row as i64, dx + col as i64))
        .filter_map(|(r, c)| grid.get(r as usize).and_then(|v| v.get(c as usize)))
        .collect()
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
                        b'L' if adjacent_seats.iter().all(|&&v| v != b'#') => b'#',
                        b'#' if adjacent_seats.iter().filter(|&&&v| v == b'#').count() >= 4 => b'L',
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
