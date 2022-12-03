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

#[derive(Copy, Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl TryFrom<u8> for Outcome {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(Outcome::Lose),
            b'Y' => Ok(Outcome::Draw),
            b'Z' => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

/// A full game with OpponentChoice, OwnChoice
struct GameWithChoices(Choice, Choice);

impl TryFrom<(u8, u8)> for GameWithChoices {
    type Error = ();

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        let opponent_choice = Choice::try_from(value.0)?;
        let own_choice = Choice::try_from(value.1)?;

        Ok(GameWithChoices(opponent_choice, own_choice))
    }
}

/// A full game with OpponentChoice and the Game Outcome
struct GameWithOutcome(Choice, Outcome);

impl TryFrom<(u8, u8)> for GameWithOutcome {
    type Error = ();

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        let opponent_choice = Choice::try_from(value.0)?;
        let outcome = Outcome::try_from(value.1)?;

        Ok(GameWithOutcome(opponent_choice, outcome))
    }
}

fn main() {
    let input = read_from_stdin();

    print_solution(
        2,
        1,
        "What would your total score be if everything goes exactly according to your strategy guide?",
        format!("{}", total_score_from_strategy_guide_with_choice(&input)),
    );

    print_solution(
        2,
        2,
        "Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?",
        format!("{}", total_score_from_strategy_guide_with_outcome(&input)),
    );
}

/// D2P1
fn total_score_from_strategy_guide_with_choice(input: &str) -> u64 {
    parse_input_with_own_choice(input)
        .expect("parse input")
        .iter()
        .map(score_from_choices)
        .sum()
}

fn parse_input_with_own_choice(input: &str) -> Result<Vec<GameWithChoices>, ()> {
    input.lines().map(parse_line_with_own_choice).collect()
}

fn parse_line_with_own_choice(line: &str) -> Result<GameWithChoices, ()> {
    let bytes = line.as_bytes();
    GameWithChoices::try_from((bytes[0], bytes[2]))
}

fn score_from_choices(round: &GameWithChoices) -> u64 {
    let outcome = match (round.1, round.0) {
        (Choice::Rock, Choice::Rock) => Outcome::Draw,
        (Choice::Rock, Choice::Paper) => Outcome::Lose,
        (Choice::Rock, Choice::Scissors) => Outcome::Win,
        (Choice::Paper, Choice::Rock) => Outcome::Win,
        (Choice::Paper, Choice::Paper) => Outcome::Draw,
        (Choice::Paper, Choice::Scissors) => Outcome::Lose,
        (Choice::Scissors, Choice::Rock) => Outcome::Lose,
        (Choice::Scissors, Choice::Paper) => Outcome::Win,
        (Choice::Scissors, Choice::Scissors) => Outcome::Draw,
    };

    pick_score(round.1) + outcome_score(outcome)
}

/// D2P2
fn total_score_from_strategy_guide_with_outcome(input: &str) -> u64 {
    parse_input_with_outcome(input)
        .expect("parse input")
        .iter()
        .map(score_from_outcome)
        .sum()
}

fn parse_input_with_outcome(input: &str) -> Result<Vec<GameWithOutcome>, ()> {
    input.lines().map(parse_line_with_outcome).collect()
}

fn parse_line_with_outcome(line: &str) -> Result<GameWithOutcome, ()> {
    let bytes = line.as_bytes();
    GameWithOutcome::try_from((bytes[0], bytes[2]))
}

fn score_from_outcome(round: &GameWithOutcome) -> u64 {
    let own_pick = match (round.0, round.1) {
        (Choice::Rock, Outcome::Lose) => Choice::Scissors,
        (Choice::Rock, Outcome::Draw) => Choice::Rock,
        (Choice::Rock, Outcome::Win) => Choice::Paper,
        (Choice::Paper, Outcome::Lose) => Choice::Rock,
        (Choice::Paper, Outcome::Draw) => Choice::Paper,
        (Choice::Paper, Outcome::Win) => Choice::Scissors,
        (Choice::Scissors, Outcome::Lose) => Choice::Paper,
        (Choice::Scissors, Outcome::Draw) => Choice::Scissors,
        (Choice::Scissors, Outcome::Win) => Choice::Rock,
    };

    pick_score(own_pick) + outcome_score(round.1)
}

fn outcome_score(outcome: Outcome) -> u64 {
    match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn pick_score(choice: Choice) -> u64 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day2.txt");

    #[test]
    fn solves_p1_example() {
        let example = "A Y\nB X\nC Z";
        let score = total_score_from_strategy_guide_with_choice(example);

        assert_eq!(score, 15);
    }

    #[test]
    fn solves_p2_example() {
        let example = "A Y\nB X\nC Z";
        let score = total_score_from_strategy_guide_with_outcome(example);

        assert_eq!(score, 12);
    }

    #[test]
    fn solves_p1() {
        let score = total_score_from_strategy_guide_with_choice(PUZZLE_INPUT);

        assert_eq!(score, 11767);
    }

    #[test]
    fn solves_p2() {
        let score = total_score_from_strategy_guide_with_outcome(PUZZLE_INPUT);

        assert_eq!(score, 13886);
    }
}
