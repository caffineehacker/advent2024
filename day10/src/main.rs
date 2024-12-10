use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
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

#[derive(Debug, Clone)]
struct Input {
    elevations: HashMap<Position, i64>,
    map_limits: Position,
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
    let trailheads = input
        .elevations
        .iter()
        .filter(|(_, e)| **e == 0)
        .map(|(p, _)| p)
        .collect_vec();

    trailheads
        .iter()
        .map(|start| {
            let mut to_visit = VecDeque::new();
            to_visit.push_back(**start);
            let mut terminals = HashSet::new();

            while !to_visit.is_empty() {
                let next = to_visit.pop_front().unwrap();
                let elevation = input.elevations.get(&next).unwrap();

                if *elevation == 9 {
                    terminals.insert(next);
                    continue;
                }

                if next.x > 0
                    && *input
                        .elevations
                        .get(&Position {
                            x: next.x - 1,
                            y: next.y,
                        })
                        .unwrap()
                        == *elevation + 1
                {
                    to_visit.push_back(Position {
                        x: next.x - 1,
                        y: next.y,
                    });
                }

                if next.y > 0
                    && *input
                        .elevations
                        .get(&Position {
                            x: next.x,
                            y: next.y - 1,
                        })
                        .unwrap()
                        == *elevation + 1
                {
                    to_visit.push_back(Position {
                        x: next.x,
                        y: next.y - 1,
                    });
                }

                if next.x < input.map_limits.x - 1
                    && *input
                        .elevations
                        .get(&Position {
                            x: next.x + 1,
                            y: next.y,
                        })
                        .unwrap()
                        == *elevation + 1
                {
                    to_visit.push_back(Position {
                        x: next.x + 1,
                        y: next.y,
                    });
                }

                if next.y < input.map_limits.y - 1
                    && *input
                        .elevations
                        .get(&Position {
                            x: next.x,
                            y: next.y + 1,
                        })
                        .unwrap()
                        == *elevation + 1
                {
                    to_visit.push_back(Position {
                        x: next.x,
                        y: next.y + 1,
                    });
                }
            }

            terminals.len() as i64
        })
        .sum::<i64>()
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

    // Creates a HashMap<char, Vec<Position>>
    let map_limits = Position {
        x: lines[0].len() as i64,
        y: lines.len() as i64,
    };

    Input {
        elevations: lines
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(|(x, c)| {
                        (
                            Position {
                                x: x as i64,
                                y: y as i64,
                            },
                            c.to_digit(10).unwrap() as i64,
                        )
                    })
                    .collect_vec()
            })
            .collect(),
        map_limits,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 36);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
