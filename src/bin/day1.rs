use std::num::ParseIntError;

use twentytwo::{solution::print_solution, stdin::read_from_stdin};

type ElfWithCalories = Vec<u64>;
type ElvesWithCalories = Vec<ElfWithCalories>;

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
        "Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?",
        format!("{}", calorie_sum_of_3_elves_with_most_cals(&input)),
    );
}

/// D1P1
fn elf_with_most_calories(input: &str) -> u64 {
    let elves_with_calories: ElvesWithCalories = parse_input(input).expect("Failed to parse input");
    let elves_with_calorie_sums: Vec<u64> = elves_with_calorie_sums(elves_with_calories);

    elves_with_calorie_sums
        .into_iter()
        .max()
        .expect("Could not find max")
}

/// D1P2
fn calorie_sum_of_3_elves_with_most_cals(input: &str) -> u64 {
    let elves_with_calories: ElvesWithCalories = parse_input(input).expect("Failed to parse input");
    let mut elves_with_calorie_sums: Vec<u64> = elves_with_calorie_sums(elves_with_calories);
    elves_with_calorie_sums.sort();
    elves_with_calorie_sums.reverse();

    elves_with_calorie_sums.iter().take(3).sum()
}

fn elves_with_calorie_sums(elves: ElvesWithCalories) -> Vec<u64> {
    elves.iter().map(|elf| elf.iter().sum::<u64>()).collect()
}

fn parse_input(input: &str) -> Result<ElvesWithCalories, ParseIntError> {
    let trimmed_input = input.trim();

    trimmed_input
        .split("\n\n")
        .map(|elf_string| elf_string.split('\n').collect())
        .map(parse_elf_calories)
        .collect()
}

fn parse_elf_calories(input: Vec<&str>) -> Result<ElfWithCalories, ParseIntError> {
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
