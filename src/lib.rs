use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn iter_lines(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let lines = read_lines(filename).expect("Can't read input");
    lines.into_iter().map(Result::unwrap)
}

pub fn from_split_lines<T, F, const N: usize>(
    lines: impl Iterator<Item = String>,
    f: F,
) -> impl Iterator<Item = T>
where
    T: Sized,
    F: 'static + Fn([String; N]) -> T,
{
    lines.map(move |l| {
        let s = l
            .trim()
            .splitn(N, " ")
            .map(String::from)
            .collect::<Vec<String>>();
        let args = <[String; N]>::try_from(s).ok().unwrap();
        let x: T = f(args);
        x
    })
}
