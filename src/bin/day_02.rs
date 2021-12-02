
use std::path::{Path, PathBuf};

use advent_of_code_2021::read_lines;
use structopt::StructOpt;
use std::convert::TryFrom;

#[derive(StructOpt)]
struct Opts {
    part_no: usize,

    #[structopt(parse(from_os_str), default_value = "data/day_02.txt")]
    filename: PathBuf,
}

fn iter_lines(filename: impl AsRef<Path>) -> impl Iterator<Item=String> {
    let lines = read_lines(filename).expect("Can't read input");
    lines
        .into_iter()
        .map(Result::unwrap)
}

fn part_1(lines: impl Iterator<Item=String>) -> u32 {
    let (final_p, final_d) = lines.map(|l| {
        let v = l.splitn(2, " ").collect::<Vec<&str>>();
        let [cmd, x] = <[&str; 2]>::try_from(v).ok().unwrap();
        (String::from(cmd), x.parse::<u32>().unwrap())
    })
    .fold((0, 0), |(p, d), (cmd, x)| {
        match cmd.as_str() {
            "forward" => (p + x, d),
            "up" => (p, d - x),
            "down" => (p, d + x),
            _ => panic!("Invalid command")
        }
    });
    final_p * final_d
}

fn part_2(lines: impl Iterator<Item=String>) -> u32 {
    let (final_p, final_d, _) = lines.map(|l| {
        let v = l.splitn(2, " ").collect::<Vec<&str>>();
        let [cmd, val] = <[&str; 2]>::try_from(v).ok().unwrap();
        (String::from(cmd), val.parse::<u32>().unwrap())
    })
    .fold((0, 0, 0), |(p, d, a), (cmd, x)| {
        let (np, nd, na) = match cmd.as_str() {
            "forward" => (p + x, d + a*x, a),
            "up" => (p, d, a - x),
            "down" => (p, d, a + x),
            _ => panic!("Invalid command")
        };
        (np, nd, na)
    });
    final_p * final_d
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

    static TEST_DATA: [&str; 6] = [
    "forward 5",
    "down 5",
    "forward 8",
    "up 3",
    "down 8",
    "forward 2",
    ];

    fn test_data() -> impl Iterator<Item=String> {
        TEST_DATA.iter().map(|p| String::from(*p))
    }

    #[test]
    pub fn test_part_1() {
        assert_eq!(150, part_1(test_data()));
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(900, part_2(test_data()));
    }
}