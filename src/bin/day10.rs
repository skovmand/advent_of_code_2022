use itertools::{unfold, Itertools};
use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();

    print_solution(
        10,
        1,
        "Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. What is the sum of these six signal strengths?",
        format!("{}", signal_sum(&input)),
    );

    print_solution(
        10,
        2,
        "Render the image given by your program. What eight capital letters appear on your CRT?",
        render_crt(&input),
    );
}

// D10P1
fn signal_sum(input: &str) -> i64 {
    let instructions = parse_input(input);
    let mut signal_strengths = signal_strengths(instructions);

    [
        (20, 20),
        (40, 60),
        (40, 100),
        (40, 140),
        (40, 180),
        (40, 220),
    ]
    .iter()
    .map(|(steps, step_sum)| step_sum * signal_strengths.nth(steps - 1).expect("iter pos"))
    .sum()
}

// D10P2
fn render_crt(input: &str) -> String {
    let instructions = parse_input(input);
    let signal_strengths = signal_strengths(instructions);

    signal_strengths
        .chunks(40)
        .into_iter()
        .map(|chunk| {
            chunk
                .enumerate()
                .map(|(i, strength)| {
                    let diff = (strength - (i as i64)).abs();

                    if diff == 0 || diff == 1 {
                        "#"
                    } else {
                        "."
                    }
                })
                .collect::<String>()
        })
        .map(|line| format!("{}\n", line))
        .collect()
}

// My first use of unfold 🎉
// Creates an iterator of signal strengths from a list of instructions
fn signal_strengths(instructions: Vec<Instruction>) -> impl Iterator<Item = i64> {
    unfold((0_usize, 1), move |(i, strength)| {
        match &instructions.get(*i) {
            Some(Instruction::AddX(value)) => {
                let return_value = vec![*strength, *strength];
                *strength += value;
                *i += 1;

                Some(return_value)
            }
            Some(Instruction::NoOp) => {
                *i += 1;
                Some(vec![*strength])
            }
            None => None,
        }
    })
    .flatten()
}

// Parsing ----->

enum Instruction {
    AddX(i64),
    NoOp,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if let Some(amount) = value.strip_prefix("addx ") {
            let amount_i32 = amount.parse::<i64>().expect("parse amount");
            Instruction::AddX(amount_i32)
        } else if value == "noop" {
            Instruction::NoOp
        } else {
            println!("failed to parse: '{}'", value);
            panic!("parse instructions");
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

#[cfg(test)]
mod tests {
    use crate::{render_crt, signal_sum};

    const EXAMPLE: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    const EXAMPLE_IMAGE: &str = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day10.txt");

    const PUZZLE_ANSWER: &str = r#"###..#..#..##...##...##..###..#..#.####.
#..#.#..#.#..#.#..#.#..#.#..#.#..#....#.
###..#..#.#....#..#.#....###..#..#...#..
#..#.#..#.#....####.#....#..#.#..#..#...
#..#.#..#.#..#.#..#.#..#.#..#.#..#.#....
###...##...##..#..#..##..###...##..####.
"#;

    #[test]
    fn solves_p1_example() {
        assert_eq!(signal_sum(EXAMPLE), 13140);
    }

    #[test]
    fn solves_p1() {
        assert_eq!(signal_sum(PUZZLE_INPUT), 14920);
    }

    #[test]
    fn solves_p2_example() {
        assert_eq!(render_crt(EXAMPLE), EXAMPLE_IMAGE);
    }

    #[test]
    fn solves_p2() {
        assert_eq!(render_crt(PUZZLE_INPUT), PUZZLE_ANSWER);
    }
}
