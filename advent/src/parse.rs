use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use crate::position::{Position, Position_isize};

pub struct Parser {
    input_file: String,
}

impl Parser {
    pub fn new(input_file: &str) -> Self {
        Self {
            input_file: input_file.to_string(),
        }
    }

    pub fn get_lines(&self) -> Vec<String> {
        let file = File::open(&self.input_file).expect("Failed to open file");
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|line| line.expect("Failed to read line"))
            .collect()
    }

    pub fn as_maze(&self) -> Maze {
        let lines = self.get_lines();
        let mut results = HashMap::new();

        lines.iter().enumerate().for_each(|(y, line)| {
            line.char_indices().for_each(|(x, c)| {
                let positions = results.entry(c).or_insert(HashSet::new());
                positions.insert(Position_isize {
                    x: x as isize,
                    y: y as isize,
                });
            });
        });

        Maze {
            walls: results.remove(&'#').unwrap(),
            special_chars: results,
        }
    }
}

pub struct Maze {
    pub walls: HashSet<Position_isize>,
    pub special_chars: HashMap<char, HashSet<Position_isize>>,
}

impl Maze {
    pub fn get_only_position(&self, c: char) -> Option<Position<isize>> {
        self.special_chars.get(&c)?.iter().next().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let parser = Parser::new(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/maze_test.txt"));
        let maze = parser.as_maze();

        assert_eq!(maze.walls.len(), 67);
        assert_eq!(
            *maze.special_chars.get(&'S').unwrap().iter().next().unwrap(),
            Position::<isize> { x: 1, y: 2 }
        );
        assert_eq!(
            *maze.special_chars.get(&'E').unwrap().iter().next().unwrap(),
            Position::<isize> { x: 24, y: 1 }
        );
    }
}
