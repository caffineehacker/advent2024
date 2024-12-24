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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct Position {
//     x: i64,
//     y: i64,
// }

#[derive(Debug, Clone, Hash)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone, Hash)]
struct Operation {
    left: String,
    right: String,
    target: String,
    gate: Gate,
}

#[derive(Debug, Clone)]
struct Input {
    values: HashMap<String, i64>,
    operations: Vec<Operation>,
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
    let mut values = input.values.clone();

    let mut all_zs = HashSet::new();
    for (k, _v) in &values {
        if k.starts_with("z") {
            all_zs.insert(k.clone());
        }
    }

    for op in &input.operations {
        if op.target.starts_with("z") {
            all_zs.insert(op.target.clone());
        }
    }

    loop {
        for i in 0..input.operations.len() {
            let operation = &input.operations[i];

            if values.contains_key(&operation.left) && values.contains_key(&operation.right) {
                values.insert(
                    operation.target.clone(),
                    match operation.gate {
                        Gate::OR => values[&operation.left] | values[&operation.right],
                        Gate::XOR => values[&operation.left] ^ values[&operation.right],
                        Gate::AND => values[&operation.left] & values[&operation.right],
                    },
                );
            }
        }

        let mut all_fulfilled = true;
        let mut result = 0;
        for k in all_zs.iter().sorted().rev() {
            if !values.contains_key(k) {
                all_fulfilled = false;
                break;
            } else {
                result = (result << 1) + values[k];
            }
        }

        if all_fulfilled {
            return result;
        }
    }
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

    let values = lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (gate, value) = l.split_once(": ").unwrap();
            (gate.to_string(), value.parse::<i64>().unwrap())
        })
        .collect::<HashMap<String, i64>>();

    let operations = lines
        .iter()
        .skip_while(|l| !l.is_empty())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (left, gate, right, _arrow, target) = l.split_whitespace().collect_tuple().unwrap();
            Operation {
                left: left.to_string(),
                right: right.to_string(),
                target: target.to_string(),
                gate: match gate {
                    "OR" => Gate::OR,
                    "XOR" => Gate::XOR,
                    "AND" => Gate::AND,
                    _ => panic!("Unknown gate: {}", gate),
                },
            }
        })
        .collect_vec();

    Input { values, operations }

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

    // Creates a HashMap<char, Vec<Position>>
    // let map_limits = Position {
    //     x: lines[0].len() as i64,
    //     y: lines.len() as i64,
    // };

    // Input {
    //     antennas: lines
    //         .into_iter()
    //         .enumerate()
    //         .flat_map(|(y, line)| {
    //             line.chars()
    //                 .enumerate()
    //                 .filter(|(_, c)| *c != '.')
    //                 .map(|(x, c)| {
    //                     (
    //                         c,
    //                         Position {
    //                             x: x as i64,
    //                             y: y as i64,
    //                         },
    //                     )
    //                 })
    //                 .collect_vec()
    //         })
    //         .sorted_by(|(a, _), (b, _)| Ord::cmp(a, b))
    //         .chunk_by(|(c, _)| *c)
    //         .into_iter()
    //         .map(|(c, positions)| (c, positions.map(|(_, p)| p).collect_vec()))
    //         .collect(),
    //     map_limits,
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 4);
    }

    #[test]
    fn test_part1_2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test2.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 2024);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
