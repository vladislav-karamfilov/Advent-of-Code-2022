fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let tree_height_map = &read_tree_height_map();

    let mut visible_trees = tree_height_map.len() * 2 + (tree_height_map[0].len() - 2) * 2;

    for row in 1..tree_height_map.len() - 1 {
        for col in 1..tree_height_map[row].len() - 1 {
            if is_visible_tree(tree_height_map, row, col) {
                visible_trees += 1;
            }
        }
    }

    println!("{visible_trees}");
}

fn is_visible_tree_from_left(
    tree_height_map: &[Vec<i8>],
    tree_height: i8,
    tree_row: usize,
    tree_col: usize,
) -> bool {
    for col in 0..tree_col {
        if tree_height_map[tree_row][col] >= tree_height {
            return false;
        }
    }

    true
}

fn is_visible_tree_from_right(
    tree_height_map: &[Vec<i8>],
    tree_height: i8,
    tree_row: usize,
    tree_col: usize,
) -> bool {
    for col in tree_col + 1..tree_height_map[0].len() {
        if tree_height_map[tree_row][col] >= tree_height {
            return false;
        }
    }

    true
}

fn is_visible_tree_from_bottom(
    tree_height_map: &[Vec<i8>],
    tree_height: i8,
    tree_row: usize,
    tree_col: usize,
) -> bool {
    for row in tree_row + 1..tree_height_map.len() {
        if tree_height_map[row][tree_col] >= tree_height {
            return false;
        }
    }

    true
}

fn is_visible_tree_from_top(
    tree_height_map: &[Vec<i8>],
    tree_height: i8,
    tree_row: usize,
    tree_col: usize,
) -> bool {
    for row in 0..tree_row {
        if tree_height_map[row][tree_col] >= tree_height {
            return false;
        }
    }

    true
}
fn is_visible_tree(tree_height_map: &[Vec<i8>], tree_row: usize, tree_col: usize) -> bool {
    let tree_height = tree_height_map[tree_row][tree_col];

    is_visible_tree_from_top(tree_height_map, tree_height, tree_row, tree_col)
        || is_visible_tree_from_bottom(tree_height_map, tree_height, tree_row, tree_col)
        || is_visible_tree_from_left(tree_height_map, tree_height, tree_row, tree_col)
        || is_visible_tree_from_right(tree_height_map, tree_height, tree_row, tree_col)
}

fn read_tree_height_map() -> Vec<Vec<i8>> {
    let mut tree_height_map = Vec::new();

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        tree_height_map.push(line.chars().map(|ch| ch as i8 - '0' as i8).collect())
    }

    tree_height_map
}
