#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use twentytwo::{print_solution, read_from_stdin, util::regex_capture_to_u8};

type Stacks = HashMap<u8, Vec<char>>;

#[derive(PartialEq)]
enum Crane {
    CrateMover9000,
    CrateMover9001,
}

#[derive(Debug)]
struct Instruction {
    amount: u8,
    from_stack: u8,
    to_stack: u8,
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new("^move (\\d+) from (\\d+) to (\\d+)$").unwrap();
        }

        let captures = REGEX.captures(value).expect("regex match");

        Ok(Instruction {
            amount: regex_capture_to_u8(&captures, 1),
            from_stack: regex_capture_to_u8(&captures, 2),
            to_stack: regex_capture_to_u8(&captures, 3),
        })
    }
}

fn main() {
    let input = read_from_stdin();

    print_solution(
        5,
        1,
        "After the rearrangement procedure completes, what crate ends up on top of each stack?",
        which_crate_on_top_of_each_stack(&input, Crane::CrateMover9000),
    );

    print_solution(
        5,
        2,
        "After the rearrangement procedure completes, what crate ends up on top of each stack? (using CrateMover 9001)",
        which_crate_on_top_of_each_stack(&input, Crane::CrateMover9001),
    );
}

// D5P1 + D5P2
fn which_crate_on_top_of_each_stack(input: &str, crane_model: Crane) -> String {
    let (stacks, instructions) = parse_input(input);

    let final_stack_state = instructions.iter().fold(stacks, |mut acc, ins| {
        let source_stack = acc.get(&ins.from_stack).unwrap();
        let (lifted, new_source_stack) = source_stack.split_at(ins.amount as usize);

        // Put the lifted crates at the top of the new destination stack
        let mut new_destination_stack = lifted.to_vec();

        // Only reverse the lifted crates in part 1
        if crane_model == Crane::CrateMover9000 {
            new_destination_stack.reverse();
        }

        // Move all crates from the old destination to the new one
        let mut destination_stack = acc.get(&ins.to_stack).unwrap().clone();
        new_destination_stack.append(&mut destination_stack);

        acc.insert(ins.from_stack, new_source_stack.to_vec());
        acc.insert(ins.to_stack, new_destination_stack);
        acc
    });

    let mut final_stacks = final_stack_state.iter().collect::<Vec<(&u8, &Vec<char>)>>();
    final_stacks.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

    final_stacks
        .iter()
        .map(|(_, stack)| stack.first().unwrap())
        .collect()
}

fn parse_input(input: &str) -> (Stacks, Vec<Instruction>) {
    let split_input = input.split("\n\n").collect::<Vec<&str>>();

    let stacks = parse_stacks(split_input.first().expect("Read stacks"));

    let instructions = split_input
        .get(1)
        .expect("Read instructions")
        .lines()
        .map(Instruction::try_from)
        .collect::<Result<Vec<Instruction>, ()>>()
        .expect("Parse instructions");

    (stacks, instructions)
}

fn parse_stacks(stack_input: &str) -> HashMap<u8, Vec<char>> {
    let character_map = character_map(stack_input);

    left_rotate(&character_map)
        .iter()
        .filter(|s| !(s.contains('[') || s.contains(']') || s.trim().is_empty()))
        .map(|s| s.trim().chars().rev().collect::<String>())
        .map(|s| {
            (
                s[0..1].parse::<u8>().unwrap(),
                s[1..].chars().rev().collect::<Vec<char>>(),
            )
        })
        .collect()
}

fn character_map(stack_input: &str) -> HashMap<(usize, usize), char> {
    stack_input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x, y), char))
        })
        .collect()
}

fn get_dimensions(character_map: &HashMap<(usize, usize), char>) -> (usize, usize) {
    let max_x = character_map.iter().map(|(t, _)| t.0).max().expect("Max x");
    let max_y = character_map.iter().map(|(t, _)| t.1).max().expect("Max y");

    (max_x, max_y)
}

fn left_rotate(character_map: &HashMap<(usize, usize), char>) -> Vec<String> {
    let (max_x, max_y) = get_dimensions(character_map);

    (0..=max_x)
        .map(|x| {
            (0..=max_y)
                .map(|y| character_map.get(&(x, y)).expect("get coord"))
                .collect::<String>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{which_crate_on_top_of_each_stack, Crane};

    const EXAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day5.txt");

    #[test]
    fn solves_p1_example() {
        assert_eq!(
            which_crate_on_top_of_each_stack(EXAMPLE, Crane::CrateMover9000),
            "CMZ"
        );
    }

    #[test]
    fn solves_p1() {
        assert_eq!(
            which_crate_on_top_of_each_stack(PUZZLE_INPUT, Crane::CrateMover9000),
            "VRWBSFZWM"
        );
    }

    #[test]
    fn solves_p2_example() {
        assert_eq!(
            which_crate_on_top_of_each_stack(EXAMPLE, Crane::CrateMover9001),
            "MCD"
        );
    }

    #[test]
    fn solves_p2() {
        assert_eq!(
            which_crate_on_top_of_each_stack(PUZZLE_INPUT, Crane::CrateMover9001),
            "RBTWJWMCF"
        );
    }
}
