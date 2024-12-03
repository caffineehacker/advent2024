use clap::Parser;
use itertools::Itertools;
use regex::Regex;
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

    let data = parse(&data_file);

    let result1 = part1(data.clone());
    println!("Part1: {}", result1);

    println!("Part 2: {}", part2(data))
}

fn part1(data: Vec<char>) -> i64 {
    let re = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let mut sum = 0;
    for (_, [num1, num2]) in re
        .captures_iter(&data.into_iter().join(""))
        .map(|c| c.extract())
    {
        let num1 = num1.parse::<i64>().unwrap();
        let num2 = num2.parse::<i64>().unwrap();
        sum += num1 * num2;
    }

    sum
}

fn part2(data: Vec<char>) -> i64 {
    0
}

fn parse(file: &str) -> Vec<char> {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .flat_map(|line| line.expect("Failed to read line").chars().collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(data);

        assert_eq!(result1, 161);
    }

    #[test]
    fn test_part2() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(data);

        assert_eq!(result2, 0);
    }
}
