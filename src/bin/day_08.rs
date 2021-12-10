use std::{path::PathBuf, collections::HashSet};
use advent_of_code_2021::iter_lines;
use structopt::StructOpt;

const DATA_FILE: &str = "data/day_08.txt";

#[derive(StructOpt)]
struct Opts {
    part_no: usize,

    #[structopt(parse(from_os_str), default_value = DATA_FILE)]
    filename: PathBuf,
}

fn parse_input(lines: impl Iterator<Item = String>) -> impl Iterator<Item = (Vec<String>, Vec<String>)> {
    lines.into_iter()
        .map(|s| {
            let (l, r) = s.trim().split_once(" | ").unwrap();
            let split_parts = |x: &str| x.trim().split(" ").map(|s| s.into()).collect::<Vec<String>>();
            (split_parts(l), split_parts(r))
        })
}

fn part_1(lines: impl Iterator<Item = String>) -> usize {
    // want digits 1 (2 segs), 4 (4 segs), 7 (3 segs), 8 (7 segs)
    let wanted = vec![2 as usize, 4, 3, 7];
    parse_input(lines)
        .map(|(_, r)| r)
        .map(|r| r.iter().filter(|x| wanted.contains(&x.len())).count())
        .sum()
}

// lit segments: numbers
//  2: 1
//  3: 7
//  4: 4
//  5: 2 3 5
//  6: 0 6 9
//  7: 8

fn in_common(a: &str, b: &str) -> usize {
    a.chars()
        .filter(|c| b.contains(*c))
        .count()
}

fn remove_if(v: &mut HashSet<String>, p: impl Fn(&&String) -> bool) -> String {
    let val = v.iter().find(p).unwrap();
    let res = val.clone();
    v.remove(&res);
    res
}

fn determine_segment_map(segs: &Vec<String>) -> Vec<String> {
    let mut segs = segs.iter()
        .map(String::clone)
        .collect::<HashSet<_>>();
    
    // 1 4 7 8 can work out by length
    let n1 = remove_if(&mut segs, |x| x.len() == 2);
    let n4 = remove_if(&mut segs, |x| x.len() == 4);
    let n7 = remove_if(&mut segs, |x| x.len() == 3);
    let n8 = remove_if(&mut segs, |x| x.len() == 7);

    // 3 has 5 segs and 2 of those are in 1
    let n3 = remove_if(&mut segs, |x| x.len() == 5 && in_common(x, &n1) == 2);
    // 2 has 5 segs, 2 in common with 4
    let n2 = remove_if(&mut segs, |x| x.len() == 5 && in_common(x, &n4) == 2);
    // 5 has 5 segs, 3 in common with 4
    let n5 = remove_if(&mut segs, |x| x.len() == 5 && in_common(x, &n4) == 3);
    // 9 has 6 segs, 5 in common with 3
    let n9 = remove_if(&mut segs, |x| x.len() == 6 && in_common(x, &n3) == 5);
    // 0 has 6 segs, 3 in common with 7
    let n0 = remove_if(&mut segs, |x| x.len() == 6 && in_common(x, &n7) == 3);
    // 6 is left
    let n6 = segs.iter().next().unwrap().clone();

    vec![n0, n1, n2, n3, n4, n5, n6, n7, n8, n9]
}

fn sort_segments(x: &str) -> String {
    let mut o = x.chars().collect::<Vec<_>>();
    o.sort();
    o.into_iter().collect::<String>()
}

fn process_input_line(l: &Vec<String>, r: &Vec<String>) -> i32 {
    let l_sorted = l.iter()
        .map(|x| sort_segments(x))
        .collect::<Vec<_>>();
    let r_sorted = r.iter()
        .map(|x| sort_segments(x))
        .collect::<Vec<_>>();
    let mut joined = l_sorted.clone();
    joined.append(&mut r_sorted.clone());

    let seg_map = determine_segment_map(&joined);

    r_sorted.iter()
        .map(|x|
            seg_map.iter().position(|i| i == x).unwrap() as i32
    )
    .fold(0, |v, i| v*10 + i)
}

fn part_2(lines: impl Iterator<Item = String>) -> i32 {
    parse_input(lines)
        .map(|(l, r)| process_input_line(&l, &r))
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

    static TEST_DATA: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    fn test_data() -> impl Iterator<Item = String> {
        TEST_DATA.lines().map(String::from)
    }

    fn real_data() -> impl Iterator<Item = String> {
        iter_lines(DATA_FILE)
    }

    #[test]
    pub fn test_parse() {
        let data = parse_input(test_data()).collect::<Vec<_>>();
        assert_eq!(10, data.len());
        assert_eq!("be", data[0].0[0]);
        assert_eq!("fgae", data[9].1[0]);
    }

    #[test]
    pub fn test_part_1() {
        assert_eq!(26, part_1(test_data()));
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(61229, part_2(test_data()));
    }

    #[test]
    pub fn test_part_1_real() {
        assert_eq!(237, part_1(real_data()));
    }

    #[test]
    pub fn test_part_2_real() {
        assert_eq!(1009098, part_2(real_data()));
    }
}
