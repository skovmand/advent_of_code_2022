use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();

    print_solution(
        3,
        1,
        "Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?",
        format!("{}", priority_sum(&input)),
    );
}

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> &[u8] {
    input.as_bytes()
}

// D3P1
fn priority_sum(input: &str) -> u64 {
    parse_input(input)
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
        assert_eq!(priority_sum(EXAMPLE), 157);
    }

    #[test]
    fn solves_p1() {
        assert_eq!(priority_sum(PUZZLE_INPUT), 8088);
    }
}
