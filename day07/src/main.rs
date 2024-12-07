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

    let data = parse(&data_file);

    let result1 = part1(&data);
    println!("Part1: {}", result1);

    println!("Part 2: {}", part2(&data))
}

struct Input {
    result: u64,
    values: Vec<u64>,
}

fn part1(data: &Vec<Input>) -> u64 {
    solve(data, false)
}

fn solve(data: &Vec<Input>, include_concat: bool) -> u64 {
    data.iter()
        .filter_map(|row| {
            if is_possible(row.result, row.values[0], &row.values[1..], include_concat) {
                Some(row.result)
            } else {
                None
            }
        })
        .sum::<u64>()
}

fn is_possible(target: u64, value: u64, remaining: &[u64], include_concat: bool) -> bool {
    if remaining.len() == 0 {
        return value == target;
    }

    if value > target {
        return false;
    }

    return is_possible(
        target,
        value + remaining[0],
        &remaining[1..],
        include_concat,
    ) || is_possible(
        target,
        value * remaining[0],
        &remaining[1..],
        include_concat,
    ) || (include_concat
        && is_possible(
            target,
            (value.to_string() + &remaining[0].to_string())
                .parse::<u64>()
                .unwrap(),
            &remaining[1..],
            include_concat,
        ));
}

fn part2(data: &Vec<Input>) -> u64 {
    solve(data, true)
}

fn parse(file: &str) -> Vec<Input> {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    lines
        .iter()
        .map(|line| {
            let (result, remainder) = line.split_once(':').unwrap();
            let values: Vec<u64> = remainder
                .trim()
                .split_ascii_whitespace()
                .map(|value| value.parse::<u64>().expect("Failed to parse value"))
                .collect();
            Input {
                result: result.parse::<u64>().expect("Failed to parse result"),
                values,
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&data);

        assert_eq!(result1, 3749);
    }

    #[test]
    fn test_part2() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&data);

        assert_eq!(result2, 11387);
    }
}
