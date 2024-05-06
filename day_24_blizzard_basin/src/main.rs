use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use priority_queue::PriorityQueue;

fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let map = read_map_of_valley_with_blizzards();

    let rows = map.len();
    let cols = map[0].len();
    let start_col = map[0].chars().position(|ch| ch == '.').unwrap();
    let end_col = map[rows - 1].chars().position(|ch| ch == '.').unwrap();

    let valley_info = ValleyInfo {
        rows,
        cols,
        start_position: Position {
            row: 0,
            col: start_col,
        },
        end_position: Position {
            row: rows - 1,
            col: end_col,
        },
    };

    let mut blizzards = vec![];
    for (row, map_row) in map.iter().enumerate() {
        if row == 0 || row == rows - 1 {
            continue;
        }

        for (col, ch) in map_row.chars().enumerate() {
            if col == 0 || col == cols - 1 {
                continue;
            }

            let blizzard_direction = match ch {
                '>' => Some(MoveDirection::Right),
                '<' => Some(MoveDirection::Left),
                '^' => Some(MoveDirection::Up),
                'v' => Some(MoveDirection::Down),
                _ => None,
            };

            if blizzard_direction.is_some() {
                blizzards.push(Blizzard {
                    direction: blizzard_direction.unwrap(),
                    position: Position { row, col },
                });
            }
        }
    }

    let minutes = calculate_min_minutes_to_reach_valley_end(&valley_info, &blizzards);

    println!("{minutes}");
}

// Implementation of A* search algorithm: https://en.wikipedia.org/wiki/A*_search_algorithm
fn calculate_min_minutes_to_reach_valley_end(
    valley_info: &ValleyInfo,
    blizzards: &[Blizzard],
) -> i32 {
    let mut states = PriorityQueue::new();

    let mut initial_state = ValleyState {
        position: valley_info.start_position,
        minute: 0,
        estimated_distance_to_end: 0,
    };

    initial_state.set_estimated_distance_to_end(&valley_info.end_position);

    let state_score = initial_state.get_score();
    states.push(initial_state, Reverse(state_score));

    let mut blizzards_per_minute = HashMap::new();
    blizzards_per_minute.insert(0, blizzards.to_vec());

    let mut seen = HashSet::new();

    while let Some((current_state, _)) = states.pop() {
        if current_state.position == valley_info.end_position {
            return current_state.minute as i32;
        }

        if !seen.insert((current_state.position, current_state.minute)) {
            continue;
        }

        if !blizzards_per_minute.contains_key(&current_state.minute) {
            let current_minute_blizzards = move_blizzards(
                blizzards_per_minute
                    .get(&(current_state.minute - 1))
                    .unwrap(),
                valley_info,
            );

            blizzards_per_minute.insert(current_state.minute, current_minute_blizzards);
        }

        let blizzards = blizzards_per_minute.get(&current_state.minute).unwrap();

        // print_valley(&current_state, blizzards, valley_info);

        let next_states = calculate_next_states(&current_state, blizzards, valley_info, &seen);
        for next_state in next_states {
            let state_score = next_state.get_score();
            states.push(next_state, Reverse(state_score));
        }

        // Wait on same position
        let same_position_state = ValleyState {
            position: current_state.position,
            minute: current_state.minute + 1,
            estimated_distance_to_end: current_state.estimated_distance_to_end,
        };

        let state_score = same_position_state.get_score();
        states.push(same_position_state, Reverse(state_score));
    }

    -1
}

fn print_valley(current_state: &ValleyState, blizzards: &[Blizzard], valley_info: &ValleyInfo) {
    println!("On minute {}", current_state.minute);
    for row in 0..valley_info.rows {
        for col in 0..valley_info.cols {
            let position = Position { row, col };
            if position == current_state.position {
                print!("E");
            } else if position == valley_info.start_position || position == valley_info.end_position
            {
                print!(".");
            } else if row == 0
                || row == valley_info.rows - 1
                || col == 0
                || col == valley_info.cols - 1
            {
                print!("#");
            } else {
                if let Some(blizzard) = blizzards.iter().find(|b| b.position == position) {
                    match blizzard.direction {
                        MoveDirection::Up => print!("^"),
                        MoveDirection::Down => print!("v"),
                        MoveDirection::Right => print!(">"),
                        MoveDirection::Left => print!("<"),
                    }
                } else {
                    print!(".");
                }
            }
        }

        println!();
    }

    println!();
}

fn calculate_next_states(
    current_state: &ValleyState,
    blizzards: &[Blizzard],
    valley_info: &ValleyInfo,
    seen: &HashSet<(Position, usize)>,
) -> Vec<ValleyState> {
    let mut result = vec![];

    let (current_row, current_col) = (current_state.position.row, current_state.position.col);

    // Up
    if current_row > 1 {
        let new_position = Position {
            row: current_row - 1,
            col: current_col,
        };

        if !seen.contains(&(new_position, current_state.minute + 1))
            && !blizzards.iter().any(|b| b.position == new_position)
        {
            result.push(ValleyState {
                position: new_position,
                minute: current_state.minute + 1,
                estimated_distance_to_end: 0,
            });
        }
    }

    // Down
    if current_row < valley_info.rows - 2 {
        let new_position = Position {
            row: current_row + 1,
            col: current_col,
        };

        if !seen.contains(&(new_position, current_state.minute + 1))
            && !blizzards.iter().any(|b| b.position == new_position)
        {
            result.push(ValleyState {
                position: new_position,
                minute: current_state.minute + 1,
                estimated_distance_to_end: 0,
            });
        }
    } else if current_row == valley_info.rows - 2 && current_col == valley_info.end_position.col {
        result.push(ValleyState {
            position: valley_info.end_position,
            minute: current_state.minute + 1,
            estimated_distance_to_end: 0,
        });
    }

    // Right
    if current_row > 0 && current_col < valley_info.cols - 2 {
        let new_position = Position {
            row: current_row,
            col: current_col + 1,
        };

        if !seen.contains(&(new_position, current_state.minute + 1))
            && !blizzards.iter().any(|b| b.position == new_position)
        {
            result.push(ValleyState {
                position: new_position,
                minute: current_state.minute + 1,
                estimated_distance_to_end: 0,
            });
        }
    }

    // Left
    if current_row > 0 && current_col > 1 {
        let new_position = Position {
            row: current_row,
            col: current_col - 1,
        };

        if !seen.contains(&(new_position, current_state.minute + 1))
            && !blizzards.iter().any(|b| b.position == new_position)
        {
            result.push(ValleyState {
                position: new_position,
                minute: current_state.minute + 1,
                estimated_distance_to_end: 0,
            });
        }
    }

    for next_state in result.iter_mut() {
        next_state.set_estimated_distance_to_end(&valley_info.end_position);
    }

    result
}

fn move_blizzards(blizzards: &[Blizzard], valley_info: &ValleyInfo) -> Vec<Blizzard> {
    let mut result = Vec::with_capacity(blizzards.len());

    for blizzard in blizzards {
        let new_row;
        let new_col;

        match blizzard.direction {
            MoveDirection::Up => {
                new_col = blizzard.position.col;

                if blizzard.position.row > 1 {
                    new_row = blizzard.position.row - 1;
                } else {
                    new_row = valley_info.rows - 2;
                }
            }
            MoveDirection::Down => {
                new_col = blizzard.position.col;

                if blizzard.position.row < valley_info.rows - 2 {
                    new_row = blizzard.position.row + 1;
                } else {
                    new_row = 1;
                }
            }
            MoveDirection::Right => {
                new_row = blizzard.position.row;

                if blizzard.position.col < valley_info.cols - 2 {
                    new_col = blizzard.position.col + 1;
                } else {
                    new_col = 1;
                }
            }
            MoveDirection::Left => {
                new_row = blizzard.position.row;

                if blizzard.position.col > 1 {
                    new_col = blizzard.position.col - 1;
                } else {
                    new_col = valley_info.cols - 2;
                }
            }
        }

        result.push(Blizzard {
            direction: blizzard.direction,
            position: Position {
                row: new_row,
                col: new_col,
            },
        });
    }

    result
}

fn read_map_of_valley_with_blizzards() -> Vec<String> {
    let mut map = vec![];

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        map.push(line.to_string());
    }

    map
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct ValleyState {
    position: Position,
    minute: usize,
    estimated_distance_to_end: usize,
}

impl ValleyState {
    fn get_score(&self) -> usize {
        self.minute + self.estimated_distance_to_end
    }

    fn set_estimated_distance_to_end(&mut self, end: &Position) {
        self.estimated_distance_to_end =
            end.row.abs_diff(self.position.row) + end.col.abs_diff(self.position.col);
    }
}

#[derive(Clone, Debug)]
struct Blizzard {
    direction: MoveDirection,
    position: Position,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

struct ValleyInfo {
    rows: usize,
    cols: usize,
    start_position: Position,
    end_position: Position,
}

#[derive(Clone, Copy, Debug)]
enum MoveDirection {
    Up,
    Down,
    Right,
    Left,
}
