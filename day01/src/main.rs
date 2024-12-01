use clap::Parser;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "")]
    data_file: String,
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    let data_file = if args.data_file.is_empty() {
        format!("{}/src/data.txt", env!("CARGO_MANIFEST_DIR"))
    } else {
        args.data_file
    };

    let (left, right) = parse(&data_file);

    let result1 = part1(left.clone(), right.clone());
    println!("Part1: {}", result1);
}

fn part1(mut left: Vec<i64>, mut right: Vec<i64>) -> u64 {
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u64>()
}

fn parse(file: &str) -> (Vec<i64>, Vec<i64>) {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    lines
        .iter()
        .map(|line| {
            let (a, b) = line.split_ascii_whitespace().collect_tuple().unwrap();
            (
                a.parse::<i64>().expect("Failed to parse"),
                b.parse::<i64>().expect("Failed to parse"),
            )
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (left, right) = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(left, right);
        assert_eq!(result1, 11);
    }
}
