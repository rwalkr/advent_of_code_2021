use std::{path::PathBuf, collections::VecDeque};
use advent_of_code_2021::iter_lines;
use structopt::StructOpt;

const DATA_FILE: &str = "data/day_09.txt";

#[derive(StructOpt)]
struct Opts {
    part_no: usize,

    #[structopt(parse(from_os_str), default_value = DATA_FILE)]
    filename: PathBuf,
}

type Grid = Vec<Vec<u32>>;

fn parse_input(lines: impl Iterator<Item = String>) -> Grid {
    lines.into_iter()
        .map(|s| {
            s.trim().bytes().map(|i| (i - b'0') as u32 ).collect()
        })
        .collect()
}

fn is_low_point(g: &Grid, r: usize, c: usize) -> Option<u32> {
    const INV: u32 = 999;
    let h = g[r][c];
    let u = if r > 0 { g[r-1][c] } else { INV };
    let d = if r < g.len()-1 { g[r+1][c] } else { INV };
    let l = if c > 0 { g[r][c-1] } else { INV };
    let r = if c < g[0].len()-1 { g[r][c+1] } else { INV };
    if h < u && h < d && h < l && h < r {
        Some(h)
    } else {
        None
    }
}

fn part_1(lines: impl Iterator<Item = String>) -> u32 {
    let grid = parse_input(lines);
    let mut sum = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if let Some(h) = is_low_point(&grid, r, c) {
                let v = h + 1;
                sum += v;
            }
        }
    }
    sum
}

fn part_2(lines: impl Iterator<Item = String>) -> i32 {
    let mut grid = parse_input(lines);

    // find the low points
    let mut lows= vec![];
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if let Some(_) = is_low_point(&grid, r, c) {
                lows.push((r, c));
            }
        }
    }

    // mark the basins with numbers >= 10

    // work queue says which cells to examine next
    //  start from the low points
    let mut work_queue = VecDeque::new();
    for (i, p) in lows.iter().enumerate() {
        let v = 10 + i as u32;
        work_queue.push_back((*p, v));
    }

    // process the queue
    //   if the cell is not already marked, or not high (9), then mark it and
    //   queue its neighbours for examination
    while !work_queue.is_empty() {
        let ((r, c), t) = work_queue.pop_front().unwrap();
        let v = grid[r][c];
        if v >= 9 {
            continue;
        }
        grid[r][c] = t;
        // enqueue neighbours
        if c > 0 {
            work_queue.push_back(((r, c-1), t));
        }
        if c < grid[r].len()-1 {
            work_queue.push_back(((r, c+1), t));
        }
        if r > 0 {
            work_queue.push_back(((r-1, c), t));
        }
        if r < grid.len()-1 {
            work_queue.push_back(((r+1, c), t));
        }
    }

    // build the histogram of the basin IDs
    let mut basin_sizes = vec![0; lows.len()];
    for r in grid {
        for v in r {
            if v > 9 {
                let b = v - 10;
                basin_sizes[b as usize] += 1;
            }
        }
    }

    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).fold(1, |acc, v| acc*v)
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

    static TEST_DATA: &str = r"2199943210
    3987894921
    9856789892
    8767896789
    9899965678";

    fn test_data() -> impl Iterator<Item = String> {
        TEST_DATA.lines().map(String::from)
    }

    fn real_data() -> impl Iterator<Item = String> {
        iter_lines(DATA_FILE)
    }

    #[test]
    pub fn test_parse() {
        let data = parse_input(test_data());
        assert_eq!(5, data.len());
        assert_eq!(10, data[0].len());
        assert_eq!(10, data[4].len());
        assert_eq!(vec![2, 1, 9, 9], data[0][0..4]);
    }

    #[test]
    pub fn test_part_1() {
        assert_eq!(15, part_1(test_data()));
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(1134, part_2(test_data()));
    }

    #[test]
    pub fn test_part_1_real() {
        assert_eq!(554, part_1(real_data()));
    }

    #[test]
    pub fn test_part_2_real() {
        assert_eq!(1017792, part_2(real_data()));
    }
}
