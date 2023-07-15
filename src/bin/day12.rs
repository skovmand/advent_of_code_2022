//! Day 12
//! Took a lot of thought, and I did the PriorityQueue myself in a really simple vec-backed version.
//! The 2nd part can definitely be more optimized by reusing already known paths.  Update: I optimized it, and it's A LOT faster now.

use std::collections::{HashMap, HashSet};
use twentytwo::{print_solution, read_from_stdin, PriorityQueue};

fn main() {
    let input = read_from_stdin();
    let grid = Grid::from(input.as_str());

    print_solution(
        12,
        1,
        "What is the fewest steps required to move from your current position to the location that should get the best signal?",
        format!("{}", find_shortest_route_from_start_to_end(&grid))
    );

    print_solution(
        12,
        2,
        "What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?",
        format!("{}", find_shortest_route_from_end_to_height_zero(grid))
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

/// Part 1: Solve from start to end
fn find_shortest_route_from_start_to_end(grid: &Grid) -> usize {
    find_shortest_route(
        grid,
        Square::can_step_up_and_down_to_other,
        IsRouteEndTest::StartPosition,
    )
}

/// Part 2: Flip the start and end, use alternate way of finding end and height
/// Hope that a path is possible :-D
fn find_shortest_route_from_end_to_height_zero(mut grid: Grid) -> usize {
    std::mem::swap(&mut grid.start, &mut grid.end);
    find_shortest_route(&grid, Square::can_step_downwards_to_other, IsRouteEndTest::HeightZero)
}

/// We have to check whether we are at the route end in two different ways
enum IsRouteEndTest {
    StartPosition,
    HeightZero,
}

/// Find the count of fewest steps from start to end in a Grid
/// Uses the is_climbable function to decide which neighbours are climbable, this varies from part 1 to 2
fn find_shortest_route(
    grid: &Grid,
    is_climbable: fn(&Square, &Square) -> bool,
    is_route_end_test: IsRouteEndTest,
) -> usize {
    let mut finished_positions: HashSet<Position> = HashSet::new();
    let mut pq: PriorityQueue<Position> = PriorityQueue::new();

    pq.enqueue(grid.start, 0);

    loop {
        if let Some((priority, dequeued)) = pq.dequeue() {
            match is_route_end_test {
                IsRouteEndTest::StartPosition => {
                    if dequeued == grid.end {
                        break priority;
                    }
                }
                IsRouteEndTest::HeightZero => {
                    if *grid.grid.get(&dequeued).unwrap().as_ref() == 0 {
                        break priority;
                    }
                }
            }

            grid.climbable_neighbours(&dequeued, is_climbable)
                .iter()
                .filter(|position| !finished_positions.contains(position))
                .map(|position| (position, priority + 1))
                .for_each(|(position, new_priority)| pq.enqueue(*position, new_priority));

            finished_positions.insert(dequeued);
        } else {
            panic!("The route is not possible")
        }
    }
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

        assert_eq!(find_shortest_route_from_start_to_end(&grid), 31);
    }

    #[test]
    fn solves_d12_p1() {
        let grid = Grid::from(PUZZLE_INPUT);

        assert_eq!(find_shortest_route_from_start_to_end(&grid), 361);
    }

    #[test]
    fn solves_d12_p2_example() {
        let grid = Grid::from(EXAMPLE);

        assert_eq!(find_shortest_route_from_end_to_height_zero(grid), 29);
    }

    #[test]
    fn solves_d12_p2() {
        let grid = Grid::from(PUZZLE_INPUT);

        assert_eq!(find_shortest_route_from_end_to_height_zero(grid), 354);
    }
}
