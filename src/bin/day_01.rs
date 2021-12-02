use std::path::{Path, PathBuf};

use advent_of_code_2021::read_lines;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    part_no: usize,

    #[structopt(parse(from_os_str), default_value = "data/day_01.txt")]
    filename: PathBuf,
}

fn part_1(filename: impl AsRef<Path>) -> usize {
    let lines = read_lines(filename).expect("Can't read input");
    let depths: Vec<u32> = lines
        .into_iter()
        .filter_map(|l| l.unwrap().parse().ok())
        .collect();
    let increases = depths.windows(2).filter(|vals| vals[1] > vals[0]).count();
    increases
}

fn part_2(filename: impl AsRef<Path>) -> usize {
    let lines = read_lines(filename).expect("Can't read input");
    let depths: Vec<u32> = lines
        .into_iter()
        .filter_map(|l| l.unwrap().parse().ok())
        .collect();
    let windowed_sums: Vec<u32> = depths.windows(3).map(|vals| vals.iter().sum()).collect();
    let increases = windowed_sums
        .windows(2)
        .filter(|vals| vals[1] > vals[0])
        .count();
    increases
}

fn main() {
    let opts = Opts::from_args();
    match opts.part_no {
        1 => println!("{}", part_1(opts.filename)),
        2 => println!("{}", part_2(opts.filename)),
        _ => panic!("Invalid part"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_part_1() {
        assert_eq!(7, part_1("data/day_01_test.txt"));
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(5, part_2("data/day_01_test.txt"));
    }
}