use clap::Parser;
use itertools::Itertools;
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

#[derive(Debug, Clone, Hash)]
struct Input {
    blocks: Vec<i64>,
}

fn main() {
    let args = Args::parse();
    let data_file = if args.data_file.is_empty() {
        format!("{}/src/data.txt", env!("CARGO_MANIFEST_DIR"))
    } else {
        args.data_file
    };

    let input = parse(&data_file);

    let result1 = part1(&input, args.debug);
    println!("Part1: {}", result1);

    println!("Part 2: {}", part2(&input))
}

fn part1(input: &Input, debug: bool) -> i64 {
    // Let figure out the files and their offests / sizes first:
    // let mut files = Vec::new();
    // files.push((0, input.blocks[0]));
    // let mut current_free_index = 1;
    // let mut remaining_free = input.blocks[current_free_index];
    // let mut current_block = input.blocks[1];
    // let mut current_index_to_move = (input.blocks.len() - 1) as i64;
    // // If the last block is empty we can skip it
    // if current_index_to_move % 2 == 1 {
    //     current_index_to_move -= 1;
    // }
    // let mut remaining_blocks_to_move = input.blocks[current_index_to_move as usize];

    // if remaining_free >= remaining_blocks_to_move {
    //     files.push((current_index_to_move / 2, remaining_blocks_to_move));
    //     remaining_free -= remaining_blocks_to_move;
    //     remaining_blocks_to_move = 0;
    // } else {
    //     files.push((current_index_to_move / 2, remaining_free));
    //     remaining_blocks_to_move -= remaining_free;
    //     remaining_free = 0;
    // }

    // if remaining_free == 0 {
    //     current_free_index += 2;
    //     remaining_free = input.blocks[current_free_index];
    // }

    // Start with a sparse set of files and free space. Move from the last file block to each free space and calculate the final checksum
    let mut current_empty_index = 1;
    let mut current_block = *input.blocks.get(0).unwrap();
    let mut current_empty_block_offset = 0;
    let mut checksum = 0;
    let mut tail = (input.blocks.len() - 1) as i64;
    // If the last block is empty we can skip it
    if tail % 2 == 1 {
        tail -= 1;
    }

    if debug {
        for _ in 0..current_block {
            print!("0");
        }
    }

    let mut remaining_blocks_to_move = input.blocks[tail as usize];

    while current_empty_index < input.blocks.len() {
        // println!(
        //     "State: current_empty: {}, current_block: {}, current_empty_block_offset: {}, checksum: {}, tail: {}",
        //     current_empty_index, current_block, current_empty_block_offset, checksum, tail
        // );
        // println!("Remaining blocks to move: {}", remaining_blocks_to_move);
        if tail < current_empty_index as i64 {
            return checksum;
        }
        if tail - 1 == current_empty_index as i64 {
            for _ in current_block..(current_block + remaining_blocks_to_move) {
                print!("{}", (tail / 2));
            }
            // We need to shift the entry left to fill the remaining empty space
            checksum += (current_block..(current_block + remaining_blocks_to_move))
                .map(|index| index * (tail / 2))
                .sum::<i64>();
            return checksum;
        }
        let remaining_empty_blocks = input.blocks[current_empty_index] - current_empty_block_offset;
        for _ in
            current_block..(current_block + remaining_empty_blocks.min(remaining_blocks_to_move))
        {
            print!("{}", (tail / 2));
        }
        checksum += (current_block
            ..(current_block + remaining_empty_blocks.min(remaining_blocks_to_move)))
            .map(|index| index * (tail / 2))
            .sum::<i64>();
        if remaining_blocks_to_move >= remaining_empty_blocks {
            // println!(
            //     "Filled empty space {} >= {}",
            //     remaining_blocks_to_move, remaining_empty_blocks
            // );
            // We filled the empty blocks and have more to move
            // Add the checksum of the occupied blocks before the next empty space
            current_block += remaining_empty_blocks;
            remaining_blocks_to_move -= remaining_empty_blocks;
            if current_empty_index + 1 == tail as usize {
                // We ran into the same block, just count the remaining items
                checksum += (current_block..(current_block + remaining_blocks_to_move))
                    .map(|index| index * (tail / 2))
                    .sum::<i64>();
                for _ in 0..remaining_blocks_to_move {
                    print!("{}", tail / 2);
                }
                return checksum;
            } else {
                checksum += (current_block
                    ..(current_block + input.blocks[current_empty_index + 1]))
                    .map(|index| index * ((current_empty_index as i64 + 1) / 2))
                    .sum::<i64>();
                for _ in 0..(input.blocks[current_empty_index + 1]) {
                    print!("{}", (current_empty_index + 1) / 2);
                }
                current_block += input.blocks[current_empty_index + 1];
            }
            current_empty_index += 2;
            current_empty_block_offset = 0;
            if remaining_blocks_to_move == 0 {
                // println!("No more blocks to move");
                tail -= 2;
                remaining_blocks_to_move = input.blocks[tail as usize];
            }
        } else {
            // println!("More empty space than blocks to move");
            current_block += remaining_blocks_to_move;
            current_empty_block_offset += remaining_blocks_to_move;
            tail -= 2;
            remaining_blocks_to_move = input.blocks[tail as usize];
        }
    }

    0
}

fn part2(input: &Input) -> i64 {
    // Start with a sparse set of files and free space. Move from the last file block to each free space and calculate the final checksum
    let mut current_block = 0;
    let mut checksum = 0;

    let mut free_blocks = Vec::new();

    for i in 0..input.blocks.len() {
        if i % 2 == 0 {
            current_block += input.blocks[i];
            continue;
        }

        if input.blocks[i] == 0 {
            continue;
        }

        println!("Free block at {} with {}", current_block, input.blocks[i]);

        free_blocks.push((current_block, input.blocks[i]));
        current_block += input.blocks[i];
    }

    let mut occupied_blocks = Vec::new();

    current_block = 0;
    for i in 0..input.blocks.len() {
        if i % 2 == 1 {
            current_block += input.blocks[i];
            continue;
        }

        occupied_blocks.push((current_block, input.blocks[i]));
        current_block += input.blocks[i];
    }

    let mut tail = occupied_blocks.len() - 1;
    while tail > 0 {
        let result = free_blocks
            .iter()
            .find_position(|fb| fb.1 >= occupied_blocks[tail].1 && fb.0 < occupied_blocks[tail].0);

        match result {
            Some(result) => {
                let (index, (block, length)) = result;

                println!("Moving {} to {}", tail, block);

                checksum += (*block..(*block + occupied_blocks[tail].1))
                    .map(|index| index * tail as i64)
                    .sum::<i64>();

                if *length == occupied_blocks[tail].1 {
                    println!("Deleting free block at {}", block);
                    free_blocks.remove(index);
                } else {
                    free_blocks[index].1 -= occupied_blocks[tail].1;
                    free_blocks[index].0 += occupied_blocks[tail].1;

                    println!(
                        "Moved free block to {} with {}",
                        free_blocks[index].0, free_blocks[index].1
                    );
                }
            }
            None => {
                println!("Not moving {}", tail);
                checksum += (occupied_blocks[tail].0
                    ..(occupied_blocks[tail].0 + occupied_blocks[tail].1))
                    .map(|index| index * tail as i64)
                    .sum::<i64>();
            }
        };

        tail -= 1;
    }

    checksum
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    Input {
        blocks: lines
            .get(0)
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input, true);

        assert_eq!(result1, 1928);
    }

    #[test]
    fn test_part1_2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test2.txt"));
        let result1 = part1(&input, true);

        assert_eq!(result1, 5);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result2 = part2(&input);

        assert_eq!(result2, 2858);
    }
}
