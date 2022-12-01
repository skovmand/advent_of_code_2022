use std::num::ParseIntError;

use twentytwo::{solution::print_solution, stdin::read_from_stdin};

type CalorieCount = u64;
type ElfWithInventory = Vec<CalorieCount>;
type ElvesWithCalorieSums = Vec<CalorieCount>;

fn main() {
    let input = read_from_stdin();

    print_solution(
        1,
        1,
        "Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?",
        format!("{}", elf_with_most_calories(&input)),
    );

    print_solution(
        1,
        2,
        "Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?",
        format!("{}", calorie_sum_of_3_elves_with_most_cals(&input)),
    );
}

/// D1P1
fn elf_with_most_calories(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .max()
        .expect("Could not find max")
}

/// D1P2
fn calorie_sum_of_3_elves_with_most_cals(input: &str) -> u64 {
    let mut elves_with_calorie_sums: Vec<u64> = parse_input(input);
    elves_with_calorie_sums.sort();
    elves_with_calorie_sums.reverse();

    elves_with_calorie_sums.iter().take(3).sum()
}

fn parse_input(input: &str) -> ElvesWithCalorieSums {
    input
        .trim()
        .split("\n\n")
        .map(|elf_string| elf_string.split('\n').collect())
        .map(parse_elf_calories)
        .map(|elf_with_cals| elf_with_cals.map(|v| v.iter().sum::<u64>()))
        .collect::<Result<ElvesWithCalorieSums, ParseIntError>>()
        .expect("Failed to parse input")
}

fn parse_elf_calories(input: Vec<&str>) -> Result<ElfWithInventory, ParseIntError> {
    input
        .iter()
        .map(|cal| cal.parse::<u64>())
        .collect::<Result<Vec<u64>, std::num::ParseIntError>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day1.txt");

    #[test]
    fn solves_d1p1() {
        assert_eq!(elf_with_most_calories(PUZZLE_INPUT), 69528);
    }

    #[test]
    fn solves_d1p2() {
        assert_eq!(calorie_sum_of_3_elves_with_most_cals(PUZZLE_INPUT), 206152);
    }
}
