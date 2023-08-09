//! Day 15: Beacon Exclusion Zone
//!
//! TIL: I tried using once_cell
//! TIL2: I tried using coalesce from itertools
//! TIL3: Building in release mode runs a lot faster. Solving both days takes 528ms on my machine!

use once_cell::sync::Lazy;

use itertools::Itertools;
use regex::Regex;
use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();
    let data = parse_sensor_data(&input);

    print_solution(
        15,
        1,
        "Consult the report from the sensors you just deployed. In the row where y=2000000, how many positions cannot contain a beacon?",
        format!("{}", count_positions_that_cannot_contain_a_beacon(2000000, &data))
    );

    print_solution(
        15,
        2,
        "Find the only possible position for the distress beacon. What is its tuning frequency?",
        format!("{}", tuning_frequency(4000000, &data)),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

// We don't even need the beacon in the SensorData since we calculate the manhattan_distance when parsing
#[derive(Debug)]
struct SensorData {
    sensor: Position,
    manhattan_distance: i64,
}

// D15P1
fn count_positions_that_cannot_contain_a_beacon(y: i64, sensor_data: &[SensorData]) -> i64 {
    ranges(y, sensor_data)
        .iter()
        .map(|Range { start, end }| (end - start).abs())
        .sum()
}

// D15P2
fn tuning_frequency(max_y: i64, sensor_data: &[SensorData]) -> i64 {
    let (beacon_y, ranges) = (0..=max_y)
        .map(|y| (y, ranges(y, sensor_data)))
        .find(|(_, ranges)| ranges.len() == 2)
        .expect("Could not find line with beacon");

    let beacon_x = (ranges.first().unwrap()).end + 1;

    beacon_x * 4000000 + beacon_y
}

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

// For a given line at y, calculate all the ranges that beacons cannot be,
// then coalesce into as few ranges as possible.
fn ranges(y: i64, sensor_data: &[SensorData]) -> Vec<Range> {
    let mut all_ranges = sensor_data
        .iter()
        .map(|data| {
            // Add the vertical distance from y to the sensor
            let y_dist = (data.sensor.y - y).unsigned_abs() as i64;
            (data, y_dist)
        })
        .filter(|(data, y_dist)| {
            // If the distance from sensor to y is larger than the manhattan distance from sensor to beacon,
            // the sensor doesn't cover any part of the line and can be ignored.
            y_dist <= &data.manhattan_distance
        })
        .map(|(data, y_dist)| {
            let effective_radius = data.manhattan_distance - y_dist;

            Range {
                start: data.sensor.x - effective_radius,
                end: data.sensor.x + effective_radius,
            }
        })
        .collect::<Vec<Range>>();

    // Sort all the ranges to prepare to coalesce
    all_ranges.sort_by(|r1, r2| r1.start.cmp(&r2.start));

    all_ranges
        .into_iter()
        .coalesce(|r1, r2| {
            // How can things overlap?
            // We know that r1.start is lower than r2.start due to the initial sorting
            //
            // 1) Range 1 is completely before the 2nd one - there is no overlap
            // aaaaaa
            //        bbbbbb
            //
            // 2) Range 2 fits onto the end of range 1
            // aaaaaa
            //       bbbbbbbb
            //
            // 3) Range 2 overlaps with range 1, but the end is not larger
            // aaaaaaaaaaaaaa
            //             bbbbb
            //
            // 4) Range 2 overlaps with range 1, but the end of range 1 is larger
            // aaaaaaaaaaaaaa
            //  bbb
            if r1.end + 1 >= r2.start {
                if r1.end >= r2.end {
                    // Case 4
                    Ok(r1)
                } else {
                    // Case 2, 3
                    Ok(Range {
                        start: r1.start,
                        end: r2.end,
                    })
                }
            } else {
                // Case 1
                Err((r1, r2))
            }
        })
        .collect()
}

fn manhattan_distance(p1: &Position, p2: &Position) -> i64 {
    i64::abs(p1.x - p2.x) + i64::abs(p1.y - p2.y)
}

// Parsing:

fn parse_sensor_data(input: &str) -> Vec<SensorData> {
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> SensorData {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"Sensor at x=(?<sensor_x>-?\d+), y=(?<sensor_y>-?\d+): closest beacon is at x=(?<beacon_x>-?\d+), y=(?<beacon_y>-?\d+)").unwrap()
    });

    let captures = RE.captures(input).expect("No match for input line");

    let sensor = Position {
        x: captures["sensor_x"].parse::<i64>().unwrap(),
        y: captures["sensor_y"].parse::<i64>().unwrap(),
    };

    let beacon = Position {
        x: captures["beacon_x"].parse::<i64>().unwrap(),
        y: captures["beacon_y"].parse::<i64>().unwrap(),
    };

    SensorData {
        sensor,
        manhattan_distance: manhattan_distance(&sensor, &beacon),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day15.txt");

    #[test]
    fn solves_d15_p1_example() {
        let data = parse_sensor_data(EXAMPLE_INPUT);
        let solution = count_positions_that_cannot_contain_a_beacon(10, &data);

        assert_eq!(solution, 26);
    }

    #[test]
    fn solves_d15_p1() {
        let data = parse_sensor_data(PUZZLE_INPUT);
        let solution = count_positions_that_cannot_contain_a_beacon(2000000, &data);

        assert_eq!(solution, 5809294);
    }

    #[test]
    fn solves_d15_p2_example() {
        let data = parse_sensor_data(EXAMPLE_INPUT);
        let solution = tuning_frequency(20, &data);

        assert_eq!(solution, 56000011);
    }

    #[test]
    fn solves_d15_p2() {
        let data = parse_sensor_data(PUZZLE_INPUT);
        let solution = tuning_frequency(4000000, &data);

        assert_eq!(solution, 10693731308112);
    }
}
