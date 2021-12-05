use std::{path::PathBuf, ops::Range};
use advent_of_code_2021::iter_lines;
use structopt::StructOpt;

const DATA_FILE: &str = "data/day_05.txt";

#[derive(StructOpt)]
struct Opts {
    part_no: usize,

    #[structopt(parse(from_os_str), default_value = DATA_FILE)]
    filename: PathBuf,
}

type Move = ((u32, u32), (u32, u32));

fn parse_input(lines: impl Iterator<Item = String>) -> Vec<Move> {
    lines.into_iter()
        .map(|s| {
            let (f, t) = s.trim().split_once(" -> ").unwrap();
            let (fx, fy) = f.split_once(",").unwrap();
            let (tx, ty) = t.split_once(",").unwrap();
            ((fx.parse().unwrap(), fy.parse().unwrap()), (tx.parse().unwrap(), ty.parse().unwrap()))
        })
        .collect::<Vec<Move>>()
}

fn is_horizontal(m: &Move) -> Option<(usize, Range<usize>)> {
    if m.0.1 == m.1.1 {
        let mut col_range = [ m.0.0 as usize, m.1.0 as usize ];
        col_range.sort();
        Some((m.0.1 as usize, col_range[0] .. col_range[1]+1))
    } else {
        None
    }
}

fn is_vertical(m: &Move) -> Option<(usize, Range<usize>)> {
    if m.0.0 == m.1.0 {
        let mut row_range = [ m.0.1 as usize, m.1.1 as usize ];
        row_range.sort();
        Some((m.0.0 as usize, row_range[0] .. row_range[1]+1))
    } else {
        None
    }
}

fn is_diagonal(m: &Move) -> Option<((i32, i32), i32, usize)> {
    let dx = m.0.0 as i32 -  m.1.0 as i32;
    if m.0.0 < m.1.0 {
        let dy = m.1.1 as i32 -  m.0.1 as i32;
        Some(((m.0.0 as i32, m.0.1 as i32), dy.signum(), dx.abs() as usize + 1))
    } else {
        let dy = m.0.1 as i32 -  m.1.1 as i32;
        Some(((m.1.0 as i32, m.1.1 as i32), dy.signum(), dx.abs() as usize + 1))
    }
}

#[allow(dead_code)]
fn print_board(b: &[[u8; 1000]; 1000]) {
    for r in 0..10 {
        for c in 0..10 {
            print!("{} ", b[r][c]);
        }
        println!("");
    }
}

fn part_1(lines: impl Iterator<Item = String>) -> u32 {
    const SZ: usize = 1000;
    let mut board = [[0u8; SZ]; SZ];
    for m in parse_input(lines) {
        if let Some((r, cols)) = is_horizontal(&m) {
            for c in cols {
                board[r][c] += 1;
            }
        }
        else if let Some((c, rows)) = is_vertical(&m) {
            for r in rows {
                board[r][c] += 1;
            }
        }
    }
    board.iter()
        .map(|r| r.iter().filter(|x| **x>1).count() as u32)
        .sum()
}

fn part_2(lines: impl Iterator<Item = String>) -> u32 {
    const SZ: usize = 1000;
    let mut board = [[0u8; SZ]; SZ];
    for m in parse_input(lines) {
        if let Some((r, cols)) = is_horizontal(&m) {
            for c in cols {
                board[r][c] += 1;
            }
        }
        else if let Some((c, rows)) = is_vertical(&m) {
            for r in rows {
                board[r][c] += 1;
            }
        }
        else if let Some(((c, r), dy, l)) = is_diagonal(&m) {
            for i in 0..l {
                board[(r + i as i32*dy) as usize][c as usize + i] += 1;
            }
        }
    }
    //print_board(&board);
    board.iter()
        .map(|r| r.iter().filter(|x| **x>1).count() as u32)
        .sum()
}

fn main() {
    let opts = Opts::from_args();
    let data = iter_lines(opts.filename);
    match opts.part_no {
        1 => println!("{}", part_1(data)),
        2 => println!("{}", part_2(data)),
        _ => panic!("Invalid part"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_DATA: &str = r"0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    fn test_data() -> impl Iterator<Item = String> {
        TEST_DATA.lines().map(String::from)
    }

    fn real_data() -> impl Iterator<Item = String> {
        iter_lines(DATA_FILE)
    }

    #[test]
    pub fn test_parse() {
        let moves = parse_input(test_data());
        assert_eq!(10, moves.len());
        assert_eq!((0, 9), moves[0].0);
        assert_eq!((5, 9), moves[0].1);
        assert_eq!((5, 5), moves[9].0);
        assert_eq!((8, 2), moves[9].1);

    }

    #[test]
    pub fn test_part_1() {
        assert_eq!(5, part_1(test_data()));
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(12, part_2(test_data()));
    }

    #[test]
    pub fn test_part_1_real() {
        assert_eq!(6007, part_1(real_data()));
    }

    #[test]
    pub fn test_part_2_real() {
        assert_eq!(19349, part_2(real_data()));
    }
}
