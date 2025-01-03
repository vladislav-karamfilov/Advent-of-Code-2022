fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let (mut board_map, path) = read_board_map_and_path();

    normalize_board_map(&mut board_map);

    let (final_position, final_move_direction) = follow_path(&board_map, &path);

    let final_password = calculate_final_password(final_position, final_move_direction);

    println!("{final_password}");
}

fn follow_path(board_map: &[Vec<char>], path: &str) -> (Position, MoveDirection) {
    let mut current_position = Position {
        row: 0,
        col: board_map[0].iter().position(|ch| *ch == '.').unwrap(),
    };

    let mut current_move_direction = MoveDirection::Right;

    let path_chars = path.chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < path_chars.len() {
        if path_chars[i] == 'R' {
            current_move_direction = match current_move_direction {
                MoveDirection::Right => MoveDirection::Down,
                MoveDirection::Down => MoveDirection::Left,
                MoveDirection::Left => MoveDirection::Up,
                MoveDirection::Up => MoveDirection::Right,
            };

            i += 1;
        } else if path_chars[i] == 'L' {
            current_move_direction = match current_move_direction {
                MoveDirection::Right => MoveDirection::Up,
                MoveDirection::Down => MoveDirection::Right,
                MoveDirection::Left => MoveDirection::Down,
                MoveDirection::Up => MoveDirection::Left,
            };

            i += 1;
        } else {
            let end_i = match path[i..].chars().position(|ch| ch == 'R' || ch == 'L') {
                Some(end_i) => i + end_i,
                None => path.len(),
            };

            let tiles_to_move = path[i..end_i].parse::<u8>().unwrap();
            for _ in 0..tiles_to_move {
                let next_position =
                    calculate_next_position(board_map, current_position, current_move_direction);

                if next_position == current_position {
                    break;
                }

                current_position = next_position;
            }

            i = end_i;
        }
    }

    (current_position, current_move_direction)
}

fn calculate_next_position(
    board_map: &[Vec<char>],
    current_position: Position,
    move_direction: MoveDirection,
) -> Position {
    match move_direction {
        MoveDirection::Right => {
            let mut next_col = if current_position.col < board_map[current_position.row].len() - 1 {
                current_position.col + 1
            } else {
                0
            };

            while board_map[current_position.row][next_col] == ' ' {
                next_col += 1;

                if next_col == board_map[current_position.row].len() {
                    next_col = 0;
                }
            }

            if board_map[current_position.row][next_col] == '.' {
                return Position {
                    col: next_col,
                    ..current_position
                };
            }

            return current_position;
        }
        MoveDirection::Down => {
            let mut next_row = if current_position.row < board_map.len() - 1 {
                current_position.row + 1
            } else {
                0
            };

            while board_map[next_row][current_position.col] == ' ' {
                next_row += 1;

                if next_row == board_map.len() {
                    next_row = 0;
                }
            }

            if board_map[next_row][current_position.col] == '.' {
                return Position {
                    row: next_row,
                    ..current_position
                };
            }

            return current_position;
        }
        MoveDirection::Left => {
            let mut next_col = if current_position.col > 0 {
                current_position.col - 1
            } else {
                board_map[current_position.row].len() - 1
            };

            while board_map[current_position.row][next_col] == ' ' {
                next_col -= 1;

                if next_col == 0 && board_map[current_position.row][next_col] == ' ' {
                    next_col = board_map[current_position.row].len() - 1;
                }
            }

            if board_map[current_position.row][next_col] == '.' {
                return Position {
                    col: next_col,
                    ..current_position
                };
            }

            return current_position;
        }
        MoveDirection::Up => {
            let mut next_row = if current_position.row > 0 {
                current_position.row - 1
            } else {
                board_map.len() - 1
            };

            while board_map[next_row][current_position.col] == ' ' {
                next_row -= 1;

                if next_row == 0 && board_map[next_row][current_position.col] == ' ' {
                    next_row = board_map.len() - 1;
                }
            }

            if board_map[next_row][current_position.col] == '.' {
                return Position {
                    row: next_row,
                    ..current_position
                };
            }

            return current_position;
        }
    }
}

fn calculate_final_password(
    final_position: Position,
    final_move_direction: MoveDirection,
) -> usize {
    1_000 * (final_position.row + 1)
        + 4 * (final_position.col + 1)
        + match final_move_direction {
            MoveDirection::Right => 0,
            MoveDirection::Down => 1,
            MoveDirection::Left => 2,
            MoveDirection::Up => 3,
        }
}

fn normalize_board_map(board_map: &mut [Vec<char>]) {
    let max_cols = board_map.iter().map(|x| x.len()).max().unwrap();
    for line in board_map.iter_mut() {
        let white_spaces_to_add = max_cols - line.len();
        if white_spaces_to_add > 0 {
            line.extend(vec![' '; white_spaces_to_add]);
        }
    }
}

fn read_board_map_and_path() -> (Vec<Vec<char>>, String) {
    let mut board_map = vec![];
    let mut path = "".to_string();
    let mut is_reading_board_map = true;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            if is_reading_board_map {
                is_reading_board_map = false;
                continue;
            }

            break;
        }

        if is_reading_board_map {
            board_map.push(
                line.trim_end_matches(|ch| ch == '\r' || ch == '\n')
                    .chars()
                    .collect(),
            );
        } else {
            path = trimmed_line.to_string();
        }
    }

    (board_map, path)
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy)]
enum MoveDirection {
    Right,
    Down,
    Left,
    Up,
}
