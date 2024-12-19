use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, VecDeque},
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
    source_towels: Vec<Vec<char>>,
    target_towels: Vec<Vec<char>>,
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

fn part1(input: &Input) -> usize {
    input
        .target_towels
        .iter()
        .filter(|target| {
            let mut possibilities = BinaryHeap::new();
            possibilities.push(0);

            while !possibilities.is_empty() {
                let possibility = possibilities.pop().unwrap();

                if possibility == target.len() {
                    return true;
                }

                let remaining = &target[possibility..];

                input
                    .source_towels
                    .iter()
                    .filter(|st| remaining.starts_with(*st))
                    .map(|st| st.len() + possibility)
                    .sorted()
                    .dedup()
                    .for_each(|p| possibilities.push(p));
            }

            false
        })
        .count()
}

fn part2(input: &Input) -> i64 {
    0
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    Input {
        source_towels: lines
            .get(0)
            .unwrap()
            .split_ascii_whitespace()
            .map(|t| t.trim_end_matches(',').chars().collect_vec())
            .sorted()
            .collect_vec(),
        target_towels: lines
            .iter()
            .skip(2)
            .map(|t| t.chars().collect_vec())
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

        assert_eq!(result1, 6);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
