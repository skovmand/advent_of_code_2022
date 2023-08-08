//! Day 14: Regolith Reservoir
//!
//! TIL: Calling .as_slice() on a Vec enables pattern matching on its elements!
//! TIL2: Already knew this, but nice to once again see that using iterators makes things much easier to consume
//! TIL3: You can declare an enum inside a function. Clever if it's only used there!

use std::{
    cmp::{max, min},
    collections::HashMap,
};

use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();
    let rock_paths = parse_input(&input);
    let rock_structure = build_rock_structure(rock_paths, false);

    print_solution(
        14,
        1,
        "Using your scan, simulate the falling sand. How many units of sand come to rest before sand starts flowing into the abyss below?",
        format!("{}", count_sands_that_come_to_rest(rock_structure.clone()))
    );

    // Now set a floor
    let mut rock_structure = rock_structure;
    rock_structure.has_floor = true;

    print_solution(
        14,
        2,
        "Using your scan, simulate the falling sand until the source of the sand becomes blocked. How many units of sand come to rest?",
        format!("{}", count_sands_before_entry_is_filled(rock_structure))
    );
}

const ENTRY_HOLE: Position = Position { row: 0, column: 500 };

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Clone, Debug)]
enum Element {
    Rock,
    Sand,
}

#[derive(Clone, Debug)]
struct RockStructure {
    data: HashMap<Position, Element>,
    max_y: usize,
    has_floor: bool,
}

impl RockStructure {
    /// Custom check for if a field is filled (because then we can make an infinite floor!)
    /// If the y is at the floor level, it is always filled
    fn is_filled(&self, position: Position) -> bool {
        (self.has_floor && position.row == self.max_y + 2) || self.data.get(&position).is_some()
    }

    /// Put sand at a position in the rock structure
    fn put_sand(&mut self, position: Position) {
        self.data.insert(position, Element::Sand);
    }

    /// Check if a position is in-bounds (we only need to check max_y, since min_y is 0)
    fn position_in_bounds(&self, position: Position) -> bool {
        if self.has_floor {
            position.row <= self.max_y + 2
        } else {
            position.row <= self.max_y
        }
    }

    /// Given the current position, get the next position that the sand can flow to
    fn next_sand_position(&self, position: Position) -> Option<Position> {
        let down = Position {
            row: position.row + 1,
            column: position.column,
        };

        let down_left = Position {
            row: position.row + 1,
            column: position.column - 1,
        };

        let down_right = Position {
            row: position.row + 1,
            column: position.column + 1,
        };

        match (
            self.is_filled(down),
            self.is_filled(down_left),
            self.is_filled(down_right),
        ) {
            (false, _, _) => Some(down),
            (true, false, _) => Some(down_left),
            (true, true, false) => Some(down_right),
            (true, true, true) => None,
        }
    }

    /// Pour sand from the entry point, return the position where the sand comes to rest
    fn pour_sand(&mut self) -> Option<Position> {
        let mut sand_position = ENTRY_HOLE;

        loop {
            if let Some(next_position) = self.next_sand_position(sand_position) {
                if self.position_in_bounds(next_position) {
                    // The sand can move downwards more
                    sand_position = next_position;
                } else {
                    // The position is out of bounds, the sand never comes to rest
                    break None;
                }
            } else {
                // The sand has come to rest
                self.put_sand(sand_position);
                break Some(sand_position);
            }
        }
    }
}

// If the sand-pouring is an iterator, it will be much nicer to consume!
// As a bonus, it's really easy to build too, since pour_sand emits an Option<Position>
impl Iterator for RockStructure {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.pour_sand()
    }
}

/// D14P1
fn count_sands_that_come_to_rest(rock_structure: RockStructure) -> usize {
    rock_structure.count()
}

/// D14P2
fn count_sands_before_entry_is_filled(rock_structure: RockStructure) -> usize {
    rock_structure
        .enumerate()
        .find(|(_, pos)| *pos == ENTRY_HOLE)
        .map(|(count, _)| count + 1)
        .expect("Could not find any sand covering the entry hole")
}

// Parsing:

fn parse_input(input: &str) -> Vec<Vec<Position>> {
    input.lines().map(parse_path_line).collect()
}

fn parse_path_line(line_input: &str) -> Vec<Position> {
    line_input.split(" -> ").map(parse_position).collect()
}

fn parse_position(position_input: &str) -> Position {
    match position_input
        .split(',')
        .map(|pos_str| pos_str.parse::<usize>().expect("unparseable coordinate"))
        .collect::<Vec<usize>>()
        .as_slice()
    {
        [a, b] => Position { column: *a, row: *b },
        _ => panic!("position doesn't have 2 coordinates"),
    }
}

fn build_rock_structure(rock_paths: Vec<Vec<Position>>, has_floor: bool) -> RockStructure {
    enum RockDirection {
        Vertical,
        Horizontal,
    }

    let rock_structure: HashMap<Position, Element> = rock_paths.iter().fold(HashMap::new(), |mut acc, rock_path| {
        // Zip in pairs
        let rock_path_2 = (*rock_path).clone();

        // Insert paths
        rock_path
            .iter()
            .zip(rock_path_2.iter().skip(1))
            .fold(&mut acc, |acc, (source, dest)| {
                // Figure out if the path is vertical or horizontal
                let direction = match (source.column.abs_diff(dest.column), source.row.abs_diff(dest.row)) {
                    (0, _) => RockDirection::Vertical,
                    (_, 0) => RockDirection::Horizontal,
                    (_, _) => panic!("Both column and row cannot change at the same time"),
                };

                match direction {
                    RockDirection::Vertical => {
                        (min(source.row, dest.row)..=max(source.row, dest.row)).for_each(|row| {
                            acc.insert(
                                Position {
                                    row,
                                    column: source.column,
                                },
                                Element::Rock,
                            );
                        })
                    }
                    RockDirection::Horizontal => (min(source.column, dest.column)..=max(source.column, dest.column))
                        .for_each(|column| {
                            acc.insert(
                                Position {
                                    column,
                                    row: source.row,
                                },
                                Element::Rock,
                            );
                        }),
                }

                acc
            });

        acc
    });

    let max_y = rock_structure.keys().map(|key| key.row).max().unwrap();

    RockStructure {
        data: rock_structure,
        max_y,
        has_floor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day14.txt");

    #[test]
    fn d14_p1_example() {
        let rock_paths = parse_input(EXAMPLE_INPUT);
        let rock_structure = build_rock_structure(rock_paths, false);
        let sands_that_come_to_rest = count_sands_that_come_to_rest(rock_structure);

        assert_eq!(sands_that_come_to_rest, 24);
    }

    #[test]
    fn d14_p1_solution() {
        let rock_paths = parse_input(PUZZLE_INPUT);
        let rock_structure = build_rock_structure(rock_paths, false);
        let sands_that_come_to_rest = count_sands_that_come_to_rest(rock_structure);

        assert_eq!(sands_that_come_to_rest, 873);
    }

    #[test]
    fn d14_p2_example() {
        let rock_paths = parse_input(EXAMPLE_INPUT);
        let rock_structure = build_rock_structure(rock_paths, true);
        let sands_before_entry_is_filled = count_sands_before_entry_is_filled(rock_structure);

        assert_eq!(sands_before_entry_is_filled, 93);
    }

    #[test]
    fn d14_p2_solution() {
        let rock_paths = parse_input(PUZZLE_INPUT);
        let rock_structure = build_rock_structure(rock_paths, true);
        let sands_before_entry_is_filled = count_sands_before_entry_is_filled(rock_structure);

        assert_eq!(sands_before_entry_is_filled, 24813);
    }
}
