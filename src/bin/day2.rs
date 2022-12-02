use twentytwo::{print_solution, read_from_stdin};

#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<u8> for Choice {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => Ok(Choice::Rock),
            b'B' => Ok(Choice::Paper),
            b'C' => Ok(Choice::Scissors),
            b'X' => Ok(Choice::Rock),
            b'Y' => Ok(Choice::Paper),
            b'Z' => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

struct Round(Choice, Choice);

impl TryFrom<(u8, u8)> for Round {
    type Error = ();

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        let opponent_choice = Choice::try_from(value.0)?;
        let own_choice = Choice::try_from(value.1)?;

        Ok(Round(opponent_choice, own_choice))
    }
}

fn main() {
    let input = read_from_stdin();

    print_solution(
        2,
        1,
        "What would your total score be if everything goes exactly according to your strategy guide?",
        format!("{}", total_score_from_strategy_guide(&input)),
    );
}

fn total_score_from_strategy_guide(input: &str) -> u64 {
    parse_input(input)
        .expect("parse input")
        .iter()
        .map(to_score)
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Round>, ()> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Round, ()> {
    let bytes = line.as_bytes();
    Round::try_from((bytes[0], bytes[2]))
}

fn to_score(round: &Round) -> u64 {
    let pick_score = match round.1 {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    let outcome_score = match (round.1, round.0) {
        (Choice::Rock, Choice::Rock) => 3,
        (Choice::Rock, Choice::Paper) => 0,
        (Choice::Rock, Choice::Scissors) => 6,
        (Choice::Paper, Choice::Rock) => 6,
        (Choice::Paper, Choice::Paper) => 3,
        (Choice::Paper, Choice::Scissors) => 0,
        (Choice::Scissors, Choice::Rock) => 0,
        (Choice::Scissors, Choice::Paper) => 6,
        (Choice::Scissors, Choice::Scissors) => 3,
    };

    pick_score + outcome_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day2.txt");

    #[test]
    fn solves_p1_example() {
        let example = "A Y\nB X\nC Z";
        let score = total_score_from_strategy_guide(example);

        assert_eq!(score, 15);
    }

    #[test]
    fn solves_p1() {
        let score = total_score_from_strategy_guide(PUZZLE_INPUT);

        assert_eq!(score, 11767);
    }
}
