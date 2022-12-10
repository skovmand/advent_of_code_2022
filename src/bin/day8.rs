use std::ops::Range;

use twentytwo::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();

    print_solution(
        8,
        1,
        "Consider your map; how many trees are visible from outside the grid?",
        format!("{}", count_visible_trees(&input)),
    );

    print_solution(
        8,
        2,
        "Consider each tree on your map. What is the highest scenic score possible for any tree?",
        format!("{}", max_scenic_score(&input)),
    );
}

// D8P1
fn count_visible_trees(input: &str) -> usize {
    let (trees, max_x, max_y) = parse_trees(input);

    (0..max_y)
        .flat_map(|y| (0..max_x).map(move |x| (x, y)))
        .filter(|(x, y)| is_tree_visible(&trees, *x, *y, max_x, max_y))
        .count()
}

fn is_tree_visible(trees: &Trees, x: usize, y: usize, max_x: usize, max_y: usize) -> bool {
    let tree_height = get_height(trees, x, y);
    let tree_is_lower = |(x, y): (usize, usize)| get_height(trees, x, y) < tree_height;

    west_iterator(x, y).all(tree_is_lower)
        || east_iterator(x, y, max_x).all(tree_is_lower)
        || north_iterator(x, y).all(tree_is_lower)
        || south_iterator(x, y, max_y).all(tree_is_lower)
}

// D8P2
fn max_scenic_score(input: &str) -> usize {
    let (trees, max_x, max_y) = parse_trees(input);

    (0..max_y)
        .flat_map(|y| (0..max_x).map(move |x| (x, y)))
        .map(|(x, y)| scenic_score(&trees, x, y, max_x, max_y))
        .max()
        .unwrap()
}

// This took me a while to get right 😅
type IteratorsAndRanges = [(Box<dyn Iterator<Item = (usize, usize)>>, Range<usize>); 4];

fn scenic_score(trees: &Trees, x: usize, y: usize, max_x: usize, max_y: usize) -> usize {
    let tree_height = get_height(trees, x, y);
    let higher_tree = |(x, y): &(usize, usize)| get_height(trees, *x, *y) >= tree_height;

    let iter_array: IteratorsAndRanges = [
        (Box::new(west_iterator(x, y)), west_range(x)),
        (Box::new(east_iterator(x, y, max_x)), east_range(x, max_x)),
        (Box::new(north_iterator(x, y)), north_range(y)),
        (Box::new(south_iterator(x, y, max_y)), south_range(y, max_y)),
    ];

    iter_array
        .into_iter()
        .map(|(iter, range)| {
            iter.enumerate()
                .find(|(_, coords)| higher_tree(coords))
                .map(|(i, _)| i + 1)
                .unwrap_or_else(|| range.len())
        })
        .product()
}

// Ranges and iterators -->
fn west_range(x: usize) -> Range<usize> {
    0..x
}

fn east_range(x: usize, max_x: usize) -> Range<usize> {
    x + 1..max_x
}

fn north_range(y: usize) -> Range<usize> {
    0..y
}

fn south_range(y: usize, max_y: usize) -> Range<usize> {
    y + 1..max_y
}

// Iterator from x towards west
fn west_iterator(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    west_range(x).rev().map(move |x| (x, y))
}

// Iterator from x towards east
fn east_iterator(x: usize, y: usize, max_x: usize) -> impl Iterator<Item = (usize, usize)> {
    east_range(x, max_x).map(move |x| (x, y))
}

// Iterator from x towards north
fn north_iterator(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    north_range(y).rev().map(move |y| (x, y))
}

// Iterator from x towards north
fn south_iterator(x: usize, y: usize, max_y: usize) -> impl Iterator<Item = (usize, usize)> {
    south_range(y, max_y).map(move |y| (x, y))
}

fn get_height(trees: &Trees, x: usize, y: usize) -> u8 {
    trees[y][x]
}

type Row = Vec<u8>;
type Trees = Vec<Row>;

fn parse_trees(input: &str) -> (Trees, usize, usize) {
    let trees = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| {
                    let height = b - 48;
                    assert!((0..=9).contains(&height));
                    height
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Trees>();

    let max_y = trees.len();
    let max_x = trees.first().expect("first row").len();

    (trees, max_x, max_y)
}

#[cfg(test)]
mod tests {
    use crate::{count_visible_trees, max_scenic_score};

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day8.txt");

    const EXAMPLE: &str = r#"30373
25512
65332
33549
35390
"#;

    #[test]
    fn solves_p1_examples() {
        assert_eq!(count_visible_trees(EXAMPLE), 21);
    }

    #[test]
    fn solves_p2_examples() {
        assert_eq!(max_scenic_score(EXAMPLE), 8);
    }

    #[test]
    fn solves_p1() {
        assert_eq!(count_visible_trees(PUZZLE_INPUT), 1684);
    }
}
