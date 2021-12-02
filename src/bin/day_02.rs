use std::path::PathBuf;

use advent_of_code_2021::{from_split_lines, iter_lines};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    part_no: usize,

    #[structopt(parse(from_os_str), default_value = "data/day_02.txt")]
    filename: PathBuf,
}

fn parse_input(lines: impl Iterator<Item = String>) -> impl Iterator<Item = (String, u32)> {
    from_split_lines(lines, |args: [String; 2]| {
        let cmd = args[0].clone();
        let val = args[1].parse::<u32>().unwrap();
        (cmd, val)
    })
}

fn part_1(lines: impl Iterator<Item = String>) -> u32 {
    let (final_p, final_d) =
        parse_input(lines).fold((0, 0), |(p, d), (cmd, x)| match cmd.as_str() {
            "forward" => (p + x, d),
            "up" => (p, d - x),
            "down" => (p, d + x),
            _ => panic!("Invalid command"),
        });
    final_p * final_d
}

fn part_2(lines: impl Iterator<Item = String>) -> u32 {
    let (final_p, final_d, _) = parse_input(lines).fold((0, 0, 0), |(p, d, a), (cmd, x)| {
        let (np, nd, na) = match cmd.as_str() {
            "forward" => (p + x, d + a * x, a),
            "up" => (p, d, a - x),
            "down" => (p, d, a + x),
            _ => panic!("Invalid command"),
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

    fn test_data() -> impl Iterator<Item = String> {
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
