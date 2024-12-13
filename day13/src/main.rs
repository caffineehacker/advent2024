use clap::Parser;
use itertools::Itertools;
use std::ops::{Add, Mul};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use z3::ast::Ast;
use z3::{ast::Int, SatResult};

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
struct Game {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

#[derive(Debug, Clone, Hash)]
struct Input {
    games: Vec<Game>,
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
        .games
        .iter()
        .map(|game| {
            // We need to be able to find ax + by = c
            // Note that a and b are the a and b buttons for a single dimension (just x or y), c is the prize in that dimension

            let config = z3::Config::new();
            let context = z3::Context::new(&config);
            let solver = z3::Solver::new(&context);
            let a_press = z3::ast::Int::new_const(&context, "a_press");
            let ref_a_press = &a_press;
            let b_press = z3::ast::Int::new_const(&context, "b_press");
            let ref_b_press = &b_press;

            solver.assert(
                &Int::from_i64(&context, game.button_a.x)
                    .mul(ref_a_press)
                    .add(&Int::from_i64(&context, game.button_b.x).mul(ref_b_press))
                    ._eq(&Int::from_i64(&context, game.prize.x)),
            );
            solver.assert(
                &Int::from_i64(&context, game.button_a.y)
                    .mul(ref_a_press)
                    .add(&Int::from_i64(&context, game.button_b.y).mul(ref_b_press))
                    ._eq(&Int::from_i64(&context, game.prize.y)),
            );
            solver.assert(&ref_a_press.lt(&Int::from_i64(&context, 101)));
            solver.assert(&ref_b_press.lt(&Int::from_i64(&context, 101)));

            let result = solver.check();
            if result != SatResult::Sat {
                // Can't satisfy the constraints so we can't get this prize
                return 0;
            }

            let cost = solver
                .get_model()
                .unwrap()
                .eval(&a_press.mul(&Int::from_i64(&context, 3)).add(b_press), true)
                .unwrap()
                .as_i64()
                .unwrap();
            cost
        })
        .sum::<i64>()
}

fn part2(input: &Input) -> i64 {
    input
        .games
        .iter()
        .map(|game| {
            // We need to be able to find ax + by = c
            // Note that a and b are the a and b buttons for a single dimension (just x or y), c is the prize in that dimension

            let config = z3::Config::new();
            let context = z3::Context::new(&config);
            let solver = z3::Solver::new(&context);
            let a_press = z3::ast::Int::new_const(&context, "a_press");
            let ref_a_press = &a_press;
            let b_press = z3::ast::Int::new_const(&context, "b_press");
            let ref_b_press = &b_press;

            solver.assert(
                &Int::from_i64(&context, game.button_a.x)
                    .mul(ref_a_press)
                    .add(&Int::from_i64(&context, game.button_b.x).mul(ref_b_press))
                    ._eq(&Int::from_i64(&context, game.prize.x + 10000000000000)),
            );
            solver.assert(
                &Int::from_i64(&context, game.button_a.y)
                    .mul(ref_a_press)
                    .add(&Int::from_i64(&context, game.button_b.y).mul(ref_b_press))
                    ._eq(&Int::from_i64(&context, game.prize.y + 10000000000000)),
            );

            let result = solver.check();
            if result != SatResult::Sat {
                // Can't satisfy the constraints so we can't get this prize
                return 0;
            }

            let cost = solver
                .get_model()
                .unwrap()
                .eval(&a_press.mul(&Int::from_i64(&context, 3)).add(b_press), true)
                .unwrap()
                .as_i64()
                .unwrap();
            cost
        })
        .sum::<i64>()
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut games = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        //         Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400
        let button_a_line = &lines[i];
        let button_b_line = &lines[i + 1];
        let prize_line = &lines[i + 2];
        games.push(Game {
            button_a: Position {
                x: button_a_line[(button_a_line.find("X").unwrap() + 1)..]
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .trim_end_matches(',')
                    .parse::<i64>()
                    .unwrap(),
                y: button_a_line[(button_a_line.find("Y").unwrap() + 1)..]
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            },
            button_b: Position {
                x: button_b_line[(button_b_line.find("X").unwrap() + 1)..]
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .trim_end_matches(',')
                    .parse::<i64>()
                    .unwrap(),
                y: button_b_line[(button_b_line.find("Y").unwrap() + 1)..]
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            },
            prize: Position {
                x: prize_line[(prize_line.find("X").unwrap() + 2)..]
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .trim_end_matches(',')
                    .parse::<i64>()
                    .unwrap(),
                y: prize_line[(prize_line.find("Y").unwrap() + 2)..]
                    .parse::<i64>()
                    .unwrap(),
            },
        });

        i += 4;
    }

    Input { games }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 480);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
