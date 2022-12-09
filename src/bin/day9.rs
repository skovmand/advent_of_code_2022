use std::{cmp::Ordering, collections::HashSet, hash::Hash};

use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();

    print_solution(
        9,
        1,
        "Simulate your complete hypothetical series of motions. How many positions does the tail of the rope visit at least once?",
        format!("{}", count_visited_positions(&input))
    );
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Instruction {
    direction: Direction,
    amount: u8,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let (str_direction, amount) = input.split_at(1);

        let direction = match str_direction {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Unknown direction"),
        };

        let amount = amount[1..].parse::<u8>().expect("parse amount");

        Instruction { direction, amount }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Position(i32, i64);

impl Position {
    fn new() -> Self {
        Position(0, 0)
    }

    fn step_in_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
        }
    }

    fn move_diagonally(&mut self, head_position: &Position) {
        self.0 = match self.0.cmp(&head_position.0) {
            Ordering::Less => self.0 + 1,
            Ordering::Equal => self.0,
            Ordering::Greater => self.0 - 1,
        };

        self.1 = match self.1.cmp(&head_position.1) {
            Ordering::Less => self.1 + 1,
            Ordering::Equal => self.1,
            Ordering::Greater => self.1 - 1,
        };
    }

    fn touches_head_position(&self, head_position: &Position) -> bool {
        let delta_x = (self.0 - head_position.0).abs();
        let delta_y = (self.1 - head_position.1).abs();

        (delta_x == 0 || delta_x == 1) && (delta_y == 0 || delta_y == 1)
    }
}

type PositionSet = HashSet<Position>;

fn count_visited_positions(input: &str) -> i64 {
    let instructions = parse_input(input);

    let (_, _, visited_positions): (Position, Position, PositionSet) = instructions.iter().fold(
        (Position::new(), Position::new(), HashSet::new()),
        |(mut head_position, mut tail_position, mut visited): (Position, Position, PositionSet),
         instruction| {
            (0..instruction.amount).for_each(|_| {
                head_position.step_in_direction(&instruction.direction);

                if !tail_position.touches_head_position(&head_position) {
                    tail_position.move_diagonally(&head_position);
                }

                visited.insert(tail_position);
            });

            (head_position, tail_position, visited)
        },
    );

    visited_positions.len() as i64
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(Instruction::from)
        .collect::<Vec<Instruction>>()
}

#[cfg(test)]
mod tests {
    use crate::count_visited_positions;

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day9.txt");

    const EXAMPLE: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    #[test]
    fn solves_p1_example() {
        assert_eq!(count_visited_positions(EXAMPLE), 13);
    }

    #[test]
    fn solves_p1() {
        assert_eq!(count_visited_positions(PUZZLE_INPUT), 5878);
    }
}
