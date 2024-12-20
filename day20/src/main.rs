use clap::Parser;
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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
    walls: HashSet<Position>,
    start: Position,
    exit: Position,
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
    let max_bounds = Position {
        x: input.walls.iter().map(|w| w.x).max().unwrap(),
        y: input.walls.iter().map(|w| w.y).max().unwrap(),
    };
    // First find the best route, then we can start cheating
    // Map from a Position to the number of seconds it takes to get to the end. This is without cheating.
    let mut shortest_path_to_end: HashMap<Position, i64> = HashMap::new();

    let mut states = BinaryHeap::new();
    states.push(Reverse((0, input.exit)));

    while states.len() > 0 {
        let Reverse((time, pos)) = states.pop().unwrap();

        if shortest_path_to_end.contains_key(&pos) {
            continue;
        }
        shortest_path_to_end.insert(pos.clone(), time);

        let new_pos = pos + Position { x: -1, y: 0 };
        if pos.x > 0 && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }

        let new_pos = pos + Position { x: 1, y: 0 };
        if pos.x < max_bounds.x && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }

        let new_pos = pos + Position { x: 0, y: -1 };
        if pos.y > 0 && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }

        let new_pos = pos + Position { x: 0, y: 1 };
        if pos.y < max_bounds.y && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }
    }

    let shortest_no_cheat_path = shortest_path_to_end.get(&input.start).unwrap();
    println!("Shortest path: {:?}", shortest_no_cheat_path);

    let mut shortest_path_to_start: HashMap<Position, i64> = HashMap::new();

    let mut states = BinaryHeap::new();
    states.push(Reverse((0, input.start)));

    while states.len() > 0 {
        let Reverse((time, pos)) = states.pop().unwrap();

        if shortest_path_to_start.contains_key(&pos) {
            continue;
        }
        shortest_path_to_start.insert(pos.clone(), time);

        let new_pos = pos + Position { x: -1, y: 0 };
        if pos.x > 0 && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }

        let new_pos = pos + Position { x: 1, y: 0 };
        if pos.x < max_bounds.x && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }

        let new_pos = pos + Position { x: 0, y: -1 };
        if pos.y > 0 && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }

        let new_pos = pos + Position { x: 0, y: 1 };
        if pos.y < max_bounds.y && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }
    }

    let mut cheats = HashMap::new();
    for y in 0..=max_bounds.y {
        for x in 0..=max_bounds.x {
            let pos = Position { x, y };

            if input.walls.contains(&pos) {
                continue;
            }

            // Can cheat for two steps, but really that's just into a wall and immediately out.
            // The new time will be the steps from start + 2 (time in wall) + steps to end of the new place

            let start_time = *shortest_path_to_start.get(&pos).unwrap();

            for dir in vec![
                Position { x: 0, y: 1 },
                Position { x: 0, y: -1 },
                Position { x: -1, y: 0 },
                Position { x: 1, y: 0 },
            ]
            .into_iter()
            {
                let wall_candidate = pos + dir;
                if wall_candidate.x >= 0
                    && wall_candidate.y >= 0
                    && wall_candidate.x <= max_bounds.x
                    && wall_candidate.y <= max_bounds.y
                    && input.walls.contains(&wall_candidate)
                {
                    vec![
                        Position { x: 0, y: 1 },
                        Position { x: 0, y: -1 },
                        Position { x: -1, y: 0 },
                        Position { x: 1, y: 0 },
                    ]
                    .into_iter()
                    .filter_map(|p| {
                        let dest = wall_candidate + p;
                        if dest.x < 0
                            || dest.y < 0
                            || dest.x >= max_bounds.x
                            || dest.y >= max_bounds.y
                        {
                            return None;
                        }
                        if input.walls.contains(&dest) {
                            return None;
                        }
                        if dest == pos {
                            return None;
                        }

                        Some(dest)
                    })
                    .for_each(|dest| {
                        cheats.insert(
                            (pos.clone(), dest.clone()),
                            start_time + 2 + shortest_path_to_end.get(&dest).unwrap(),
                        );
                    });
                }
            }
        }
    }

    println!(
        "Cheats: {:?}",
        cheats
            .iter()
            .filter(|(_, t)| *t < shortest_no_cheat_path)
            .sorted_by_key(|(_, t)| *t)
            .count()
    );

    cheats
        .iter()
        .filter(|(_, t)| **t <= shortest_no_cheat_path - 100)
        .count()
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

    let walls: HashSet<Position> = lines
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
    let start = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'S')
                .map(move |(x, _)| Position {
                    x: x as i64,
                    y: y as i64,
                })
        })
        .collect_vec()[0];
    let exit = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'E')
                .map(move |(x, _)| Position {
                    x: x as i64,
                    y: y as i64,
                })
        })
        .collect_vec()[0];

    Input { walls, start, exit }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 0);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
