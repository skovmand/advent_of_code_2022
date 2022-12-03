use itertools::{Chunk, Itertools};
use std::vec::IntoIter;
use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();

    print_solution(
        3,
        1,
        "Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?",
        format!("{}", priority_sum_of_supplies(&input)),
    );

    print_solution(
        3,
        2,
        "Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?",
        format!("{}", priority_sum_of_badges(&input)),
    );
}

// D3P1
fn priority_sum_of_supplies(input: &str) -> u64 {
    parse_rucksacks(input)
        .into_iter()
        .map(split_rucksack)
        .map(element_in_both_compartments)
        .map(to_priority_score)
        .map(u64::from)
        .sum()
}

fn split_rucksack(rucksack: &[u8]) -> (&[u8], &[u8]) {
    rucksack.split_at(rucksack.len() / 2)
}

fn element_in_both_compartments((compartment_a, compartment_b): (&[u8], &[u8])) -> u8 {
    *compartment_a
        .iter()
        .find(|v: &&u8| compartment_b.contains(v))
        .expect("Common element")
}

// D3P2
fn priority_sum_of_badges(input: &str) -> u64 {
    parse_rucksacks(input)
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(badge_element)
        .map(to_priority_score)
        .map(u64::from)
        .sum()
}

fn badge_element(rucksack_chunk: Chunk<IntoIter<&[u8]>>) -> u8 {
    match rucksack_chunk.collect::<Vec<&[u8]>>().as_slice() {
        [a, b, c] => *a
            .iter()
            .find(|elem| b.contains(elem) && c.contains(elem))
            .expect("Common badge"),
        _ => panic!("Unexpected rucksack count"),
    }
}

// Common
fn parse_rucksacks(input: &str) -> Vec<&[u8]> {
    input.lines().map(parse_rucksack).collect()
}

fn parse_rucksack(input: &str) -> &[u8] {
    input.as_bytes()
}

fn to_priority_score(element: u8) -> u8 {
    if (97..=122).contains(&element) {
        // Transform a-z to range 1-26
        element - 96
    } else if (65..=90).contains(&element) {
        // Transform A-Z to range 27-52
        element - 38
    } else {
        panic!("Unknown rucksack item");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day3.txt");

    #[test]
    fn solves_p1_example() {
        assert_eq!(priority_sum_of_supplies(EXAMPLE), 157);
    }

    #[test]
    fn solves_p1() {
        assert_eq!(priority_sum_of_supplies(PUZZLE_INPUT), 8088);
    }

    #[test]
    fn solves_p2_example() {
        assert_eq!(priority_sum_of_badges(EXAMPLE), 70);
    }

    #[test]
    fn solves_p2() {
        assert_eq!(priority_sum_of_badges(PUZZLE_INPUT), 2522);
    }
}
