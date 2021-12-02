use std::path::PathBuf;
use advent_of_code_2021::{from_split_lines, iter_lines};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    part_no: usize,

    #[structopt(parse(from_os_str), default_value = "data/day_01.txt")]
    filename: PathBuf,
}

fn parse_input(lines: impl Iterator<Item = String>) -> impl Iterator<Item = u32> {
    from_split_lines(lines, |args: [String; 1]| args[0].parse::<u32>().unwrap())
}

fn part_1(lines: impl Iterator<Item = String>) -> usize {
    let depths: Vec<u32> = parse_input(lines).collect();
    let increases = depths.windows(2).filter(|vals| vals[1] > vals[0]).count();
    increases
}

fn part_2(lines: impl Iterator<Item = String>) -> usize {
    let depths: Vec<u32> = parse_input(lines).collect();
    let windowed_sums: Vec<u32> = depths.windows(3).map(|vals| vals.iter().sum()).collect();
    let increases = windowed_sums
        .windows(2)
        .filter(|vals| vals[1] > vals[0])
        .count();
    increases
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

    static TEST_DATA: &str = r"199
200
208
210
200
207
240
269
260
263";

    fn test_data() -> impl Iterator<Item = String> {
        TEST_DATA.lines().map(String::from)
    }

    #[test]
    pub fn test_part_1() {
        assert_eq!(7, part_1(test_data()));
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(5, part_2(test_data()));
    }
}
