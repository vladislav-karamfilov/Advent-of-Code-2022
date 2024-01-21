fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let tree_height_map = &read_tree_height_map();

    let mut max_scenic_score = i32::MIN;

    for row in 1..tree_height_map.len() - 1 {
        for col in 1..tree_height_map[row].len() - 1 {
            let scenic_score = calculate_tree_scenic_score(tree_height_map, row, col);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("{max_scenic_score}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let tree_height_map = &read_tree_height_map();

    // Trees on the edges are always visible
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

fn calculate_tree_scenic_score(
    tree_height_map: &[Vec<i8>],
    tree_row: usize,
    tree_col: usize,
) -> i32 {
    let tree_height = tree_height_map[tree_row][tree_col];

    calculate_tree_viewing_distance_on_left(tree_height_map, tree_height, tree_row, tree_col)
        * calculate_tree_viewing_distance_on_right(tree_height_map, tree_height, tree_row, tree_col)
        * calculate_tree_viewing_distance_on_top(tree_height_map, tree_height, tree_row, tree_col)
        * calculate_tree_viewing_distance_on_bottom(
            tree_height_map,
            tree_height,
            tree_row,
            tree_col,
        )
}

fn calculate_tree_viewing_distance_on_left(
    tree_height_map: &[Vec<i8>],
    tree_height: i8,
    tree_row: usize,
    tree_col: usize,
) -> i32 {
    let mut result = 0;

    for col in (0..tree_col).rev() {
        result += 1;

        if tree_height_map[tree_row][col] >= tree_height {
            break;
        }
    }

    result
}

fn calculate_tree_viewing_distance_on_right(
    tree_height_map: &[Vec<i8>],
    tree_height: i8,
    tree_row: usize,
    tree_col: usize,
) -> i32 {
    let mut result = 0;

    for col in tree_col + 1..tree_height_map[0].len() {
        result += 1;

        if tree_height_map[tree_row][col] >= tree_height {
            break;
        }
    }

    result
}

fn calculate_tree_viewing_distance_on_bottom(
    tree_height_map: &[Vec<i8>],
    tree_height: i8,
    tree_row: usize,
    tree_col: usize,
) -> i32 {
    let mut result = 0;

    for row in tree_row + 1..tree_height_map.len() {
        result += 1;

        if tree_height_map[row][tree_col] >= tree_height {
            break;
        }
    }

    result
}

fn calculate_tree_viewing_distance_on_top(
    tree_height_map: &[Vec<i8>],
    tree_height: i8,
    tree_row: usize,
    tree_col: usize,
) -> i32 {
    let mut result = 0;

    for row in (0..tree_row).rev() {
        result += 1;

        if tree_height_map[row][tree_col] >= tree_height {
            break;
        }
    }

    result
}

fn is_visible_tree(tree_height_map: &[Vec<i8>], tree_row: usize, tree_col: usize) -> bool {
    let tree_height = tree_height_map[tree_row][tree_col];

    is_visible_tree_from_top(tree_height_map, tree_height, tree_row, tree_col)
        || is_visible_tree_from_bottom(tree_height_map, tree_height, tree_row, tree_col)
        || is_visible_tree_from_left(tree_height_map, tree_height, tree_row, tree_col)
        || is_visible_tree_from_right(tree_height_map, tree_height, tree_row, tree_col)
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
