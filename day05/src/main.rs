use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
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

#[derive(Debug, Clone)]
struct Data {
    rules: Vec<(i64, i64)>,
    prints: Vec<Vec<i64>>,
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

fn part1(data: Data) -> i64 {
    data.prints
        .iter()
        .filter(|pages| is_valid_print(pages, &data.rules))
        .map(|pages| pages.get(pages.len() / 2).unwrap())
        .sum::<i64>()
}

fn is_valid_print(pages: &Vec<i64>, rules: &Vec<(i64, i64)>) -> bool {
    let page_numbers: HashSet<i64> = HashSet::from_iter(pages.iter().cloned());
    let relevant_rules = rules
        .iter()
        .filter(|(from, to)| page_numbers.contains(from) && page_numbers.contains(to))
        .collect_vec();

    relevant_rules.iter().all(|(first, second)| {
        pages
            .iter()
            .enumerate()
            .find(|(_, p)| **p == *first)
            .unwrap()
            .0
            < pages
                .iter()
                .enumerate()
                .find(|(_, p)| **p == *second)
                .unwrap()
                .0
    })
}

fn part2(data: Data) -> i64 {
    0
}

fn parse(file: &str) -> Data {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    Data {
        rules: lines
            .iter()
            .take_while(|line| !line.is_empty())
            .map(|line| line.split_once('|').unwrap())
            .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
            .collect_vec(),
        prints: lines
            .iter()
            .skip_while(|line| !line.is_empty())
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split(',')
                    .map(|page| page.parse::<i64>().unwrap())
                    .collect_vec()
            })
            .collect_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(data);

        assert_eq!(result1, 143);
    }

    #[test]
    fn test_part2() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(data);

        assert_eq!(result2, 0);
    }
}
