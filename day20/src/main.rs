use advent::parse::Parser as AventParser;
use advent::position::Position;
use clap::Parser;
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "")]
    data_file: String,
    #[arg(long)]
    debug: bool,
}

#[derive(Debug, Clone)]
struct Input {
    walls: HashSet<Position<isize>>,
    start: Position<isize>,
    exit: Position<isize>,
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
    let mut shortest_path_to_end = HashMap::new();

    let mut states = BinaryHeap::new();
    states.push(Reverse((0, input.exit)));

    while states.len() > 0 {
        let Reverse((time, pos)) = states.pop().unwrap();

        if shortest_path_to_end.contains_key(&pos) {
            continue;
        }
        shortest_path_to_end.insert(pos.clone(), time);

        if let Some(new_pos) = pos.checked_sub_x(1) {
            if !input.walls.contains(&new_pos) {
                states.push(Reverse((time + 1, new_pos)));
            }
        }

        if let Some(new_pos) = pos.limited_add_x(1, max_bounds) {
            if !input.walls.contains(&new_pos) {
                states.push(Reverse((time + 1, new_pos)));
            }
        }

        if let Some(new_pos) = pos.checked_sub_y(1) {
            if !input.walls.contains(&new_pos) {
                states.push(Reverse((time + 1, new_pos)));
            }
        }

        if let Some(new_pos) = pos.limited_add_y(1, max_bounds) {
            if !input.walls.contains(&new_pos) {
                states.push(Reverse((time + 1, new_pos)));
            }
        }
    }

    let shortest_no_cheat_path = shortest_path_to_end.get(&input.start).unwrap();
    println!("Shortest path: {:?}", shortest_no_cheat_path);

    let mut shortest_path_to_start = HashMap::new();

    let mut states = BinaryHeap::new();
    states.push(Reverse((0, input.start)));

    while states.len() > 0 {
        let Reverse((time, pos)) = states.pop().unwrap();

        if shortest_path_to_start.contains_key(&pos) {
            continue;
        }
        shortest_path_to_start.insert(pos.clone(), time);

        if let Some(new_pos) = pos.checked_sub_x(1) {
            if !input.walls.contains(&new_pos) {
                states.push(Reverse((time + 1, new_pos)));
            }
        }

        if let Some(new_pos) = pos.limited_add_x(1, max_bounds) {
            if !input.walls.contains(&new_pos) {
                states.push(Reverse((time + 1, new_pos)));
            }
        }

        if let Some(new_pos) = pos.checked_sub_y(1) {
            if !input.walls.contains(&new_pos) {
                states.push(Reverse((time + 1, new_pos)));
            }
        }

        if let Some(new_pos) = pos.limited_add_y(1, max_bounds) {
            if !input.walls.contains(&new_pos) {
                states.push(Reverse((time + 1, new_pos)));
            }
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

fn part2(input: &Input) -> usize {
    let max_bounds = Position {
        x: input.walls.iter().map(|w| w.x).max().unwrap(),
        y: input.walls.iter().map(|w| w.y).max().unwrap(),
    };
    // First find the best route, then we can start cheating
    // Map from a Position to the number of seconds it takes to get to the end. This is without cheating.
    let mut shortest_path_to_end = HashMap::new();

    let mut states = BinaryHeap::new();
    states.push(Reverse((0, input.exit)));

    while states.len() > 0 {
        let Reverse((time, pos)) = states.pop().unwrap();

        if shortest_path_to_end.contains_key(&pos) {
            continue;
        }
        shortest_path_to_end.insert(pos.clone(), time);

        let new_pos = pos - Position { x: 1, y: 0 };
        if pos.x > 0 && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }

        let new_pos = pos + Position { x: 1, y: 0 };
        if pos.x < max_bounds.x && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }

        let new_pos = pos - Position { x: 0, y: 1 };
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

    let mut shortest_path_to_start = HashMap::new();

    let mut states = BinaryHeap::new();
    states.push(Reverse((0, input.start)));

    while states.len() > 0 {
        let Reverse((time, pos)) = states.pop().unwrap();

        if shortest_path_to_start.contains_key(&pos) {
            continue;
        }
        shortest_path_to_start.insert(pos.clone(), time);

        let new_pos = pos - Position { x: 1, y: 0 };
        if pos.x > 0 && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }

        let new_pos = pos + Position { x: 1, y: 0 };
        if pos.x < max_bounds.x && !input.walls.contains(&new_pos) {
            states.push(Reverse((time + 1, new_pos)));
        }

        let new_pos = pos - Position { x: 0, y: 1 };
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

            for addx in 0..=20 {
                for addy in 0..=20 {
                    if addy + addx > 20 {
                        continue;
                    }

                    let dest = pos + Position { x: addx, y: addy };
                    if !(dest.x < 0
                        || dest.y < 0
                        || dest.x > max_bounds.x
                        || dest.y > max_bounds.y
                        || input.walls.contains(&dest))
                    {
                        cheats.insert(
                            (pos, dest),
                            start_time + addx + addy + shortest_path_to_end.get(&dest).unwrap(),
                        );
                    }

                    let dest = pos + Position { x: -addx, y: addy };
                    if !(dest.x < 0
                        || dest.y < 0
                        || dest.x > max_bounds.x
                        || dest.y > max_bounds.y
                        || input.walls.contains(&dest))
                    {
                        cheats.insert(
                            (pos, dest),
                            start_time + addx + addy + shortest_path_to_end.get(&dest).unwrap(),
                        );
                    }

                    let dest = pos + Position { x: addx, y: -addy };
                    if !(dest.x < 0
                        || dest.y < 0
                        || dest.x > max_bounds.x
                        || dest.y > max_bounds.y
                        || input.walls.contains(&dest))
                    {
                        cheats.insert(
                            (pos, dest),
                            start_time + addx + addy + shortest_path_to_end.get(&dest).unwrap(),
                        );
                    }

                    let dest = pos + Position { x: -addx, y: -addy };
                    if !(dest.x < 0
                        || dest.y < 0
                        || dest.x > max_bounds.x
                        || dest.y > max_bounds.y
                        || input.walls.contains(&dest))
                    {
                        cheats.insert(
                            (pos, dest),
                            start_time + addx + addy + shortest_path_to_end.get(&dest).unwrap(),
                        );
                    }
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

fn parse(file: &str) -> Input {
    let maze = AventParser::new(file).as_maze();

    Input {
        start: maze.get_only_position('S').unwrap(),
        exit: maze.get_only_position('E').unwrap(),
        walls: maze.walls,
    }
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
