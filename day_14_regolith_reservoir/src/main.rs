fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let mut cave_slice = read_cave_slice();

    let units_of_sand = 0;
    loop {
        let mut row = 0;
        let mut col = 500;
        // TODO:
    }

    println!("{units_of_sand}");
}

fn print_cave_slice(cave_slice: &[Vec<char>]) {
    for row in cave_slice {
        for col in row {
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
    cave_slice: &mut [Vec<char>],
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
        for _ in 0..rows_to_add {
            cave_slice.push(vec!['.'; current_cols]);
        }
    }

    let (cols_to_add, overflowed) = target_cols.overflowing_sub(current_cols);
    if !overflowed && cols_to_add > 0 {
        for row in cave_slice.iter_mut() {
            for _ in 0..=cols_to_add {
                row.push('.');
            }
        }
    }
}
