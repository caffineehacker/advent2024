use clap::Parser;
use itertools::Itertools;
use std::{
    collections::HashSet,
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
    boxes: HashSet<Position>,
    walls: HashSet<Position>,
    robot: Position,
    instructions: Vec<Position>,
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
    let instructions = input.instructions.clone();
    let mut input = input.clone();
    print_grid(&input);
    for instruction in instructions {
        // println!("Moving: {:?}", instruction);
        move_robot(&mut input, instruction);
        // print_grid(&input);
        // println!();
    }

    input.boxes.iter().map(|s| (s.y * 100) + s.x).sum::<i64>()
}

fn print_grid(input: &Input) {
    let max_x = input.walls.iter().max_by_key(|w| w.x).unwrap().x + 1;
    let max_y = input.walls.iter().max_by_key(|w| w.y).unwrap().y + 1;

    for y in 0..max_y {
        for x in 0..max_x {
            let position = Position { x, y };
            if input.walls.contains(&position) {
                print!("#");
            } else if input.boxes.contains(&position) {
                print!("O");
            } else if input.robot == position {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn move_robot(input: &mut Input, instruction: Position) {
    let robot_destination = input.robot + instruction;
    if input.walls.contains(&robot_destination) {
        return;
    }

    let mut boxes_to_move = Vec::new();

    loop {
        let target_position = if boxes_to_move.len() > 0 {
            boxes_to_move[boxes_to_move.len() - 1]
        } else {
            input.robot
        } + instruction;
        if input.walls.contains(&target_position) {
            return;
        }
        if input.boxes.contains(&target_position) {
            boxes_to_move.push(target_position);
            continue;
        }
        break;
    }

    for b in boxes_to_move.iter() {
        input.boxes.remove(b);
    }

    for b in boxes_to_move {
        input.boxes.insert(b + instruction);
    }

    input.robot = input.robot + instruction;
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

    let grid_lines = lines.iter().take_while(|l| !l.is_empty()).collect_vec();
    let boxes: HashSet<Position> = grid_lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'O')
                .map(move |(x, _)| Position {
                    x: x as i64,
                    y: y as i64,
                })
        })
        .collect();
    let walls: HashSet<Position> = grid_lines
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
    let robot = grid_lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '@')
                .map(move |(x, _)| Position {
                    x: x as i64,
                    y: y as i64,
                })
        })
        .collect_vec()[0];

    let instructions = lines
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .flat_map(|line| {
            line.chars().map(|c| {
                if c == '<' {
                    Position { x: -1, y: 0 }
                } else if c == '^' {
                    Position { x: 0, y: -1 }
                } else if c == '>' {
                    Position { x: 1, y: 0 }
                } else {
                    Position { x: 0, y: 1 }
                }
            })
        })
        .collect_vec();

    Input {
        boxes,
        walls,
        robot,
        instructions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 10092);
    }

    #[test]
    fn test_part1_2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test2.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 2028);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
