fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut cave_slice = read_cave_slice();
    if cave_slice.is_empty() {
        println!("0");
        return;
    }

    let cols_to_add = vec!['.'; cave_slice[0].len()];
    for row in cave_slice.iter_mut() {
        row.extend_from_slice(&cols_to_add);
    }

    cave_slice.push(vec!['.'; cave_slice[0].len()]);
    cave_slice.push(vec!['#'; cave_slice[0].len()]);

    let mut units_of_sand = 1;
    loop {
        let (sand_unit_row, sand_unit_col) = perform_sand_unit_fall(&cave_slice, 0, 500);
        if sand_unit_row == 0 && sand_unit_col == 500 {
            break;
        }

        cave_slice[sand_unit_row][sand_unit_col] = 'o';
        units_of_sand += 1;

        // print_cave_slice(&cave_slice, 492);
        // println!();
    }

    println!("{units_of_sand}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut cave_slice = read_cave_slice();

    let mut units_of_sand = 0;
    loop {
        let (sand_unit_row, sand_unit_col) = perform_sand_unit_fall(&cave_slice, 0, 500);
        if sand_unit_row >= cave_slice.len() - 1 {
            break;
        }

        cave_slice[sand_unit_row][sand_unit_col] = 'o';
        units_of_sand += 1;

        // print_cave_slice(&cave_slice, 492);
        // println!();
    }

    println!("{units_of_sand}");
}

fn perform_sand_unit_fall(cave_slice: &[Vec<char>], row: usize, col: usize) -> (usize, usize) {
    if row >= cave_slice.len() {
        return (row, col);
    }

    let mut current_row = row;
    while current_row < cave_slice.len() - 1 && cave_slice[current_row + 1][col] == '.' {
        current_row += 1;
    }

    if current_row < cave_slice.len() - 1 {
        if col > 0 && cave_slice[current_row + 1][col - 1] == '.' {
            return perform_sand_unit_fall(cave_slice, current_row + 1, col - 1);
        }

        if col < cave_slice[0].len() - 1 && cave_slice[current_row + 1][col + 1] == '.' {
            return perform_sand_unit_fall(cave_slice, current_row + 1, col + 1);
        }
    }

    (current_row, col)
}

#[allow(dead_code)]
fn print_cave_slice(cave_slice: &[Vec<char>], skip: usize) {
    for row in cave_slice {
        for col in row.iter().skip(skip) {
            print!("{col}");
        }

        println!();
    }
}

fn read_cave_slice() -> Vec<Vec<char>> {
    let mut cave_slice: Vec<Vec<char>> = vec![];

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let rock_paths = line
            .split(" -> ")
            .map(|raw_rock_path| {
                let raw_path_row_and_col = raw_rock_path
                    .split(',')
                    .map(|raw_coord| raw_coord.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                (raw_path_row_and_col[1], raw_path_row_and_col[0])
            })
            .collect::<Vec<(usize, usize)>>();

        let (mut last_row, mut last_col) = match rock_paths.first() {
            Some(v) => *v,
            None => break,
        };

        resize_cave_slice_if_needed(&mut cave_slice, last_row + 1, last_col + 1);

        for (row, col) in rock_paths.iter().skip(1) {
            resize_cave_slice_if_needed(&mut cave_slice, row + 1, col + 1);

            let (start_row, end_row) = (*row.min(&last_row), *row.max(&last_row));
            let (start_col, end_col) = (*col.min(&last_col), *col.max(&last_col));

            for row in start_row..=end_row {
                for col in start_col..=end_col {
                    cave_slice[row][col] = '#';
                }
            }

            last_row = *row;
            last_col = *col;
        }
    }

    cave_slice
}

fn resize_cave_slice_if_needed(
    cave_slice: &mut Vec<Vec<char>>,
    target_rows: usize,
    target_cols: usize,
) {
    let current_cols = if cave_slice.is_empty() {
        0
    } else {
        cave_slice[0].len()
    };

    let (rows_to_add, overflowed) = target_rows.overflowing_sub(cave_slice.len());
    if !overflowed && rows_to_add > 0 {
        cave_slice.extend_from_slice(&vec![vec!['.'; current_cols]; rows_to_add]);
    }

    let (cols_to_add, overflowed) = target_cols.overflowing_sub(current_cols);
    if !overflowed && cols_to_add > 0 {
        let cols_to_add = vec!['.'; cols_to_add];
        for row in cave_slice.iter_mut() {
            row.extend_from_slice(&cols_to_add);
        }
    }
}
