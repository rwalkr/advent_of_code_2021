use std::path::PathBuf;
use advent_of_code_2021::{from_split_lines, iter_lines};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    part_no: usize,

    #[structopt(parse(from_os_str), default_value = "data/day_03.txt")]
    filename: PathBuf,
}

fn parse_input(lines: impl Iterator<Item = String>) -> impl Iterator<Item = u32> {
    from_split_lines(lines, |args: [String; 1]| u32::from_str_radix(&args[0], 2).unwrap())
}

fn part_1(lines: impl Iterator<Item = String>) -> u32 {
    let mut lines = lines.peekable();
    let l1 = lines.peek().unwrap();
    let bits = l1.trim().len();
    let inputs: Vec<u32> = parse_input(lines).collect();
    let mut one_counts = Vec::<usize>::new();
    one_counts.resize(bits, 0);
    for i in inputs.iter() {
        for b in 0..bits {
            if (i & (1 << b)) != 0 {
                one_counts[b] += 1;
            }
        }
    }
    let gamma = one_counts.iter().enumerate()
        .fold(0, |v, (i, c)| {
            v | ((1 << i) * ((*c >= inputs.len()/2) as u32))
        });
    let epsilon = gamma ^ ((1 << bits)-1);
    gamma * epsilon
}

fn count_ones(values: &Vec<u32>, p: usize) -> usize {
    let m = 1 << p;
    values.iter().filter(|v| **v & m != 0).count()
}

fn is_one_most_common(values: &Vec<u32>, p: usize) -> bool {
    let ones = count_ones(values, p);
    let zeros = values.len() - ones;
    ones >= zeros
}

fn part_2(lines: impl Iterator<Item = String>) -> u32 {
    let mut lines = lines.peekable();
    let l1 = lines.peek().unwrap();
    let bits = l1.trim().len();
    let inputs: Vec<u32> = parse_input(lines).collect();
    let mut oxy_inputs = inputs.clone();
    for b in (0..bits).rev() {
        if oxy_inputs.len() == 1 {
            break;
        }
        let c = is_one_most_common(&oxy_inputs, b) as u32;
        oxy_inputs = oxy_inputs.into_iter().filter(|v| *v & (1 << b) == (c << b)).collect();
    }
    let oxy = oxy_inputs[0];
    let mut co2_inputs = inputs.clone();
    for b in (0..bits).rev() {
        if co2_inputs.len() == 1 {
            break;
        }
        let c = is_one_most_common(&co2_inputs, b) as u32 ^ 1;
        co2_inputs = co2_inputs.into_iter().filter(|v| *v & (1 << b) == (c << b)).collect();
    }
    let co2 = co2_inputs[0];
    println!("{:b} {} / {}, {:b} {} / {}", oxy, oxy, oxy_inputs.len(), co2, co2, co2_inputs.len());
    oxy * co2
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

    static TEST_DATA: &str = r"00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    fn test_data() -> impl Iterator<Item = String> {
        TEST_DATA.lines().map(String::from)
    }

    #[test]
    pub fn test_part_1() {
        assert_eq!(198, part_1(test_data()));
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(230, part_2(test_data()));
    }
}
