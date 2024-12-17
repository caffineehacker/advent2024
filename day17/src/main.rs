use clap::Parser;
use itertools::Itertools;
use log::trace;
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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct Position {
//     x: i64,
//     y: i64,
// }

#[derive(Debug, Clone, Hash)]
struct Input {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<i64>,
}

fn main() {
    env_logger::init();
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

fn part1(input: &Input) -> String {
    let mut ip = 0;
    let mut register_a = input.register_a;
    let mut register_b = input.register_b;
    let mut register_c = input.register_c;

    let mut out = Vec::new();

    while ip < input.program.len() {
        trace!(
            "ip: {}, a: {}, b: {}, c: {}",
            ip,
            register_a,
            register_b,
            register_c
        );
        let instruction = input.program[ip];
        ip += 1;
        match instruction {
            0 => {
                // adv
                if ip == input.program.len() {
                    break;
                }
                let operand = input.program[ip];
                trace!("adv {}", operand);
                ip += 1;
                register_a = register_a
                    / 2_i64.pow(
                        get_combo_operand_value(operand, register_a, register_b, register_c) as u32,
                    );
            }
            1 => {
                // bxl
                if ip == input.program.len() {
                    break;
                }
                let operand = input.program[ip];
                trace!("bxl {}", operand);
                ip += 1;
                register_b = register_b ^ operand;
            }
            2 => {
                // bst
                if ip == input.program.len() {
                    break;
                }
                let operand =
                    get_combo_operand_value(input.program[ip], register_a, register_b, register_c);
                trace!("bst {}", operand);
                ip += 1;
                register_b = operand % 8;
            }
            3 => {
                // jnz
                if register_a == 0 {
                    // TODO: Should we increase ip here?
                    trace!("jnz");
                    ip += 1;
                    continue;
                }

                let operand = input.program[ip];
                trace!("jnz {}", operand);
                ip = operand as usize;
            }
            4 => {
                // bxc
                trace!("bxc");
                register_b ^= register_c;

                // increases ip even though nothing is used.
                ip += 1;
            }
            5 => {
                // out
                if ip == input.program.len() {
                    break;
                }
                let operand = input.program[ip];
                ip += 1;
                trace!("out {}", operand);
                out.push(get_combo_operand_value(operand, register_a, register_b, register_c) % 8);
            }
            6 => {
                // bdv
                if ip == input.program.len() {
                    break;
                }
                let operand = input.program[ip];
                ip += 1;
                trace!("bdv {}", operand);
                register_b = register_a
                    / 2_i64.pow(
                        get_combo_operand_value(operand, register_a, register_b, register_c) as u32,
                    );
            }
            7 => {
                // cdv
                if ip == input.program.len() {
                    break;
                }
                let operand = input.program[ip];
                ip += 1;
                trace!("cdv {}", operand);
                register_c = register_a
                    / 2_i64.pow(
                        get_combo_operand_value(operand, register_a, register_b, register_c) as u32,
                    );
            }
            _ => panic!("Bad instruction"),
        };
    }

    out.iter().map(|v| v.to_string()).join(",").to_owned()
}

fn part2(input: &Input) -> i64 {
    0
}

fn get_combo_operand_value(operand: i64, a: i64, b: i64, c: i64) -> i64 {
    match operand {
        0..=3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Bad operand"),
    }
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let (a, b, c) = lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| l.split_once(": ").unwrap().1.parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();

    Input {
        register_a: a,
        register_b: b,
        register_c: c,
        program: lines
            .iter()
            .skip_while(|line| !line.is_empty())
            .filter(|line| !line.is_empty())
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(|page| page.parse::<i64>().unwrap())
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

        assert_eq!(result1, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_combo() {
        assert_eq!(get_combo_operand_value(0, 100, 200, 300), 0);
        assert_eq!(get_combo_operand_value(1, 100, 200, 300), 1);
        assert_eq!(get_combo_operand_value(2, 100, 200, 300), 2);
        assert_eq!(get_combo_operand_value(3, 100, 200, 300), 3);
        assert_eq!(get_combo_operand_value(4, 100, 200, 300), 100);
        assert_eq!(get_combo_operand_value(5, 100, 200, 300), 200);
        assert_eq!(get_combo_operand_value(6, 100, 200, 300), 300);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
