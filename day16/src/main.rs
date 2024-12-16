use clap::Parser;
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
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

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    walls: HashSet<Position>,
    start: Position,
    exit: Position,
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
    let mut visited_cells_and_direction = HashSet::new();

    // Holds (score, position, direction)
    let mut to_process = BinaryHeap::new();
    to_process.push(Reverse((0, input.start, Position { x: 1, y: 0 })));

    while let Some(entry) = to_process.pop() {
        let Reverse((score, position, direction)) = entry;

        if !visited_cells_and_direction.insert((position, direction)) {
            continue;
        }

        if position == input.exit {
            return score;
        }

        if !input.walls.contains(&(position + direction)) {
            to_process.push(Reverse((score + 1, position + direction, direction)));
        }

        if direction.x != 0 {
            to_process.push(Reverse((score + 1000, position, Position { x: 0, y: 1 })));
            to_process.push(Reverse((score + 1000, position, Position { x: 0, y: -1 })));
        } else {
            to_process.push(Reverse((score + 1000, position, Position { x: 1, y: 0 })));
            to_process.push(Reverse((score + 1000, position, Position { x: -1, y: 0 })));
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

    let walls: HashSet<Position> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Position {
                    x: x as i64,
                    y: y as i64,
                })
        })
        .collect();
    let start = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'S')
                .map(move |(x, _)| Position {
                    x: x as i64,
                    y: y as i64,
                })
        })
        .collect_vec()[0];
    let exit = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'E')
                .map(move |(x, _)| Position {
                    x: x as i64,
                    y: y as i64,
                })
        })
        .collect_vec()[0];

    Input { walls, start, exit }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 7036);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
