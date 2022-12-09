use std::{cmp::Ordering, collections::HashSet, hash::Hash};

use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();

    print_solution(
        9,
        1,
        "Simulate your complete hypothetical series of motions. How many positions does the tail of the rope visit at least once?",
        format!("{}", solve_for_nibble_size_2(&input))
    );

    print_solution(
        9,
        2,
        "Simulate your complete series of motions on a larger rope with ten knots. How many positions does the tail of the rope visit at least once?",
        format!("{}", solve_for_nibble_size_10(&input))
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

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Position(i32, i32);

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

// Weeeeee, const generics! 🎉
struct Nibble<const C: usize>([Position; C]);

impl<const C: usize> Nibble<C> {
    fn new() -> Self {
        Nibble([Position::new(); C])
    }

    fn tail_position(&self) -> Position {
        self.0[C - 1]
    }

    fn move_head(&mut self, direction: &Direction) {
        self.0[0].step_in_direction(direction)
    }

    fn balance_tail(&mut self) {
        (1..C).for_each(|i| {
            let head_position = self.0[i - 1];

            if !(self.0[i]).touches_head_position(&head_position) {
                self.0[i].move_diagonally(&head_position);
            }
        })
    }
}

// D9P1
fn solve_for_nibble_size_2(input: &str) -> usize {
    let instructions = parse_input(input);

    let nibble: Nibble<2> = Nibble::new();
    count_visited_positions(instructions, nibble)
}

// D9P2
fn solve_for_nibble_size_10(input: &str) -> usize {
    let instructions = parse_input(input);

    let nibble: Nibble<10> = Nibble::new();
    count_visited_positions(instructions, nibble)
}

fn count_visited_positions<const C: usize>(
    instructions: Vec<Instruction>,
    mut nibble: Nibble<C>,
) -> usize {
    let mut visited_positions: HashSet<Position> = HashSet::new();

    for instruction in instructions {
        for _ in 0..instruction.amount {
            nibble.move_head(&instruction.direction);
            nibble.balance_tail();
            visited_positions.insert(nibble.tail_position());
        }
    }

    visited_positions.len()
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(Instruction::from)
        .collect::<Vec<Instruction>>()
}

#[cfg(test)]
mod tests {
    use crate::{solve_for_nibble_size_10, solve_for_nibble_size_2};

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day9.txt");

    const EXAMPLE_1: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    const EXAMPLE_2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    fn solves_p1_example() {
        assert_eq!(solve_for_nibble_size_2(EXAMPLE_1), 13);
    }

    #[test]
    fn solves_p1() {
        assert_eq!(solve_for_nibble_size_2(PUZZLE_INPUT), 5878);
    }

    #[test]
    fn solves_p2_example() {
        assert_eq!(solve_for_nibble_size_10(EXAMPLE_2), 36);
    }

    #[test]
    fn solves_p2() {
        assert_eq!(solve_for_nibble_size_10(PUZZLE_INPUT), 2405);
    }
}
