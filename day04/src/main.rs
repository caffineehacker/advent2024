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

fn part1(data: Vec<Vec<char>>) -> i64 {
    let mut count = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            count += count_matches(&data, i, j);
        }
    }

    count
}

fn count_matches(data: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
    if data[i][j] != 'X' {
        return 0;
    }

    let mut count = 0;

    if i >= 3 {
        if data[i - 1][j] == 'M' && data[i - 2][j] == 'A' && data[i - 3][j] == 'S' {
            count += 1;
        }
        if j >= 3
            && data[i - 1][j - 1] == 'M'
            && data[i - 2][j - 2] == 'A'
            && data[i - 3][j - 3] == 'S'
        {
            count += 1;
        }
        if j < data[i].len() - 3
            && data[i - 1][j + 1] == 'M'
            && data[i - 2][j + 2] == 'A'
            && data[i - 3][j + 3] == 'S'
        {
            count += 1;
        }
    }
    if i < data.len() - 3 {
        if data[i + 1][j] == 'M' && data[i + 2][j] == 'A' && data[i + 3][j] == 'S' {
            count += 1;
        }
        if j >= 3
            && data[i + 1][j - 1] == 'M'
            && data[i + 2][j - 2] == 'A'
            && data[i + 3][j - 3] == 'S'
        {
            count += 1;
        }
        if j < data[i].len() - 3
            && data[i + 1][j + 1] == 'M'
            && data[i + 2][j + 2] == 'A'
            && data[i + 3][j + 3] == 'S'
        {
            count += 1;
        }
    }
    if j >= 3 {
        if data[i][j - 1] == 'M' && data[i][j - 2] == 'A' && data[i][j - 3] == 'S' {
            count += 1;
        }
    }

    if j < data[i].len() - 3 {
        if data[i][j + 1] == 'M' && data[i][j + 2] == 'A' && data[i][j + 3] == 'S' {
            count += 1;
        }
    }

    return count;
}

fn part2(data: Vec<Vec<char>>) -> i64 {
    0
}

fn parse(file: &str) -> Vec<Vec<char>> {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(data);

        assert_eq!(result1, 18);
    }

    #[test]
    fn test_part2() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(data);

        assert_eq!(result2, 0);
    }
}
