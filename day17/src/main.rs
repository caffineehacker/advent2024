use clap::Parser;
use itertools::Itertools;
use log::trace;
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
//     x: u64,
//     y: u64,
// }

#[derive(Debug, Clone, Hash)]
struct Input {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program: Vec<u64>,
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
                    / 2_u64.pow(
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
                    / 2_u64.pow(
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
                    / 2_u64.pow(
                        get_combo_operand_value(operand, register_a, register_b, register_c) as u32,
                    );
            }
            _ => panic!("Bad instruction"),
        };
    }

    out.iter().map(|v| v.to_string()).join(",").to_owned()
}

fn part2(input: &Input) -> u64 {
    // Pulling some logic from the part 1 trace output
    // bst (combo 4) -> B = A % 8
    // A = A, B = A % 8, C = 0
    // bxl 7 -> B = (A % 8) xor 7
    // A = A, B = (A % 8) xor 7, C = 0
    // cdv 5 (B) -> C = A >> (A % 8) xor 7
    // A = A, B = (A % 8) xor 7, C = A >> (A % 8) xor 7
    // adv 3 -> A = A >> 3
    // A = A >> 3, B = (A % 8) xor 7, C = A >> (A % 8) xor 7
    // bxc -> B = ((A % 8) xor 7) xor (A >> (A % 8) xor 7)
    // A = A >> 3, B = (A % 8) xor (A >> (A % 8)), C = A >> (A % 8) xor 7
    // bxl 7 -> B = (A % 8) xor (A >> (A % 8)) xor 7
    // A = A >> 3, B = (A % 8) xor (A >> (A % 8)) xor 7, C = A >> (A % 8) xor 7
    // out B % 8 = ((A % 8) xor (A >> (A % 8))) % 8

    // 2 = ((A % 8) xor (A >> (A % 8))) % 8
    // A is limited to 8 bits
    // if low 3 bits are 0 0 1, bits above 5 are 0 1 1

    // test with original number:
    // A = 52042868
    // ((A % 8) xor (A >> (A % 8))) % 8 = 2
    // ((52042868 % 8) xor (52042868 >> (52042868 % 8))) = 2
    // (4 xor (52042868 >> 4)) = 2
    // (4 xor 3252679) = 2

    // Work backwards
    // B = 6505354 = (A % 8) xor (A >> (A % 8))
    //

    let mut input = input.clone();
    let mut possibilities = HashSet::new();
    // for i in 0..8 {
    //     possibilities.insert((1, i));
    // }
    possibilities.insert((0, 0));

    while possibilities.len() > 0 {
        let entry = *possibilities.iter().min_by_key(|(_i, a)| *a).unwrap();
        let (num_count, a) = entry;
        possibilities.remove(&entry);

        for i in 0..8 {
            input.register_a = (a << 3) + i;

            let output = run_machine(&input);
            println!("{}: {:?}", input.register_a, output);
            if output == input.program {
                return input.register_a;
            }
            if output.len() > num_count
                && output.iter().take(num_count + 1).cloned().eq(input
                    .program
                    .iter()
                    .skip(input.program.len() - (num_count + 1))
                    .cloned())
            {
                possibilities.insert((num_count + 1, input.register_a));
                println!("#### {}: {}", num_count + 1, input.register_a);
            }
        }
    }

    /*
                    Possible value: 2 - length: 0
                    Possible value: 296 - length: 1
                    Possible value: 923 - length: 2
                    Possible value: 34715 - length: 3
                    Possible value: 730011 - length: 5
                    Possible value: 17507227 - length: 6
                    Possible value: 23798683 - length: 7
                    Possible value: 88810395 - length: 8

                                             100101000 = 296 (1)
                                           11100110110 = 923 (2)
                                      1000011110011011 = 34715 (3)
                                  10110010001110011011 = 730011 (5)
                             1000010110010001110011011 = 17507227 (6)
                             1011010110010001110011011 = 23798683 (7)
                           101010010110010001110011011 = 88810395 (8)
                        100101010010110010001110011011 = 625681307 (9)
                 1001110000101010010110010001110011011 = 83840672667 (10)
                11100000100101010010110010001110011011 = 241143849883 (11)
           1100011100000100101010010110010001110011011 = 6838213616539 (13)
    10000001100011100000100101010010110010001110011011 = 569788167037851 (15)
                 */

    0
}

fn run_machine(input: &Input) -> Vec<u64> {
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
                    / 2_u64.pow(
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
                if out.len() >= input.program.len() {
                    return out;
                }
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
                    / 2_u64.pow(
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
                    / 2_u64.pow(
                        get_combo_operand_value(operand, register_a, register_b, register_c) as u32,
                    );
            }
            _ => panic!("Bad instruction"),
        };
    }

    out
}

fn get_combo_operand_value(operand: u64, a: u64, b: u64, c: u64) -> u64 {
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
        .map(|l| l.split_once(": ").unwrap().1.parse::<u64>().unwrap())
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
            .map(|page| page.parse::<u64>().unwrap())
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

        assert_eq!(result2, 117440);
    }
}
