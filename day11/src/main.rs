use clap::Parser;
use itertools::Itertools;
use std::{
    collections::HashMap,
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
    let mut stones = input.values.clone();

    for iteration in 0..25 {
        run_iteration(&mut stones);

        println!("{}: {}", iteration, stones.len());
    }

    stones.len() as i64
}

fn run_iteration(stones: &mut Vec<i64>) {
    let mut i = 0;
    while i < stones.len() {
        let value = stones[i];
        if value == 0 {
            stones[i] = 1;
            i += 1;
            continue;
        }

        let value_str = value.to_string();
        if value.to_string().len() % 2 == 0 {
            stones[i] = value_str[0..(value_str.len() / 2)].parse::<i64>().unwrap();
            stones.insert(
                i + 1,
                value_str[(value_str.len() / 2)..].parse::<i64>().unwrap(),
            );
            i += 1;
        } else {
            stones[i] = value * 2024;
        }
        i += 1;
    }
}

fn run_iteration_recursive(
    stone: i64,
    iterations: i64,
    known_results: &mut HashMap<(i64, i64), i64>,
) -> i64 {
    if iterations == 0 {
        return 1;
    }

    if let Some(result) = known_results.get(&(stone, iterations)) {
        return *result;
    }

    let mut result = 0;

    if stone == 0 {
        result = run_iteration_recursive(1, iterations - 1, known_results);
    } else {
        let value_str = stone.to_string();
        if value_str.len() % 2 == 0 {
            let left_stone = value_str[0..(value_str.len() / 2)].parse::<i64>().unwrap();
            let right_stone = value_str[(value_str.len() / 2)..].parse::<i64>().unwrap();
            result += run_iteration_recursive(left_stone, iterations - 1, known_results);
            result += run_iteration_recursive(right_stone, iterations - 1, known_results);
        } else {
            result = run_iteration_recursive(stone * 2024, iterations - 1, known_results);
        }
    }

    println!("{}, {} = {}", stone, iterations, result);

    known_results.insert((stone, iterations), result);
    result
}

fn part2(input: &Input) -> u128 {
    let mut cache = HashMap::new();
    input
        .values
        .iter()
        .map(|s| run_iteration_recursive(*s, 75, &mut cache))
        .sum::<i64>() as u128
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
            .get(0)
            .unwrap()
            .split_ascii_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
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

        assert_eq!(result1, 55312);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 65601038650482);
    }
}
