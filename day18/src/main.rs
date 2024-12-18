use clap::Parser;
use itertools::Itertools;
use std::{
    cmp::Reverse,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Hash)]
struct Input {
    bytes: Vec<Position>,
}

fn main() {
    let args = Args::parse();
    let data_file = if args.data_file.is_empty() {
        format!("{}/src/data.txt", env!("CARGO_MANIFEST_DIR"))
    } else {
        args.data_file
    };

    let input = parse(&data_file);

    let result1 = part1(&input, Position { x: 70, y: 70 }, 1024);
    println!("Part1: {}", result1);

    println!("Part 2: {}", part2(&input))
}

fn part1(input: &Input, bounds: Position, bytes_to_drop: i64) -> i64 {
    let mut to_process = BinaryHeap::new();
    to_process.push(Reverse((0, Position { x: 0, y: 0 })));
    let bytes = input
        .bytes
        .iter()
        .cloned()
        .take(bytes_to_drop as usize)
        .collect_vec();

    let mut visited = HashSet::new();
    //visited.insert(Position { x: 0, y: 0 });

    while !to_process.is_empty() {
        let Reverse((dist, pos)) = to_process.pop().expect("Failed to pop");

        if !visited.insert(pos) {
            continue;
        }

        if pos == bounds {
            return dist;
        }

        let left = Position {
            x: pos.x - 1,
            y: pos.y,
        };
        if pos.x > 0 && !bytes.contains(&left) {
            to_process.push(Reverse((dist + 1, left)));
        }

        let right = Position {
            x: pos.x + 1,
            y: pos.y,
        };
        if pos.x < bounds.x && !bytes.contains(&right) {
            to_process.push(Reverse((dist + 1, right)));
        }

        let up = Position {
            x: pos.x,
            y: pos.y - 1,
        };
        if pos.y > 0 && !bytes.contains(&up) {
            to_process.push(Reverse((dist + 1, up)));
        }

        let down = Position {
            x: pos.x,
            y: pos.y + 1,
        };
        if pos.y < bounds.y && !bytes.contains(&down) {
            to_process.push(Reverse((dist + 1, down)));
        }
    }

    0
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
        bytes: lines
            .iter()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                Position {
                    x: x.parse::<i64>().unwrap(),
                    y: y.parse::<i64>().unwrap(),
                }
            })
            .collect_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input, Position { x: 6, y: 6 }, 12);

        assert_eq!(result1, 22);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
