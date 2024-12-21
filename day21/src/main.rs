use clap::Parser;
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    ops::Add,
    vec,
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

#[derive(Debug, Clone, Hash)]
struct Input {
    codes: Vec<String>,
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

fn find_distance(
    pads: &Vec<Vec<(char, Position)>>,
    source: char,
    target: char,
    robot: usize,
    robots: &mut Vec<HashMap<(char, char), i64>>,
) -> i64 {
    if robots[robot].contains_key(&(source, target)) {
        return robots[robot][&(source, target)];
    }

    let mut to_process = BinaryHeap::new();
    let source_pos = pads[robot].iter().find(|(c, _)| *c == source).unwrap().1;
    let target_pos = pads[robot].iter().find(|(c, _)| *c == target).unwrap().1;
    to_process.push(Reverse((0, source_pos, 'A')));

    let mut best_so_far = None;

    while !to_process.is_empty() {
        let Reverse((dist, pos, last_press)) = to_process.pop().unwrap();

        if let Some(best) = best_so_far {
            if best < dist {
                break;
            }
        }

        if pos == target_pos {
            let new_cost = dist + find_distance(pads, last_press, 'A', robot - 1, robots);

            if let Some(best) = best_so_far {
                if best < new_cost {
                    continue;
                }
            }

            best_so_far = Some(new_cost);
            continue;
        }

        for dir in vec![
            (Position { x: 1, y: 0 }, '>'),
            (Position { x: -1, y: 0 }, '<'),
            (Position { x: 0, y: 1 }, 'v'),
            (Position { x: 0, y: -1 }, '^'),
        ] {
            let newpos = pos + dir.0;
            if !pads[robot].iter().any(|k| k.1 == newpos) {
                continue;
            }

            to_process.push(Reverse((
                dist + find_distance(pads, last_press, dir.1, robot - 1, robots),
                newpos,
                dir.1,
            )));
        }
    }

    robots[robot].insert((source, target), best_so_far.unwrap());

    best_so_far.unwrap()
}

fn find_distances(
    pad: &Vec<(char, Position)>,
    costs: &HashMap<(char, char), (i64, Vec<char>)>,
) -> HashMap<(char, char), (i64, Vec<char>)> {
    let mut distances = HashMap::new();

    for i in 0..pad.len() {
        for j in 0..pad.len() {
            let source = pad[i];
            let target = pad[j];

            let mut to_process = BinaryHeap::new();
            to_process.push(Reverse((0, source.1, 'A', vec![])));

            while !to_process.is_empty() {
                let Reverse((dist, pos, last_press, crumbs)) = to_process.pop().unwrap();

                if let Some((best, _)) = distances.get(&(source.0, target.0)) {
                    if *best < dist {
                        break;
                    }
                }

                if pos == target.1 {
                    let mut new_crumbs = crumbs.clone();
                    new_crumbs.append(&mut costs.get(&(last_press, 'A')).unwrap().1.clone());
                    let new_cost = (dist + costs.get(&(last_press, 'A')).unwrap().0, new_crumbs);

                    if let Some((best, _)) = distances.get(&(source.0, target.0)) {
                        if *best < new_cost.0 {
                            continue;
                        }
                    }

                    distances.insert((source.0, target.0), new_cost);
                    continue;
                }

                for dir in vec![
                    (Position { x: 1, y: 0 }, '>'),
                    (Position { x: -1, y: 0 }, '<'),
                    (Position { x: 0, y: 1 }, 'v'),
                    (Position { x: 0, y: -1 }, '^'),
                ] {
                    let newpos = pos + dir.0;
                    if !pad.iter().any(|k| k.1 == newpos) {
                        continue;
                    }

                    // println!("{:?} -> {:?} ({})", pos, dir.1, last_press);
                    let mut new_crumbs = crumbs.clone();
                    new_crumbs.append(&mut costs.get(&(last_press, dir.1)).unwrap().1.clone());

                    to_process.push(Reverse((
                        dist + costs.get(&(last_press, dir.1)).unwrap().0,
                        newpos,
                        dir.1,
                        new_crumbs,
                    )));
                }
            }
        }
    }

    distances
}

fn part1(input: &Input) -> i64 {
    let d_pad = vec![
        ('^', Position { x: 1, y: 0 }),
        ('A', Position { x: 2, y: 0 }),
        ('<', Position { x: 0, y: 1 }),
        ('v', Position { x: 1, y: 1 }),
        ('>', Position { x: 2, y: 1 }),
    ];

    let mut base_costs = HashMap::new();
    for s in vec!['^', 'A', '<', 'v', '>'] {
        for d in vec!['^', 'A', '<', 'v', '>'] {
            base_costs.insert((s, d), (1, vec![d]));
        }
    }

    // Figure out the distance between buttons and number of presses for the previous panel.
    // For example, the number of presses from A to Up is 1. For this first case we're just going to hard code.
    // let mut d_pad_distances = HashMap::new();
    // d_pad_distances.insert(('A', '^'), 2);
    // d_pad_distances.insert(('A', '>'), 2);
    // d_pad_distances.insert(('A', 'v'), 3);
    // d_pad_distances.insert(('A', '<'), 4);
    // d_pad_distances.insert(('^', 'A'), 2);
    // d_pad_distances.insert(('^', '>'), 3);
    // d_pad_distances.insert(('^', 'v'), 2);
    // d_pad_distances.insert(('^', '<'), 3);
    // d_pad_distances.insert(('>', 'A'), 2);
    // d_pad_distances.insert(('>', '^'), 3);
    // d_pad_distances.insert(('>', 'v'), 2);
    // d_pad_distances.insert(('>', '<'), 3);
    // d_pad_distances.insert(('v', 'A'), 3);
    // d_pad_distances.insert(('v', '>'), 2);
    // d_pad_distances.insert(('v', '^'), 2);
    // d_pad_distances.insert(('v', '<'), 2);
    // d_pad_distances.insert(('<', 'A'), 4);
    // d_pad_distances.insert(('<', '^'), 3);
    // d_pad_distances.insert(('<', '>'), 3);
    // d_pad_distances.insert(('<', 'v'), 2);

    let robot_1_costs = find_distances(&d_pad, &base_costs);
    println!("Robot 1: {:?}", robot_1_costs);

    // So we need distances for a d_pad to d_pad distances.
    // This is for the first to second robot.
    // So for the second robot to go from < to ^ it will be more.

    let robot_2_costs = find_distances(&d_pad, &robot_1_costs);
    println!("Robot 2: {:?}", robot_2_costs);

    // I press a button to move a robot, the d_pad_distances are the number of button presses to move the robot to a given button
    // Then that robot needs to be pressing buttons to move a robot to other directional buttons.
    // The final layer being a robot pressing a numeric keypad.

    let keypad = vec![
        ('7', Position { x: 0, y: 0 }),
        ('4', Position { x: 0, y: 1 }),
        ('1', Position { x: 0, y: 2 }),
        ('8', Position { x: 1, y: 0 }),
        ('5', Position { x: 1, y: 1 }),
        ('2', Position { x: 1, y: 2 }),
        ('0', Position { x: 1, y: 3 }),
        ('9', Position { x: 2, y: 0 }),
        ('6', Position { x: 2, y: 1 }),
        ('3', Position { x: 2, y: 2 }),
        ('A', Position { x: 2, y: 3 }),
    ];

    let final_robot_costs = find_distances(&keypad, &robot_2_costs);
    println!("Robot 3: {:?}", final_robot_costs);

    input
        .codes
        .iter()
        .map(|code| {
            let distance =
                code.chars()
                    .fold((0, 'A', vec![]), |(total, last_button, crumbs), dest| {
                        let mut new_crumbs = crumbs;
                        new_crumbs.append(
                            &mut final_robot_costs
                                .get(&(last_button, dest))
                                .unwrap()
                                .1
                                .clone(),
                        );
                        (
                            total + final_robot_costs.get(&(last_button, dest)).unwrap().0,
                            dest,
                            new_crumbs,
                        )
                    });
            let number = code.trim_end_matches("A").parse::<i64>().unwrap();
            println!("{}: {} * {}", code, distance.0, number);
            println!("{:?}", distance.2);
            distance.0 * number
        })
        .sum()

    /*
            Robot 1: {('v', '^'): 2, ('A', '^'): 2, ('>', 'A'): 2, ('>', 'v'): 2, ('^', '^'): 1, ('^', '>'): 3, ('A', '>'): 2, ('v', 'A'): 3, ('>', '^'): 3, ('<', '<'): 1, ('>', '>'): 1, ('^', 'v'): 2, ('A', '<'): 4, ('<', '>'): 3, ('<', 'v'): 2, ('^', 'A'): 2, ('^', '<'): 3, ('A', 'v'): 3, ('A', 'A'): 1, ('<', '^'): 3, ('v', '>'): 2, ('v', '<'): 2, ('>', '<'): 3, ('v', 'v'): 1, ('<', 'A'): 4}
            Robot 2: {('>', 'A'): 4, ('A', '>'): 6, ('>', 'v'): 8, ('>', '>'): 1, ('A', 'v'): 9, ('v', 'v'): 1, ('^', '>'): 7, ('A', 'A'): 1, ('^', '<'): 9, ('A', '^'): 8, ('^', 'v'): 6, ('>', '^'): 9, ('>', '<'): 9, ('^', 'A'): 4, ('v', 'A'): 7, ('A', '<'): 10, ('v', '<'): 8, ('^', '^'): 1, ('<', '>'): 5, ('<', '<'): 1, ('v', '^'): 4, ('v', '>'): 4, ('<', '^'): 7, ('<', 'A'): 8, ('<', 'v'): 4}
            Robot 3: {('0', '6'): 20, ('2', '0'): 16, ('4', '8'): 19, ('A', '9'): 14, ('5', '1'): 21, ('6', '1'): 22, ('9', 'A'): 18, ('7', '7'): 1, ('A', '2'): 25, ('0', '3'): 19, ('A', '1'): 26, ('A', '0'): 18, ('6', '2'): 21, ('4', 'A'): 23, ('2', '3'): 10, ('8', '0'): 18, ('8', '2'): 17, ('3', 'A'): 16, ('2', '7'): 26, ('9', '4'): 22, ('9', '8'): 18, ('4', '5'): 10, ('A', '3'): 12, ('8', '3'): 18, ('3', '1'): 19, ('4', '0'): 22, ('7', '1'): 17, ('7', '3'): 19, ('8', '9'): 10, ('4', '4'): 1, ('5', '0'): 17, ('1', '9'): 21, ('6', '4'): 19, ('1', '4'): 12, ('8', '4'): 21, ('1', '6'): 20, ('5', '4'): 18, ('1', '8'): 20, ('0', '1'): 25, ('7', 'A'): 24, ('1', '7'): 13, ('8', '1'): 22, ('2', '9'): 20, ('5', 'A'): 18, ('1', '3'): 11, ('2', '5'): 12, ('0', '7'): 27, ('8', '6'): 17, ('A', '6'): 13, ('3', '5'): 25, ('8', '7'): 18, ('5', '5'): 1, ('5', '6'): 10, ('1', '1'): 1, ('5', '8'): 12, ('2', '2'): 1, ('2', '6'): 19, ('0', '2'): 12, ('9', '0'): 23, ('3', '4'): 26, ('0', '8'): 14, ('A', '4'): 27, ('4', '6'): 11, ('6', '3'): 16, ('3', '8'): 26, ('9', '5'): 21, ('9', '3'): 17, ('4', '7'): 12, ('2', '4'): 25, ('1', '2'): 10, ('9', '7'): 19, ('3', '7'): 27, ('A', '5'): 26, ('5', '3'): 17, ('9', '9'): 1, ('0', '5'): 13, ('4', '1'): 16, ('7', '2'): 18, ('3', '9'): 13, ('6', '6'): 1, ('2', 'A'): 17, ('A', '7'): 28, ('7', '5'): 17, ('1', 'A'): 22, ('5', '2'): 16, ('0', '4'): 26, ('0', '0'): 1, ('3', '0'): 21, ('6', 'A'): 17, ('7', '0'): 23, ('7', '4'): 16, ('3', '3'): 1, ('8', '8'): 1, ('8', 'A'): 19, ('5', '9'): 19, ('2', '1'): 18, ('0', 'A'): 10, ('9', '1'): 23, ('6', '0'): 22, ('A', 'A'): 1, ('6', '9'): 12, ('9', '2'): 22, ('7', '6'): 18, ('0', '9'): 21, ('4', '2'): 17, ('5', '7'): 25, ('8', '5'): 16, ('1', '5'): 19, ('4', '9'): 20, ('7', '8'): 10, ('7', '9'): 11, ('1', '0'): 21, ('2', '8'): 13, ('9', '6'): 16, ('3', '2'): 18, ('4', '3'): 18, ('6', '8'): 25, ('6', '7'): 26, ('6', '5'): 18, ('3', '6'): 12, ('A', '8'): 27}


    980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
    179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
    379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

    029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
            029A: 68 * 29
          v<A<AA>^>AvAA^<A>A<v<A>^>AvA^A<v<A>^>AAvA<A^>A<A>Av<A<A>^>AAA<Av>A^A]
        980A: 60 * 980
        <v<A>^>AAAvA^Av<A<AA>^>AvAA^<A>Av<A<A>^>AAA<Av>A^Av<A^>A<A>A]
        179A: 68 * 179
        <v<A>^>Av<A<A>^>AAvAA^<A>A<v<A>^>AAvA^Av<A^>AA<A>Av<A<A>^>AAA<Av>A^A]
        456A: 64 * 456
        <v<A>^>AAv<A<A>^>AAvAA^<A>Av<A^>A<A>Av<A^>A<A>Av<A<A>^>AA<Av>A^A]
        379A: 68 * 379
        <v<A = <
        >>^A = A 2= ^
        vA = >
        ^A = A 2= A 3= 3
        <vA = v, but <v<A = <
        <A = <, but >^>A = A, 2= ^
        A = <, but A = A


        <v<A>>^AvA^A  <vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        <v<A>^>AvA^A  <v<A>^>AAv<A<A>^>AAvAA^<A>Av<A^>AA<A>Av<A<A>^>AAA<Av>A^A]
             */

    // Since the first two robots will be on 'A' to submit any button we can only worry about distances between numbers for the final robot.
    // So we just need distances between buttons and don't need to worry about tracking all of the paths.

    // let mut known_distances = HashMap::new();
    // for i in 0..keypad.len() {
    //     for j in 0..keypad.len() {
    //         if i == j {
    //             continue;
    //         }

    //         let source = keypad[i];
    //         let target = keypad[j];

    //         let mut to_process = BinaryHeap::new();
    //         to_process.push(Reverse((0, source.0)));

    //         while !to_process.is_empty() {
    //             let Reverse((dist, pos)) = to_process.pop().unwrap();

    //             if pos == target.0 {
    //                 known_distances.insert((source.0, target.0), dist);
    //                 break;
    //             }

    //             for dir in vec![
    //                 Position { x: 1, y: 0 },
    //                 Position { x: -1, y: 0 },
    //                 Position { x: 0, y: 1 },
    //                 Position { x: 0, y: -1 },
    //             ] {
    //                 let newpos = pos + dir;
    //                 if !keypad.iter().any(|k| k.0 == newpos) {
    //                     continue;
    //                 }

    //                 to_process.push(Reverse((dist + , newpos)));
    //             }
    //         }
    //     }
    // }

    // let mut robots = vec!['A', 'A', 'A'];

    // 0
}

fn part2(input: &Input) -> i64 {
    let d_pad = vec![
        ('^', Position { x: 1, y: 0 }),
        ('A', Position { x: 2, y: 0 }),
        ('<', Position { x: 0, y: 1 }),
        ('v', Position { x: 1, y: 1 }),
        ('>', Position { x: 2, y: 1 }),
    ];

    let mut base_costs = HashMap::new();
    for s in vec!['^', 'A', '<', 'v', '>'] {
        for d in vec!['^', 'A', '<', 'v', '>'] {
            base_costs.insert((s, d), 1);
        }
    }

    // let robot_1_costs = find_distances(&d_pad, &base_costs);
    // println!("Robot 1: {:?}", robot_1_costs);

    // // So we need distances for a d_pad to d_pad distances.
    // // This is for the first to second robot.
    // // So for the second robot to go from < to ^ it will be more.

    // let robot_2_costs = find_distances(&d_pad, &robot_1_costs);
    // println!("Robot 2: {:?}", robot_2_costs);

    // let mut robots = Vec::new();
    // robots.push(base_costs);
    // for i in 0..25 {
    //     robots.push(find_distances(&d_pad, robots.last().unwrap()));
    // }

    let keypad = vec![
        ('7', Position { x: 0, y: 0 }),
        ('4', Position { x: 0, y: 1 }),
        ('1', Position { x: 0, y: 2 }),
        ('8', Position { x: 1, y: 0 }),
        ('5', Position { x: 1, y: 1 }),
        ('2', Position { x: 1, y: 2 }),
        ('0', Position { x: 1, y: 3 }),
        ('9', Position { x: 2, y: 0 }),
        ('6', Position { x: 2, y: 1 }),
        ('3', Position { x: 2, y: 2 }),
        ('A', Position { x: 2, y: 3 }),
    ];

    let mut pads = Vec::new();
    for _ in 0..26 {
        pads.push(d_pad.clone());
    }
    pads.push(keypad);

    let mut robots = Vec::new();
    robots.push(base_costs);
    for _ in 0..26 {
        robots.push(HashMap::new());
    }

    // let final_robot_costs = find_distances(&keypad, &robots.last().unwrap());
    // println!("Robot 3: {:?}", final_robot_costs);

    input
        .codes
        .iter()
        .map(|code| {
            let distance = code.chars().fold((0, 'A'), |(total, last_button), dest| {
                (
                    total + find_distance(&pads, last_button, dest, 26, &mut robots),
                    dest,
                )
            });
            let number = code.trim_end_matches("A").parse::<i64>().unwrap();
            // println!("{}: {} * {}", code, distance.0, number);
            // println!("{:?}", distance.2);
            distance.0 * number
        })
        .sum()
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    Input { codes: lines }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 126384);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 0);
    }
}
