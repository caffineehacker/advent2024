use clap::Parser;
use itertools::Itertools;
use multimap::MultiMap;
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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct Position {
//     x: i64,
//     y: i64,
// }

#[derive(Debug, Clone)]
struct Input {
    computers: MultiMap<String, String>,
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
    println!("{:?}", input);
    let mut triples = HashSet::new();
    for k in input.computers.keys() {
        let connected = input.computers.get_vec(k).unwrap();
        if connected.len() < 2 {
            continue;
        }

        println!("{} -> {:?}", k, connected);

        connected
            .iter()
            .combinations(2)
            .filter_map(|combos| {
                let a = combos[0];
                let b = combos[1];
                let a_connected = input.computers.get_vec(a).unwrap();
                let b_connected = input.computers.get_vec(b).unwrap();

                if a_connected.contains(b) && b_connected.contains(a) {
                    Some(vec![k, a, b].into_iter().cloned().sorted().collect_vec())
                } else {
                    None
                }
            })
            .for_each(|trip| {
                triples.insert(trip);
            });
    }

    println!("Initial count: {}", triples.len());

    triples
        .iter()
        .filter(|trip| trip.iter().any(|c| c.starts_with("t")))
        .count()
}

fn part2(input: &Input) -> String {
    let mut best_group = vec![];
    for k in input.computers.keys() {
        let connected = input.computers.get_vec(k).unwrap();

        println!("{} -> {:?}", k, connected);

        let mut i = connected.len();
        while i >= best_group.len() {
            if let Some(combo) = connected.iter().combinations(i).find(|combos| {
                println!("Testing {}, {:?}", k, combos);
                combos.iter().all(|c| {
                    let c_connected = input.computers.get_vec(*c).unwrap();
                    combos
                        .iter()
                        .all(|c2| c_connected.contains(*c2) || **c == **c2)
                })
            }) {
                let mut group = combo.clone();
                group.push(k);
                group.sort();
                println!("New best: {:?}", group);
                best_group = group;
            }

            i -= 1;
        }
    }

    println!("{:?}", best_group);

    best_group.into_iter().join(",")
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    Input {
        computers: lines
            .iter()
            .flat_map(|line| {
                let (left, right) = line.split_once('-').unwrap();
                vec![
                    (left.to_string(), right.to_string()),
                    (right.to_string(), left.to_string()),
                ]
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 7);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, "co,de,ka,ta");
    }
}
