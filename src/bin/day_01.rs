use std::path::PathBuf;

use advent_of_code_2021::read_lines;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts{
    #[structopt(parse(from_os_str), default_value="data/day_01_a.txt")]
    filename: PathBuf,
}

fn main() {
    let opts = Opts::from_args();
    let lines = read_lines(opts.filename).expect("Can't read input");
    let depths: Vec<u32> = lines.into_iter().filter_map(|l| l.unwrap().parse().ok()).collect();
    let deltas = depths.windows(2).filter(|vals| vals[1] > vals[0]).count();
    println!("{}", deltas);
}
