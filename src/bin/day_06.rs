use std::{path::{PathBuf, Iter}, ops::Range};
use advent_of_code_2021::iter_lines;
use structopt::StructOpt;

const DATA_FILE: &str = "data/day_06.txt";

#[derive(StructOpt)]
struct Opts {
    part_no: usize,

    #[structopt(parse(from_os_str), default_value = DATA_FILE)]
    filename: PathBuf,
}

fn parse_input(lines: impl Iterator<Item = String>) -> impl Iterator<Item=u32> {
    lines.into_iter()
        .flat_map(|s| {
            s.trim().split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()
        })
}

fn read_age_counts(ages: impl Iterator<Item = u32>) -> [u64; 9] {
    let mut age_counts = [0u64; 9];
    for i in ages {
        age_counts[i as usize] += 1;
    }
    age_counts
}

fn simulate(mut age_counts: [u64; 9], steps: usize) -> u64 {
    for d in 0..steps {
        let n_zero = age_counts[0];
        for i in 0..8 {
            age_counts[i] = age_counts[i+1];
        }
        age_counts[8] = n_zero;
        age_counts[6] += n_zero;
    }
    age_counts.iter().sum()
}

fn part_1(lines: impl Iterator<Item = String>) -> u64 {
    let age_counts = read_age_counts(parse_input(lines));
    simulate(age_counts, 80)
}

fn part_2(lines: impl Iterator<Item = String>) -> u64 {
    let age_counts = read_age_counts(parse_input(lines));
    simulate(age_counts, 256)
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

    static TEST_DATA: &str = r"3,4,3,1,2";

    fn test_data() -> impl Iterator<Item = String> {
        TEST_DATA.lines().map(String::from)
    }

    fn real_data() -> impl Iterator<Item = String> {
        iter_lines(DATA_FILE)
    }

    #[test]
    pub fn test_parse() {
        let ages = parse_input(test_data()).collect::<Vec<_>>();
        assert_eq!(5, ages.len());
        assert_eq!(vec![3, 4, 3, 1, 2], ages);
    }

    #[test]
    pub fn test_part_1_18days() {
        let ages = parse_input(test_data());
        let age_counts = read_age_counts(ages);
        assert_eq!(26, simulate(age_counts, 18));
    }

    #[test]
    pub fn test_part_1() {
        assert_eq!(5934, part_1(test_data()));
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(26984457539u64, part_2(test_data()));
    }

    #[test]
    pub fn test_part_1_real() {
        assert_eq!(6007, part_1(real_data()));
    }

    #[test]
    pub fn test_part_2_real() {
        assert_eq!(1631629590423, part_2(real_data()));
    }
}
