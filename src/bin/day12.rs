//! Day 12
//! Made a priority queue myself in a simple vec-backed version. Can be faster, but hey, I'm here for the learning.
//! The 2nd part was tricky and led me down a big multi-hour deroute. But I got the right idea in the end which sped up the solution from 1000ms to 20ms.
//! TIL: You can use ? inside of filter_map to return None. That's really convenient!
//! TIL2: A fn in Rust is a function pointer, but it doesn't work for closures.

use itertools::unfold;
use std::collections::{HashMap, HashSet};
use twentytwo::{print_solution, read_from_stdin, PriorityQueue};

fn main() {
    let input = read_from_stdin();
    let grid = Grid::from(input.as_str());

    print_solution(
        12,
        1,
        "What is the fewest steps required to move from your current position to the location that should get the best signal?",
        format!("{}", find_shortest_route_from_start_to_end(&grid).expect("Could not solve part 1"))
    );

    print_solution(
        12,
        2,
        "What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?",
        format!("{}", find_shortest_route_from_end_to_height_zero(grid).expect("Could not solve part 1"))
    );
}

/// A square simply wraps its height, also for start and end squares.
/// The start and end positions are saved to the Grid struct.
/// So start has a height of a, and end has a height of z.
#[derive(Debug, Clone, PartialEq)]
struct Square(u8);

impl From<u8> for Square {
    fn from(value: u8) -> Self {
        match value {
            valid_value if valid_value.is_ascii_lowercase() => Square(valid_value - b'a'),
            invalid_value => panic!("Invalid height {invalid_value}"),
        }
    }
}

impl Square {
    /// The first part - ascent max 1, descend any distance
    fn can_step_up_and_down_to_other(&self, other: &Square) -> bool {
        self.as_ref() + 1 >= *other.as_ref()
    }

    /// The second part - flip the criteria around, since we're going from the end to start, we are
    /// allowed to go at most 1 down square, but any distance up
    fn can_step_downwards_to_other(&self, other: &Square) -> bool {
        other.as_ref() + 1 >= *self.as_ref()
    }
}

impl AsRef<u8> for Square {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Clone, Debug)]
struct Grid {
    grid: HashMap<Position, Square>,
    start: Position,
    end: Position,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let (start, end, grid) = value
            .lines()
            .enumerate()
            .flat_map(|(row, text)| {
                text.bytes().enumerate().map(move |(column, letter)| {
                    let position = Position { row, column };

                    match letter {
                        b'S' => (true, false, position, Square::from(b'a')),
                        b'E' => (false, true, position, Square::from(b'z')),
                        letter => (false, false, position, Square::from(letter)),
                    }
                })
            })
            .fold(
                (None, None, HashMap::new()),
                |(found_start, found_end, mut grid), (is_start, is_end, pos, square)| {
                    (
                        found_start.or(if is_start { Some(pos) } else { None }),
                        found_end.or(if is_end { Some(pos) } else { None }),
                        {
                            grid.insert(pos, square);
                            grid
                        },
                    )
                },
            );

        Grid {
            start: start.expect("Could not find start square"),
            end: end.expect("Could not find end square"),
            grid,
        }
    }
}

impl Grid {
    fn climbable_neighbours(&self, position: &Position, is_climbable: fn(&Square, &Square) -> bool) -> Vec<Position> {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter_map(|relative_coord| {
                Some(Position {
                    column: position.column.checked_add_signed(relative_coord.0)?,
                    row: position.row.checked_add_signed(relative_coord.1)?,
                })
            })
            .filter_map(|pos| self.grid.get(&pos).map(|square| (pos, square)))
            .filter(|(_, square)| is_climbable(self.grid.get(position).unwrap(), square))
            .map(|(pos, _)| pos)
            .collect::<Vec<Position>>()
    }
}

/// Part 1: Solve from start to end using a priority queue
fn find_shortest_route_from_start_to_end(grid: &Grid) -> Option<usize> {
    find_shortest_route(
        grid,
        Square::can_step_up_and_down_to_other,
        IsRouteEndTest::StartPosition,
    )
}

/// Part 2: Flip the start and end, use alternate way of finding end and height
/// The idea is basically to find the shortest path from end to any height 0, also using the priority queue
fn find_shortest_route_from_end_to_height_zero(mut grid: Grid) -> Option<usize> {
    std::mem::swap(&mut grid.start, &mut grid.end);
    find_shortest_route(&grid, Square::can_step_downwards_to_other, IsRouteEndTest::HeightZero)
}

/// We have to check whether we are at the route end in two different ways for P1 and P2
/// Could have passed in a closure, but using this enum was quicker and more readable
enum IsRouteEndTest {
    StartPosition,
    HeightZero,
}

struct RoutingState {
    finished: HashSet<Position>,
    queue: PriorityQueue<Position>,
}

/// Find the count of fewest steps from a start position to some other position defined by route_end_test
/// Uses the is_climbable function pointer to decide which neighbours are climbable, this varies from part 1 to 2
fn find_shortest_route(
    grid: &Grid,
    is_climbable: fn(&Square, &Square) -> bool,
    is_route_end_test: IsRouteEndTest,
) -> Option<usize> {
    let mut priority_queue_iter = unfold(
        RoutingState {
            finished: HashSet::new(),
            queue: PriorityQueue::with_one_element(grid.start, 0),
        },
        |RoutingState { finished, queue }| {
            let (position, priority) = queue.dequeue()?;

            grid.climbable_neighbours(&position, is_climbable)
                .iter()
                .filter(|position| !finished.contains(position))
                .map(|position| (position, priority + 1))
                .for_each(|(position, new_priority)| queue.enqueue(*position, new_priority));

            finished.insert(position);

            Some((position, priority))
        },
    );

    priority_queue_iter
        .find(|(position, _)| match is_route_end_test {
            IsRouteEndTest::StartPosition => *position == grid.end,
            IsRouteEndTest::HeightZero => *grid.grid.get(position).unwrap().as_ref() == 0,
        })
        .map(|(_, priority)| priority)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day12.txt");

    #[test]
    fn solves_d12_p1_example() {
        let grid = Grid::from(EXAMPLE);

        assert_eq!(find_shortest_route_from_start_to_end(&grid).unwrap(), 31);
    }

    #[test]
    fn solves_d12_p1() {
        let grid = Grid::from(PUZZLE_INPUT);

        assert_eq!(find_shortest_route_from_start_to_end(&grid).unwrap(), 361);
    }

    #[test]
    fn solves_d12_p2_example() {
        let grid = Grid::from(EXAMPLE);

        assert_eq!(find_shortest_route_from_end_to_height_zero(grid).unwrap(), 29);
    }

    #[test]
    fn solves_d12_p2() {
        let grid = Grid::from(PUZZLE_INPUT);

        assert_eq!(find_shortest_route_from_end_to_height_zero(grid).unwrap(), 354);
    }
}
