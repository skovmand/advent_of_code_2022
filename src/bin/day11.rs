// I learned today to use BigInts, and that while let Some(x) can be very handy 🎉
// I also learned at the end that I didn't have to use BigInts :-D
// BigInts were very handy - I could do all the same operations with them as regular u64 and u128s!

use std::collections::{HashMap, VecDeque};
use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();

    print_solution(
        11,
        1,
        "What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?",
        format!("{}", calculate_monkey_business(&input, 20, ReduceMode::DivideByThree)),
    );

    print_solution(
        11,
        2,
        "Starting again from the initial state in your puzzle input, what is the level of monkey business after 10000 rounds?",
        format!("{}", calculate_monkey_business(&input, 10_000, ReduceMode::ModulusByModuliProduct))
    );
}

enum ReduceMode {
    DivideByThree,
    ModulusByModuliProduct,
}

fn calculate_monkey_business(input: &str, iterations: usize, mode: ReduceMode) -> u64 {
    let mut state = parse_input(input);
    let monkey_count = state.len();
    let monkey_moduli_product: u64 = state.iter().map(|s| s.1.test_value).product();

    for _ in 0..iterations {
        for current_monkey_id in 0..monkey_count {
            let mut current_monkey = state.get(&current_monkey_id).expect("get monkey").clone();

            while let Some(item) = current_monkey.take_item() {
                let new_worry_level = match current_monkey.operator {
                    Operator::Add => match current_monkey.operand2 {
                        Operand::Value(value) => item + value,
                        Operand::Old => item + item,
                    },
                    Operator::Multiply => match current_monkey.operand2 {
                        Operand::Value(value) => item * value,
                        Operand::Old => item * item,
                    },
                };

                let worry_level_after_monkey_gets_bored = match mode {
                    ReduceMode::DivideByThree => new_worry_level / 3,
                    ReduceMode::ModulusByModuliProduct => new_worry_level % monkey_moduli_product,
                };

                let mut destination_monkey: MonkeyState =
                    if worry_level_after_monkey_gets_bored % current_monkey.test_value == 0 {
                        state
                            .get(&current_monkey.target_when_true)
                            .expect("get target monkey when true")
                            .clone()
                    } else {
                        state
                            .get(&current_monkey.target_when_false)
                            .expect("get target monkey when false")
                            .clone()
                    };

                destination_monkey.receive_item(worry_level_after_monkey_gets_bored);
                state.insert(destination_monkey.id, destination_monkey);
            }

            state.insert(current_monkey_id, current_monkey);
        }
    }

    let mut all_items_inspected: Vec<u64> = state.iter().map(|t| t.1.items_inspected).collect();
    all_items_inspected.sort();
    all_items_inspected.reverse();

    all_items_inspected[0] * all_items_inspected[1]
}

#[derive(Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Clone)]
enum Operand {
    Value(u64),
    Old,
}

#[derive(Clone)]
struct MonkeyState {
    id: usize,
    items: VecDeque<u64>,
    items_inspected: u64,
    operator: Operator,
    operand2: Operand,
    test_value: u64,
    target_when_true: usize,
    target_when_false: usize,
}

impl MonkeyState {
    fn take_item(&mut self) -> Option<u64> {
        if let Some(item) = self.items.pop_front() {
            self.items_inspected += 1;
            Some(item)
        } else {
            None
        }
    }

    fn receive_item(&mut self, item: u64) {
        self.items.push_back(item)
    }
}

// Parsing -->

fn parse_input(input: &str) -> HashMap<usize, MonkeyState> {
    input
        .split("\n\n")
        .map(parse_monkey_state)
        .map(|state| (state.id, state))
        .collect()
}

fn parse_monkey_state(input: &str) -> MonkeyState {
    let lines: Vec<&str> = input.lines().collect();

    let id = lines[0][7..8].parse::<usize>().expect("parse id");

    let items: VecDeque<u64> = lines[1]
        .strip_prefix("  Starting items: ")
        .expect("match starting items")
        .split(", ")
        .map(|item_str| item_str.parse::<u64>().expect("parse_item"))
        .collect();

    let operation_raw = lines[2]
        .strip_prefix("  Operation: new = old ")
        .expect("match operation");

    let operator = match &operation_raw[0..1] {
        "+" => Operator::Add,
        "*" => Operator::Multiply,
        _ => panic!("unknown operator"),
    };

    let operand2 = match &operation_raw[2..] {
        "old" => Operand::Old,
        val => Operand::Value(val.parse::<u64>().expect("parse operand 2")),
    };

    let test_value = lines[3]
        .strip_prefix("  Test: divisible by ")
        .expect("match test")
        .parse::<u64>()
        .expect("parse divisible by");

    let target_when_true = lines[4]
        .strip_prefix("    If true: throw to monkey ")
        .expect("match true target")
        .parse::<usize>()
        .expect("parse true target");

    let target_when_false = lines[5]
        .strip_prefix("    If false: throw to monkey ")
        .expect("match false target")
        .parse::<usize>()
        .expect("parse false target");

    MonkeyState {
        id,
        items,
        items_inspected: 0,
        operator,
        operand2,
        test_value,
        target_when_true,
        target_when_false,
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate_monkey_business, ReduceMode};

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day11.txt");

    const EXAMPLE_1: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

    #[test]
    fn solves_p1_example() {
        assert_eq!(
            calculate_monkey_business(EXAMPLE_1, 20, ReduceMode::DivideByThree),
            10605
        );
    }

    #[test]
    fn solves_p1() {
        assert_eq!(
            calculate_monkey_business(PUZZLE_INPUT, 20, ReduceMode::DivideByThree),
            108240
        );
    }

    #[test]
    fn solves_p2_example() {
        assert_eq!(
            calculate_monkey_business(EXAMPLE_1, 10_000, ReduceMode::ModulusByModuliProduct),
            2713310158
        );
    }

    #[test]
    fn solves_p2() {
        assert_eq!(
            calculate_monkey_business(PUZZLE_INPUT, 10_000, ReduceMode::ModulusByModuliProduct),
            25712998901
        );
    }
}
