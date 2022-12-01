use std::num::ParseIntError;

use twentytwo::stdin::read_from_stdin;

type ElfWithCalories = Vec<u64>;
type ElvesWithCalories = Vec<ElfWithCalories>;

fn main() {
    let input = read_from_stdin();
    let elves_with_calories: ElvesWithCalories = parse_input(input).expect("Failed to parse input");
    let elves_with_calorie_sums: Vec<u64> = elves_with_calories
        .iter()
        .map(|elf| elf.iter().sum::<u64>())
        .collect();

    let max = elves_with_calorie_sums
        .iter()
        .max()
        .expect("Could not find max");

    println!("D1P1 solution: {}", max);

    let mut sorted_elves_with_calorie_sums = elves_with_calorie_sums;
    sorted_elves_with_calorie_sums.sort();
    sorted_elves_with_calorie_sums.reverse();

    let sum_of_three_largest: u64 = sorted_elves_with_calorie_sums.iter().take(3).sum();

    println!("D1P2 solution: {}", sum_of_three_largest);
}

fn parse_input(input: String) -> Result<ElvesWithCalories, ParseIntError> {
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
