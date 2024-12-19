use cached::proc_macro::cached;
use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashSet},
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
    get_possible_patterns(input).len()
}

fn get_possible_patterns(input: &Input) -> Vec<Vec<char>> {
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
        .cloned()
        .collect_vec()
}

fn part2(input: &Input) -> usize {
    let source_towels = input
        .source_towels
        .iter()
        .cloned()
        .sorted()
        .enumerate()
        .collect_vec();
    let max_source_length = input.source_towels.iter().map(|t| t.len()).max().unwrap();
    println!("Max st length: {}", max_source_length);
    // let mut source_towel_map = HashMap::new();
    // source_towels.iter().for_each(|(index, st)| {
    //     source_towel_map.insert(st.clone(), index.clone());
    // });
    let source_towels: HashSet<String> = source_towels
        .iter()
        .map(|st| st.1.iter().cloned().join(""))
        .collect();

    get_possible_patterns(input)
        .iter()
        .map(|target| possibilities(&source_towels, max_source_length, target.iter().join("")))
        .sum()
}

#[cached(convert = r##"{ format!("{:?}", target) }"##, key = "String")]
fn possibilities(
    source_towels: &HashSet<String>,
    max_source_length: usize,
    target: String,
) -> usize {
    if target.len() == 0 {
        return 1;
    }

    (1..=(max_source_length.min(target.len())))
        .map(|i| {
            if source_towels.contains(&target[..i]) {
                possibilities(source_towels, max_source_length, target[i..].to_string())
            } else {
                0
            }
        })
        .sum()
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

        assert_eq!(result2, 16);
    }
}
