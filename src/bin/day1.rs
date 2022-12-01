use std::num::ParseIntError;

use twentytwo::stdin::read_from_stdin;

type ElfWithCalories = Vec<u64>;
type ElvesWithCalories = Vec<ElfWithCalories>;

fn main() {
    let input = read_from_stdin();
    let elves_with_calories: ElvesWithCalories = parse_input(input).expect("Failed to parse input");
    let max = elves_with_calories
        .iter()
        .map(|elf| elf.iter().sum::<u64>())
        .max()
        .expect("Could not find max");

    println!("D1P1 solution: {}", max);
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
