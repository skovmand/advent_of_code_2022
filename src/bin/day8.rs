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

fn count_visible_trees(input: &str) -> usize {
    let (trees, max_x, max_y) = parse_trees(input);

    (0..max_y)
        .flat_map(|y| (0..max_x).map(move |x| (x, y)))
        .filter(|(x, y)| !is_tree_hidden(&trees, *x, *y, max_x, max_y))
        .count()
}

fn is_tree_hidden(trees: &Trees, x: usize, y: usize, max_x: usize, max_y: usize) -> bool {
    let tree_height = get_height(trees, x, y);

    // Coordinates to check
    let mut from_left = (0..x).map(|x| (x, y));
    let mut from_right = ((x + 1)..max_x).map(|x| (x, y));
    let mut from_top = (0..y).map(|y| (x, y));
    let mut from_bottom = (y + 1..max_y).map(|y| (x, y));

    let hidden_from_left = from_left.any(|(x, y)| get_height(trees, x, y) >= tree_height);
    let hidden_from_right = from_right.any(|(x, y)| get_height(trees, x, y) >= tree_height);
    let hidden_from_top = from_top.any(|(x, y)| get_height(trees, x, y) >= tree_height);
    let hidden_from_bottom = from_bottom.any(|(x, y)| get_height(trees, x, y) >= tree_height);

    hidden_from_left && hidden_from_right && hidden_from_top && hidden_from_bottom
}

fn max_scenic_score(input: &str) -> usize {
    let (trees, max_x, max_y) = parse_trees(input);

    (0..max_y)
        .flat_map(|y| (0..max_x).map(move |x| (x, y)))
        .map(|(x, y)| scenic_score(&trees, x, y, max_x, max_y))
        .max()
        .unwrap()
}

fn scenic_score(trees: &Trees, x: usize, y: usize, max_x: usize, max_y: usize) -> usize {
    let tree_height = get_height(trees, x, y);

    // Coordinates to check
    let west_len = (0..x).len();
    let east_len = (x + 1..max_x).len();
    let north_len = (0..y).len();
    let south_len = (y + 1..max_y).len();

    let mut west = (0..x).rev().map(|x| (x, y)).enumerate();
    let mut east = ((x + 1)..max_x).map(|x| (x, y)).enumerate();
    let mut north = (0..y).rev().map(|y| (x, y)).enumerate();
    let mut south = (y + 1..max_y).map(|y| (x, y)).enumerate();

    let count_west = west
        .find(|(_, (x, y))| get_height(trees, *x, *y) >= tree_height)
        .map(|(i, _)| i + 1)
        .unwrap_or(west_len);

    let count_east = east
        .find(|(_, (x, y))| get_height(trees, *x, *y) >= tree_height)
        .map(|(i, _)| i + 1)
        .unwrap_or(east_len);

    let count_north = north
        .find(|(_, (x, y))| get_height(trees, *x, *y) >= tree_height)
        .map(|(i, _)| i + 1)
        .unwrap_or(north_len);

    let count_south = south
        .find(|(_, (x, y))| get_height(trees, *x, *y) >= tree_height)
        .map(|(i, _)| i + 1)
        .unwrap_or(south_len);

    count_west * count_east * count_north * count_south
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
