use clap::Parser;
use itertools::Itertools;
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Robot {
    position: Position,
    velocity: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Hash)]
struct Input {
    robots: Vec<Robot>,
}

fn main() {
    let args = Args::parse();
    let data_file = if args.data_file.is_empty() {
        format!("{}/src/data.txt", env!("CARGO_MANIFEST_DIR"))
    } else {
        args.data_file
    };

    let input = parse(&data_file);

    let result1 = part1(&input, 101, 103);
    println!("Part1: {}", result1);

    println!("Part 2: {}", part2(&input))
}

fn part1(input: &Input, width: usize, height: usize) -> i64 {
    let final_positions = input
        .robots
        .iter()
        .map(|r| simulate_robot(r, width, height, 100))
        .collect_vec();

    let quadrant_1 = final_positions
        .iter()
        .filter(|p| p.x < ((width - 1) / 2) as i64 && p.y < ((height - 1) / 2) as i64)
        .count();
    let quadrant_2 = final_positions
        .iter()
        .filter(|p| p.x < ((width - 1) / 2) as i64 && p.y > ((height - 1) / 2) as i64)
        .count();
    let quadrant_3 = final_positions
        .iter()
        .filter(|p| p.x > ((width - 1) / 2) as i64 && p.y < ((height - 1) / 2) as i64)
        .count();
    let quadrant_4 = final_positions
        .iter()
        .filter(|p| p.x > ((width - 1) / 2) as i64 && p.y > ((height - 1) / 2) as i64)
        .count();

    (quadrant_1 * quadrant_2 * quadrant_3 * quadrant_4) as i64
}

fn part2(input: &Input) -> i64 {
    // A christmas tree with 500 pixels?
    // Base should be 101 pixels wide?

    let mut robots = input.robots.clone();
    let mut t = 0;

    // Intersting timestamps:
    // 48
    // 104 -- 56
    // 149 -- 45
    // 207 -- 58
    // 250 -- 43
    // 310 -- 60
    // 351 -- 41
    // 413 -- 62
    // 452 -- 39
    // 516 -- 64
    // 553 -- 37

    let mut jump1 = 45;
    let mut jump2 = 56;
    let mut current_jump_number = 2;

    robots
        .iter_mut()
        .for_each(|r| r.position = simulate_robot(r, 101, 103, 48));
    t = 48;

    loop {
        let jump_amount;
        if current_jump_number == 1 {
            jump_amount = jump1;
            jump1 -= 2;
            current_jump_number = 2;
        } else {
            jump_amount = jump2;
            jump2 += 2;
            current_jump_number = 1;
        }
        t += jump_amount;
        robots
            .iter_mut()
            .for_each(|r| r.position = simulate_robot(r, 101, 103, jump_amount));

        println!("t = {}", t);
        for y in 0..103 {
            for x in 0..101 {
                if robots.iter().any(|r| r.position == Position { x, y }) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    0
}

fn simulate_robot(r: &Robot, width: usize, height: usize, steps: i64) -> Position {
    Position {
        x: (r.position.x + r.velocity.x * steps).rem_euclid(width as i64),
        y: (r.position.y + r.velocity.y * steps).rem_euclid(height as i64),
    }
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    Input {
        robots: lines
            .iter()
            .map(|line| {
                let (p, v) = line.split_once(' ').unwrap();
                let (px, py) = p[2..].split_once(',').unwrap();
                let (vx, vy) = v[2..].split_once(',').unwrap();

                Robot {
                    position: Position {
                        x: px.parse::<i64>().unwrap(),
                        y: py.parse::<i64>().unwrap(),
                    },
                    velocity: Position {
                        x: vx.parse::<i64>().unwrap(),
                        y: vy.parse::<i64>().unwrap(),
                    },
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
        let result1 = part1(&input, 11, 7);

        assert_eq!(result1, 12);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
