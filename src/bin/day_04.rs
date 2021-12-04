use std::{path::PathBuf, convert::TryInto};
use advent_of_code_2021::iter_lines;
use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    part_no: usize,

    #[structopt(parse(from_os_str), default_value = "data/day_04.txt")]
    filename: PathBuf,
}

type Board = [u32; 25];

fn parse_input(mut lines: impl Iterator<Item = String>) -> (Vec<u32>, Vec<Board>) {
    let numstr = lines.next().unwrap();
    let nums: Vec<u32> = numstr
        .trim().split(",")
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec::<Board> = vec![];
    for mut board_data in &lines.chunks(6) {
        // discard blank line
        board_data.next();
        let board_vals = board_data.take(5).map(|l|
            l.trim().split_whitespace().map(|v| v.parse().unwrap()).collect::<Vec<u32>>()
        ).flatten().collect::<Vec<u32>>();
        let board: Board = board_vals.try_into().expect("Invalid board");
        boards.push(board);
    }
    (nums, boards)
}

fn sum_unmarked(board: &Board, s: u32) -> u32 {
    (0..25).filter(|i| s & (1 << i) == 0)
        .map(|i| board[i])
        .sum()
}

fn play_number(n: u32, boards_played : &mut Vec<(Board, u32)>) -> Vec<(usize, u32)> {
    const RMASK: u32 = 0b11111;
    const CMASK: u32 = 1 | (1 << 5) | (1 << 10) | (1 << 15) | (1 << 20);

    // find if any square on the board matches
    let mut winners : Vec<(usize, u32)> = vec![];
    for (bn, (b, s)) in &mut boards_played.iter_mut().enumerate() {
        for i in 0..25 {
            if b[i] == n {
                // got a match, mark it off
                *s |= 1 << i;

                // and check if this row or col is full
                let r = i / 5;
                let rmask = RMASK << (r*5);
                let c = i % 5;
                let cmask = CMASK << c;
                if (*s & rmask) == rmask ||
                    (*s & cmask) == cmask {
                    // got a row or column
                    let unmarked_total = sum_unmarked(&b, *s);
                    winners.push((bn, unmarked_total * n));
                }
            }
        }
    }
    winners
}

fn part_1(lines: impl Iterator<Item = String>) -> u32 {
    let (nums, boards) = parse_input(lines);

    // boards are 25 elements, so can use a u32 bitmask to indiacte which numbers have been seen
    let mut boards_played = boards.into_iter()
        .map(|b| (b, 0u32))
        .collect::<Vec<([u32; 25], u32)>>();

    let (_, val) = nums.iter()
        .filter_map(|n| play_number(*n, &mut boards_played).into_iter().next())
        .next()
        .unwrap();
    val
}

fn part_2(lines: impl Iterator<Item = String>) -> u32 {
    let (nums, boards) = parse_input(lines);

    // boards are 25 elements, so can use a u32 bitmask to indiacte which numbers have been seen
    let mut boards_played = boards.into_iter()
        .map(|b| (b, 0u32))
        .collect::<Vec<([u32; 25], u32)>>();

    // keep playing, but remove winning boards from active set
    nums.iter()
        .filter_map(|n| {
            let mut last_winner = None;
            for (bn, val) in play_number(*n, &mut boards_played).into_iter().rev() {
                &mut boards_played.remove(bn);
                last_winner = Some(val)
            }
            last_winner
        })
        .last()
        .unwrap()
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

    static TEST_DATA: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7
";

    fn test_data() -> impl Iterator<Item = String> {
        TEST_DATA.lines().map(String::from)
    }

    #[test]
    pub fn test_parse() {
        let (nums, boards) = parse_input(test_data());
        assert_eq!(27, nums.len());
        assert_eq!(7, nums[0]);
        assert_eq!(Some(&1u32), nums.last());

        assert_eq!(3, boards.len());
        assert_eq!([22, 13, 17, 11, 0, 8, 2], &boards[0][0..7]);
        assert_eq!([22, 11, 13, 6, 5, 2, 0, 12, 3, 7], &boards[2][15..25]);
    }

    #[test]
    pub fn test_nums_parse() {
        let numstr = "23,30,70,61,79,49,19,37,64,48,72,34,69,53,15,74,89,38,46,36,28,32,45,2,39,58,11,62,97,40,14,87,96,94,91,92,80,99,6,31,57,98,65,10,33,63,42,17,47,66,26,22,73,27,7,0,55,8,56,29,86,25,4,12,51,60,35,50,5,75,95,44,16,93,21,3,24,52,77,76,43,41,9,84,67,71,83,88,59,68,85,82,1,18,13,78,20,90,81,54";
        let nums : Vec<u32> = numstr.split(",")
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();
        assert_eq!(100, nums.len());
    }

    #[test]
    pub fn test_part_1() {
        assert_eq!(4512, part_1(test_data()));
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(1924, part_2(test_data()));
    }
}
