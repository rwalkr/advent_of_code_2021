use std::path::PathBuf;
use advent_of_code_2021::iter_lines;
use structopt::StructOpt;

const DATA_FILE: &str = "data/day_07.txt";

#[derive(StructOpt)]
struct Opts {
    part_no: usize,

    #[structopt(parse(from_os_str), default_value = DATA_FILE)]
    filename: PathBuf,
}

fn parse_input(lines: impl Iterator<Item = String>) -> Vec<i32> {
    lines.into_iter()
        .flat_map(|s| {
            s.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()
        })
        .collect()
}

fn calc_fuel(positions: &Vec<i32>, d: i32) -> Vec<i32> {
    positions.iter().map(|p| (p - d ).abs()).collect()
}

fn part_1(lines: impl Iterator<Item = String>) -> i32 {
    let positions = parse_input(lines);
    (0..positions.len())
        .map(|d| calc_fuel(&positions, d as i32).iter().sum() )
        .min().unwrap()    
}

fn calc_fuel2(positions: &Vec<i32>, d: i32) -> Vec<i32> {
    positions.iter().map(|p| (p - d ).abs())
        .map(|n| n*(n+1)/2).collect()
}

fn part_2(lines: impl Iterator<Item = String>) -> i32 {
    let positions = parse_input(lines);
    (0..positions.len())
        .map(|d| calc_fuel2(&positions, d as i32).iter().sum() )
        .min().unwrap()
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

    static TEST_DATA: &str = r"16,1,2,0,4,2,7,1,2,14";

    fn test_data() -> impl Iterator<Item = String> {
        TEST_DATA.lines().map(String::from)
    }

    fn real_data() -> impl Iterator<Item = String> {
        iter_lines(DATA_FILE)
    }

    #[test]
    pub fn test_parse() {
        let positions = parse_input(test_data());
        assert_eq!(vec![16,1,2,0,4,2,7,1,2,14], positions);
    }

    #[test]
    pub fn test_part_1() {
        assert_eq!(37, part_1(test_data()));
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(168, part_2(test_data()));
    }

    #[test]
    pub fn test_part_1_real() {
        assert_eq!(347449, part_1(real_data()));
    }

    #[test]
    pub fn test_part_2_real() {
        assert_eq!(98039527, part_2(real_data()));
    }
}
