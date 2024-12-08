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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Input {
    antennas: HashMap<char, Vec<Position>>,
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
    input
        .antennas
        .iter()
        .map(|(_, positions)| find_antinodes(positions, &input.map_limits))
        .reduce(|acc, pos| acc.union(&pos).cloned().collect())
        .unwrap()
        .len() as i64
}

fn find_antinodes(positions: &Vec<Position>, map_limits: &Position) -> HashSet<Position> {
    let mut antinode_positions = HashSet::new();
    for i in 0..(positions.len() - 1) {
        for j in (i + 1)..positions.len() {
            let diff_x = positions[i].x - positions[j].x;
            let diff_y = positions[i].y - positions[j].y;

            let antinode1 = Position {
                x: positions[i].x + diff_x,
                y: positions[i].y + diff_y,
            };

            let antinode2 = Position {
                x: positions[j].x - diff_x,
                y: positions[j].y - diff_y,
            };

            if antinode1.x >= 0
                && antinode1.x < map_limits.x
                && antinode1.y >= 0
                && antinode1.y < map_limits.y
            {
                antinode_positions.insert(antinode1);
            }

            if antinode2.x >= 0
                && antinode2.x < map_limits.x
                && antinode2.y >= 0
                && antinode2.y < map_limits.y
            {
                antinode_positions.insert(antinode2);
            }
        }
    }

    antinode_positions
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

    let map_limits = Position {
        x: lines[0].len() as i64,
        y: lines.len() as i64,
    };

    Input {
        antennas: lines
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(|(x, c)| {
                        (
                            c,
                            Position {
                                x: x as i64,
                                y: y as i64,
                            },
                        )
                    })
                    .collect_vec()
            })
            .sorted_by(|(a, _), (b, _)| Ord::cmp(a, b))
            .chunk_by(|(c, _)| *c)
            .into_iter()
            .map(|(c, positions)| (c, positions.map(|(_, p)| p).collect_vec()))
            .collect(),
        map_limits,
    }

    /*
     * Alternative implementations:
     */

    // Two sections separated by a newline
    // Input {
    //     first: lines
    //         .iter()
    //         .take_while(|line| !line.is_empty())
    //         .map(|line| line.split_once('|').unwrap())
    //         .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
    //         .collect_vec(),
    //     second: lines
    //         .iter()
    //         .skip_while(|line| !line.is_empty())
    //         .filter(|line| !line.is_empty())
    //         .map(|line| {
    //             line.split(',')
    //                 .map(|page| page.parse::<i64>().unwrap())
    //                 .collect_vec()
    //         })
    //         .collect_vec(),
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 14);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
