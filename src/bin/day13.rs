//! Day 13: Distress Signal
//!
//! TIL: char has a .to_digit helper
//! TIL2: You need to implement both PartialOrd and Ord to use .sort
//! TIL3: I can return arrays inside of a flat_map, and it works
//! TIL4: Now I know what recursive descent parsing is
//! TIL5: Passing a mutable iterator for recursive parsing works well

use std::{cmp::Ordering, iter::Peekable, str::Chars};

use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();
    let packet_data = parse_input(&input);

    print_solution(
        12,
        1,
        "Determine which pairs of packets are already in the right order. What is the sum of the indices of those pairs?",
        format!("{}", sum_of_indices_of_valid_pairs(&packet_data))
    );

    print_solution(
        12,
        2,
        "What is the decoder key for the distress signal?",
        format!("{}", decoder_key(&packet_data)),
    );
}

// Day 13 part 1
fn sum_of_indices_of_valid_pairs(packet_data: &[(Data, Data)]) -> usize {
    packet_data
        .iter()
        .enumerate()
        // The list is 1-indexed
        .map(|(index, elem)| (index + 1, elem))
        // In part 2, we implemented Ord and PartialOrd for Data. This means
        // that packets with elements in the right order sort before packets
        // with elements in the wrong order. And so we can just use `left < right` 🎉
        .filter(|(_, (left, right))| left < right)
        .map(|(index, _)| index)
        .sum()
}

// Day 13 part 2
fn decoder_key(packet_data: &[(Data, Data)]) -> usize {
    let mut all_data = packet_data
        .iter()
        .flat_map(|(data1, data2)| [data1, data2])
        .cloned()
        .collect::<Vec<Data>>();

    let extra_element_2 = Data::List(vec![Data::List(vec![Data::Number(2)])]);
    let extra_element_6 = Data::List(vec![Data::List(vec![Data::Number(6)])]);

    all_data.push(extra_element_2.clone());
    all_data.push(extra_element_6.clone());
    all_data.sort();

    all_data
        .into_iter()
        .enumerate()
        .filter(|(_, data)| data == &extra_element_2 || data == &extra_element_6)
        .map(|(index, _)| index)
        // The list is 1-indexed
        .map(|index| index + 1)
        .product()
}

// Packet comparison logic
#[derive(Debug, PartialEq)]
enum PacketOrdering {
    /// This is used whenever we can't tell anything yet about the ordering
    Indecisive,
    CorrectOrder,
    IncorrectOrder,
}

fn compare_packets(left: &Data, right: &Data) -> PacketOrdering {
    match (left, right) {
        (Data::List(left), Data::List(right)) => compare_lists(left, right),
        (Data::Number(left), Data::Number(right)) => compare_number(left, right),

        // Wrap numbers in a list, for both sides:
        (Data::List(_), Data::Number(_)) => compare_packets(left, &wrap_in_list(right.clone())),
        (Data::Number(_), Data::List(_)) => compare_packets(&wrap_in_list(left.clone()), right),
    }
}

fn wrap_in_list(data: Data) -> Data {
    Data::List(vec![data])
}

fn compare_lists(left: &[Data], right: &[Data]) -> PacketOrdering {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    loop {
        // Instead of using .zip I had to manually drive the iterators to detect
        // which one runs out of elements first.
        match (left_iter.next(), right_iter.next()) {
            // Both lists output an element
            (Some(left_val), Some(right_val)) => match compare_packets(left_val, right_val) {
                PacketOrdering::Indecisive => continue,
                PacketOrdering::CorrectOrder => break PacketOrdering::CorrectOrder,
                PacketOrdering::IncorrectOrder => break PacketOrdering::IncorrectOrder,
            },
            // Left list ran out of elements first
            (None, Some(_)) => break PacketOrdering::CorrectOrder,
            // Right list ran out of elements first
            (Some(_), None) => break PacketOrdering::IncorrectOrder,
            // Both lists ran out of elements at the same time
            (None, None) => break PacketOrdering::Indecisive,
        }
    }
}

fn compare_number(left: &u8, right: &u8) -> PacketOrdering {
    match left.cmp(right) {
        Ordering::Equal => PacketOrdering::Indecisive,
        Ordering::Less => PacketOrdering::CorrectOrder,
        Ordering::Greater => PacketOrdering::IncorrectOrder,
    }
}

// Data structure ---

#[derive(Clone, Debug, PartialEq, Eq)]
enum Data {
    List(Vec<Data>),
    Number(u8),
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match compare_packets(self, other) {
            PacketOrdering::CorrectOrder => Some(Ordering::Less),
            PacketOrdering::IncorrectOrder => Some(Ordering::Greater),
            // If the ordering turns out to be indecisive, the two packets should be equal
            // I'm unsure though, and it never happens in my data
            PacketOrdering::Indecisive => Some(Ordering::Equal),
        }
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// Parsing ---

fn parse_input(input: &str) -> Vec<(Data, Data)> {
    input.split("\n\n").map(parse_pair).collect()
}

fn parse_pair(pair: &str) -> (Data, Data) {
    let inputs = pair.split('\n').collect::<Vec<&str>>();

    (
        parse_single(inputs.first().expect("No 1st element")),
        parse_single(inputs.get(1).expect("No 2nd element")),
    )
}

fn parse_single(single: &str) -> Data {
    let parsed = parse_data(&mut single.chars().peekable());

    // Flatten the outer list (which only has one element)
    parsed.first().unwrap().clone()
}

// For this implementation, I hadn't seen that a number can have 2 digits in the puzzle data,
// and it doesn't have that in the example data. So I needed to make it peekable at the last
// minute to make it work.
fn parse_data(input: &mut Peekable<Chars>) -> Vec<Data> {
    let mut result = Vec::new();

    while let Some(char) = input.next() {
        match char {
            '[' => result.push(Data::List(parse_data(input))),
            ']' => break,
            '0'..='9' => {
                // We only have 10s, never 11s
                let value = if let Some('0') = input.peek() {
                    10
                } else {
                    char.to_digit(10).unwrap() as u8
                };

                result.push(Data::Number(value))
            }
            ',' => continue,
            _ => panic!("Unknown input"),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day13.txt");

    #[test]
    fn day13_p1_example() {
        let packet_data = parse_input(EXAMPLE_INPUT);
        assert_eq!(sum_of_indices_of_valid_pairs(&packet_data), 13);
    }

    #[test]
    fn day13_p1_answer() {
        let packet_data = parse_input(PUZZLE_INPUT);
        assert_eq!(sum_of_indices_of_valid_pairs(&packet_data), 6395);
    }

    #[test]
    fn day13_p2_example() {
        let packet_data = parse_input(EXAMPLE_INPUT);
        assert_eq!(decoder_key(&packet_data), 140);
    }

    #[test]
    fn day13_p2_answer() {
        let packet_data = parse_input(PUZZLE_INPUT);
        assert_eq!(decoder_key(&packet_data), 24921);
    }
}
