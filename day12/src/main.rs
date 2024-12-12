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
    plots: HashMap<Position, char>,
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
    let mut to_evaluate = input
        .plots
        .iter()
        .map(|(position, _)| *position)
        .collect::<HashSet<Position>>();

    let mut result = 0;

    while !to_evaluate.is_empty() {
        let start_position = *to_evaluate.iter().next().unwrap();
        let plot_type = *input.plots.get(&start_position).unwrap();

        let mut perimiter = 0;
        let mut area = 0;

        let mut to_see = VecDeque::new();
        to_see.push_back(start_position);

        while !to_see.is_empty() {
            let position = to_see.pop_front().unwrap();
            if !to_evaluate.remove(&position) {
                continue;
            }
            area += 1;

            if position.x == 0 {
                perimiter += 1;
            } else if *input
                .plots
                .get(&Position {
                    x: position.x - 1,
                    y: position.y,
                })
                .unwrap()
                != plot_type
            {
                perimiter += 1;
            } else {
                to_see.push_back(Position {
                    x: position.x - 1,
                    y: position.y,
                });
            }

            if position.x == input.map_limits.x - 1 {
                perimiter += 1;
            } else if *input
                .plots
                .get(&Position {
                    x: position.x + 1,
                    y: position.y,
                })
                .unwrap()
                != plot_type
            {
                perimiter += 1;
            } else {
                to_see.push_back(Position {
                    x: position.x + 1,
                    y: position.y,
                });
            }

            if position.y == 0 {
                perimiter += 1;
            } else if *input
                .plots
                .get(&Position {
                    x: position.x,
                    y: position.y - 1,
                })
                .unwrap()
                != plot_type
            {
                perimiter += 1;
            } else {
                to_see.push_back(Position {
                    x: position.x,
                    y: position.y - 1,
                });
            }

            if position.y == input.map_limits.y - 1 {
                perimiter += 1;
            } else if *input
                .plots
                .get(&Position {
                    x: position.x,
                    y: position.y + 1,
                })
                .unwrap()
                != plot_type
            {
                perimiter += 1;
            } else {
                to_see.push_back(Position {
                    x: position.x,
                    y: position.y + 1,
                });
            }
        }

        result += perimiter * area;
    }

    result
}

fn part2(input: &Input) -> i64 {
    let mut to_evaluate = input
        .plots
        .iter()
        .map(|(position, _)| *position)
        .collect::<HashSet<Position>>();

    let mut result = 0;

    while !to_evaluate.is_empty() {
        let start_position = *to_evaluate.iter().next().unwrap();
        let plot_type = *input.plots.get(&start_position).unwrap();

        // Records top side of a cell
        let mut horizontal_sides = HashSet::new();
        // Records left side of a cell
        let mut vertical_sides = HashSet::new();
        let mut area = 0;

        let mut to_see = VecDeque::new();
        to_see.push_back(start_position);

        while !to_see.is_empty() {
            let position = to_see.pop_front().unwrap();
            if !to_evaluate.remove(&position) {
                continue;
            }
            area += 1;

            if position.x == 0 {
                vertical_sides.insert(position);
            } else if *input
                .plots
                .get(&Position {
                    x: position.x - 1,
                    y: position.y,
                })
                .unwrap()
                != plot_type
            {
                vertical_sides.insert(position);
            } else {
                to_see.push_back(Position {
                    x: position.x - 1,
                    y: position.y,
                });
            }

            if position.x == input.map_limits.x - 1 {
                vertical_sides.insert(Position {
                    x: position.x + 1,
                    y: position.y,
                });
            } else if *input
                .plots
                .get(&Position {
                    x: position.x + 1,
                    y: position.y,
                })
                .unwrap()
                != plot_type
            {
                vertical_sides.insert(Position {
                    x: position.x + 1,
                    y: position.y,
                });
            } else {
                to_see.push_back(Position {
                    x: position.x + 1,
                    y: position.y,
                });
            }

            if position.y == 0 {
                horizontal_sides.insert(position);
            } else if *input
                .plots
                .get(&Position {
                    x: position.x,
                    y: position.y - 1,
                })
                .unwrap()
                != plot_type
            {
                horizontal_sides.insert(position);
            } else {
                to_see.push_back(Position {
                    x: position.x,
                    y: position.y - 1,
                });
            }

            if position.y == input.map_limits.y - 1 {
                horizontal_sides.insert(Position {
                    x: position.x,
                    y: position.y + 1,
                });
            } else if *input
                .plots
                .get(&Position {
                    x: position.x,
                    y: position.y + 1,
                })
                .unwrap()
                != plot_type
            {
                horizontal_sides.insert(Position {
                    x: position.x,
                    y: position.y + 1,
                });
            } else {
                to_see.push_back(Position {
                    x: position.x,
                    y: position.y + 1,
                });
            }
        }

        let mut sides = horizontal_sides
            .iter()
            .sorted_by_key(|p| p.y)
            .chunk_by(|p| p.y)
            .into_iter()
            .map(|(y, sides)| {
                sides
                    .map(|p| p.x)
                    .sorted()
                    .fold((0, None), |(count, last_x), x| {
                        (
                            if last_x == None
                                || last_x.unwrap() != x - 1
                                || vertical_sides.contains(&Position { x: x, y: y })
                                || vertical_sides.contains(&Position { x: x, y: y - 1 })
                            {
                                count + 1
                            } else {
                                count
                            },
                            Some(x),
                        )
                    })
                    .0
            })
            .sum::<i64>();
        sides += vertical_sides
            .iter()
            .sorted_by_key(|p| p.x)
            .chunk_by(|p| p.x)
            .into_iter()
            .map(|(x, sides)| {
                sides
                    .map(|p| p.y)
                    .sorted()
                    .fold((0, None), |(count, last_y), y| {
                        (
                            if last_y == None
                                || last_y.unwrap() != y - 1
                                || horizontal_sides.contains(&Position { x: x, y: y })
                                || horizontal_sides.contains(&Position { x: x - 1, y: y })
                            {
                                count + 1
                            } else {
                                count
                            },
                            Some(y),
                        )
                    })
                    .0
            })
            .sum::<i64>();
        let price = sides * area;
        println!(
            "{} - {:?}: {} * {} = {}",
            plot_type, start_position, sides, area, price
        );
        result += price;
    }

    result
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
        plots: lines
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        (
                            Position {
                                x: x as i64,
                                y: y as i64,
                            },
                            c,
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

        assert_eq!(result1, 1930);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 12066);
    }
}
