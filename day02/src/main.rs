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

    let result1 = part1(data.clone());
    println!("Part1: {}", result1);

    println!("Part 2: {}", part2(data))
}

fn part1(data: Vec<Vec<i64>>) -> usize {
    data.iter().filter(|report| is_safe(report)).count()
}

fn part2(data: Vec<Vec<i64>>) -> usize {
    data.iter()
        .filter(|report| {
            if is_safe(report) {
                return true;
            } else {
                for i in 0..report.len() {
                    let mut fixed_report = report[0..i].to_vec();
                    fixed_report.append(&mut report[(i + 1)..].to_vec());
                    if is_safe(&fixed_report) {
                        return true;
                    }
                }
                return false;
            }
        })
        .count()
}

fn is_safe(report: &Vec<i64>) -> bool {
    report.iter().tuple_windows().all(|(a, b)| {
        let diff = a - b;
        diff <= 3 && diff >= 1
    }) || report.iter().tuple_windows().all(|(a, b)| {
        let diff = b - a;
        diff <= 3 && diff >= 1
    })
}

fn parse(file: &str) -> Vec<Vec<i64>> {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(data);
        assert_eq!(result1, 2);
    }

    #[test]
    fn test_part2() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(data);

        assert_eq!(result2, 4);
    }
}
