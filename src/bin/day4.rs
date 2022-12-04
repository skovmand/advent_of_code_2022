#[macro_use]
extern crate lazy_static;

use regex::{Captures, Regex};
use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();

    print_solution(
        4,
        1,
        "In how many assignment pairs does one range fully contain the other?",
        format!("{}", count_full_overlaps(&input)),
    );

    print_solution(
        4,
        2,
        "In how many assignment pairs do the ranges overlap?",
        format!("{}", count_partial_overlaps(&input)),
    );
}

struct Range(u8, u8);

impl Range {
    fn contained_in_range(&self, other_range: &Range) -> bool {
        self.0 >= other_range.0 && self.1 <= other_range.1
    }

    fn overlaps_with_range(&self, other_range: &Range) -> bool {
        !(other_range.0 > self.1 || other_range.1 < self.0)
    }
}

impl From<(u8, u8)> for Range {
    fn from(input: (u8, u8)) -> Self {
        Range(input.0, input.1)
    }
}

// D4P1
fn count_full_overlaps(input: &str) -> u64 {
    input
        .lines()
        .map(to_ranges)
        .filter(|(r1, r2)| r1.contained_in_range(r2) || r2.contained_in_range(r1))
        .count() as u64
}

// D4P2
fn count_partial_overlaps(input: &str) -> u64 {
    input
        .lines()
        .map(to_ranges)
        .filter(|(r1, r2)| r1.overlaps_with_range(r2))
        .count() as u64
}

fn to_ranges(line: &str) -> (Range, Range) {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("^(\\d+)-(\\d+),(\\d+)-(\\d+)$").unwrap();
    }

    let captures = REGEX.captures(line).expect("regex match");

    (
        Range::from((capture_to_u8(&captures, 1), capture_to_u8(&captures, 2))),
        Range::from((capture_to_u8(&captures, 3), capture_to_u8(&captures, 4))),
    )
}

fn capture_to_u8(captures: &Captures, index: usize) -> u8 {
    captures
        .get(index)
        .unwrap()
        .as_str()
        .parse::<u8>()
        .expect("parse capture to u8")
}

#[cfg(test)]
mod tests {
    use crate::{count_full_overlaps, count_partial_overlaps};

    const EXAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day4.txt");

    #[test]
    fn solves_p1_example() {
        assert_eq!(count_full_overlaps(EXAMPLE), 2);
    }

    #[test]
    fn solves_p1() {
        assert_eq!(count_full_overlaps(PUZZLE_INPUT), 550);
    }

    #[test]
    fn solves_p2_example() {
        assert_eq!(count_partial_overlaps(EXAMPLE), 4);
    }

    #[test]
    fn solves_p2() {
        assert_eq!(count_partial_overlaps(PUZZLE_INPUT), 931);
    }
}
