use clap::Parser;
use itertools::Itertools;
use std::{
    collections::HashSet,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Data {
    guard: Position,
    stones: HashSet<Position>,
    max_position: Position,
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
    let mut guard_pos = data.guard;
    let mut guard_velocity = Position { x: 0, y: -1 };
    let mut visited = HashSet::new();

    while guard_pos.y >= 0
        && guard_pos.y <= data.max_position.y
        && guard_pos.x >= 0
        && guard_pos.x <= data.max_position.x
    {
        visited.insert(guard_pos);
        let mut new_guard_velocity = guard_velocity;

        let mut new_guard_position = Position {
            x: guard_pos.x + new_guard_velocity.x,
            y: guard_pos.y + new_guard_velocity.y,
        };
        if data.stones.contains(&new_guard_position) {
            new_guard_velocity = Position {
                x: -guard_velocity.y,
                y: guard_velocity.x,
            };
            new_guard_position = Position {
                x: guard_pos.x + new_guard_velocity.x,
                y: guard_pos.y + new_guard_velocity.y,
            };
        }

        guard_velocity = new_guard_velocity;
        guard_pos = new_guard_position;
    }

    visited.len() as i64
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
        stones: lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if c == '#' {
                            Some(Position {
                                x: x as i64,
                                y: y as i64,
                            })
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
            .collect(),
        guard: lines
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .find(|(_, c)| *c == '^')
                    .map(|(x, _)| Position {
                        x: x as i64,
                        y: y as i64,
                    })
            })
            .unwrap(),
        max_position: Position {
            x: lines.get(0).unwrap().len() as i64 - 1,
            y: lines.len() as i64 - 1,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(data);

        assert_eq!(result1, 41);
    }

    #[test]
    fn test_part2() {
        let data = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(data);

        assert_eq!(result2, 0);
    }
}
