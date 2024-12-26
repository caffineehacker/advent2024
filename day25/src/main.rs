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

#[derive(Debug, Clone, Hash)]
struct Input {
    keys: Vec<[i64; 5]>,
    locks: Vec<[i64; 5]>,
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
}

fn part1(input: &Input) -> i64 {
    println!("Keys: {:?}", input.keys);
    println!("Locks: {:?}", input.locks);
    let mut matches = 0;
    for k in &input.keys {
        for l in &input.locks {
            let mut is_match = true;
            for i in 0..5 {
                is_match &= k[i] + l[i] <= 5;
            }

            if is_match {
                println!("{:?}, {:?}", k, l);
                matches += 1;
            }
        }
    }

    matches
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let keys_and_locks = lines
        .iter()
        .chunks(8)
        .into_iter()
        .map(|c| c.filter(|l| !l.is_empty()).collect_vec())
        .collect_vec();

    let keys = keys_and_locks
        .iter()
        .filter(|l| l[0].chars().next().unwrap() == '.')
        .map(|key| {
            [
                key_height(key, 0),
                key_height(key, 1),
                key_height(key, 2),
                key_height(key, 3),
                key_height(key, 4),
            ]
        })
        .collect_vec();

    let locks = keys_and_locks
        .iter()
        .filter(|l| l[0].chars().next().unwrap() != '.')
        .map(|key| {
            [
                lock_height(key, 0),
                lock_height(key, 1),
                lock_height(key, 2),
                lock_height(key, 3),
                lock_height(key, 4),
            ]
        })
        .collect_vec();

    Input { keys, locks }
}

fn key_height(key: &Vec<&String>, index: usize) -> i64 {
    6 - (key
        .iter()
        .find_position(|l| l.chars().nth(index).unwrap() == '#')
        .unwrap()
        .0 as i64)
}

fn lock_height(lock: &Vec<&String>, index: usize) -> i64 {
    6 - (lock
        .iter()
        .rev()
        .find_position(|l| l.chars().nth(index).unwrap() == '#')
        .unwrap()
        .0 as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 3);
    }
}
