use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct Position {
//     x: i64,
//     y: i64,
// }

#[derive(Debug, Clone, Hash)]
struct Input {
    values: Vec<i64>,
}

fn main() {
    let args = Args::parse();
    let data_file = if args.data_file.is_empty() {
        format!("{}/src/data.txt", env!("CARGO_MANIFEST_DIR"))
    } else {
        args.data_file
    };

    let input = parse(&data_file);

    let result1 = part1(&input);
    println!("Part1: {}", result1);

    println!("Part 2: {}", part2(&input))
}

fn part1(input: &Input) -> i64 {
    input
        .values
        .iter()
        .map(|v| {
            let mut current = *v;
            for _ in 0..2000 {
                current = (current ^ (current * 64)) % 16777216;
                current = (current ^ (current / 32)) % 16777216;
                current = (current ^ (current * 2048)) % 16777216;
            }

            current
        })
        .sum()
}

fn part2(input: &Input) -> i64 {
    let results = input
        .values
        .iter()
        .map(|v| {
            let mut current = *v;
            let mut history = VecDeque::new();
            history.push_back(current % 10);

            let mut results = HashMap::new();

            for _ in 0..2000 {
                current = (current ^ (current * 64)) % 16777216;
                current = (current ^ (current / 32)) % 16777216;
                current = (current ^ (current * 2048)) % 16777216;

                history.push_back(current % 10);
                if history.len() > 4 {
                    let mut price_changes = Vec::new();
                    for i in 1..history.len() {
                        price_changes.push(history[i] - history[0]);
                    }

                    let previous_result = results.get(&price_changes);
                    if previous_result.is_none() {
                        results.insert(price_changes, current % 10);
                    }
                    history.pop_front();
                }
            }

            results
        })
        .collect_vec();

    let mut totals = HashMap::new();
    for r in results {
        for k in r.keys() {
            let total = totals.entry(k.clone()).or_default();
            *total += *r.get(k).unwrap();
        }
    }

    println!("{:?}", *totals.iter().max_by_key(|(_, v)| **v).unwrap().0);
    *totals.iter().max_by_key(|(_, v)| **v).unwrap().1
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    Input {
        values: lines
            .iter()
            .map(|line| line.parse::<i64>().unwrap())
            .collect_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 37327623);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test2.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 23);
    }
}
