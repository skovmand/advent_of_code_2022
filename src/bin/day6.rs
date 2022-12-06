use itertools::Itertools;
use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();

    print_solution(
        6,
        1,
        "How many characters need to be processed before the first start-of-packet marker is detected?",
        format!("{}", characters_before_marker_end(&input))
    );

    print_solution(
        6,
        2,
        "How many characters need to be processed before the first start-of-message marker is detected?",
        format!("{}", characters_before_message_marker(&input))
    );
}

fn characters_before_marker_end(input: &str) -> usize {
    count_until_unique_block_end(input, 4)
}

fn characters_before_message_marker(input: &str) -> usize {
    count_until_unique_block_end(input, 14)
}

fn count_until_unique_block_end(input: &str, unique_count: usize) -> usize {
    let (left_index, _) = input
        .as_bytes()
        .windows(unique_count)
        .enumerate()
        .find(|(_, bytes)| bytes.iter().unique().count() == unique_count)
        .expect("First unique 4 chars");

    left_index + unique_count
}

#[cfg(test)]
mod tests {
    use crate::{characters_before_marker_end, characters_before_message_marker};

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day6.txt");

    #[test]
    fn solves_p1_examples() {
        let input: Vec<(&str, usize)> = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        input.iter().for_each(|(input, expected_result)| {
            let result = characters_before_marker_end(input);
            assert_eq!(&result, expected_result);
        });
    }

    #[test]
    fn solves_p2_examples() {
        let input: Vec<(&str, usize)> = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        input.iter().for_each(|(input, expected_result)| {
            let result = characters_before_message_marker(input);
            assert_eq!(&result, expected_result);
        });
    }

    #[test]
    fn solves_p1() {
        assert_eq!(characters_before_marker_end(PUZZLE_INPUT), 1833);
    }

    #[test]
    fn solves_p2() {
        assert_eq!(characters_before_message_marker(PUZZLE_INPUT), 3425);
    }
}
