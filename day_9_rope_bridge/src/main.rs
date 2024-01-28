use std::collections::HashSet;
use std::io::{Error, ErrorKind};

fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };
    let mut tail_visited_positions = HashSet::new();
    tail_visited_positions.insert(tail_position.clone());

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let (direction, steps) = line.split_at(2);

        let direction = direction.chars().next().unwrap();
        let steps = steps.parse().expect("Steps must be number");

        let motion_result = simulate_motion(
            direction,
            steps,
            head_position,
            tail_position,
            &mut tail_visited_positions,
        );

        match motion_result {
            Ok(v) => (head_position, tail_position) = v,
            Err(err) => panic!("{err}"),
        }
    }

    println!("{}", tail_visited_positions.len());
}

fn simulate_motion(
    direction: char,
    steps: i32,
    head_position: Position,
    tail_position: Position,
    tail_visited_positions: &mut HashSet<Position>,
) -> Result<(Position, Position), Error> {
    let mut new_head_position = head_position;
    let mut new_tail_position = tail_position;

    for _ in 0..steps {
        match direction {
            'U' => {
                new_head_position = Position {
                    y: new_head_position.y + 1,
                    ..new_head_position
                };

                if !is_tail_touching_head(&new_head_position, &new_tail_position) {
                    new_tail_position =
                        simulate_tail_motion(&new_head_position, &new_tail_position);

                    tail_visited_positions.insert(new_tail_position.clone());
                }
            }

            'D' => {
                new_head_position = Position {
                    y: new_head_position.y - 1,
                    ..new_head_position
                };

                if !is_tail_touching_head(&new_head_position, &new_tail_position) {
                    new_tail_position =
                        simulate_tail_motion(&new_head_position, &new_tail_position);

                    tail_visited_positions.insert(new_tail_position.clone());
                }
            }

            'L' => {
                new_head_position = Position {
                    x: new_head_position.x - 1,
                    ..new_head_position
                };

                if !is_tail_touching_head(&new_head_position, &new_tail_position) {
                    new_tail_position =
                        simulate_tail_motion(&new_head_position, &new_tail_position);

                    tail_visited_positions.insert(new_tail_position.clone());
                }
            }

            'R' => {
                new_head_position = Position {
                    x: new_head_position.x + 1,
                    ..new_head_position
                };

                if !is_tail_touching_head(&new_head_position, &new_tail_position) {
                    new_tail_position =
                        simulate_tail_motion(&new_head_position, &new_tail_position);

                    tail_visited_positions.insert(new_tail_position.clone());
                }
            }

            _ => return Err(Error::new(ErrorKind::Other, "Unknown direction")),
        }
    }

    Ok((new_head_position, new_tail_position))
}

fn simulate_tail_motion(head_position: &Position, tail_position: &Position) -> Position {
    // On same row
    if head_position.x == tail_position.x {
        return if head_position.y > tail_position.y {
            Position {
                y: head_position.y - 1,
                ..*head_position
            }
        } else {
            Position {
                y: head_position.y + 1,
                ..*head_position
            }
        };
    }

    // On same col
    if head_position.y == tail_position.y {
        return if head_position.x > tail_position.x {
            Position {
                x: head_position.x - 1,
                ..*head_position
            }
        } else {
            Position {
                x: head_position.x + 1,
                ..*head_position
            }
        };
    }

    if head_position.y > tail_position.y {
        if head_position.x > tail_position.x {
            // Head is on upper right diagonal
            Position {
                x: tail_position.x + 1,
                y: tail_position.y + 1,
            }
        } else {
            // Head is on upper left diagonal
            Position {
                x: tail_position.x - 1,
                y: tail_position.y + 1,
            }
        }
    } else if head_position.x > tail_position.x {
        // Head is on bottom left diagonal
        Position {
            x: tail_position.x + 1,
            y: tail_position.y - 1,
        }
    } else {
        // Head is on bottom right diagonal
        Position {
            x: tail_position.x - 1,
            y: tail_position.y - 1,
        }
    }
}

fn is_tail_touching_head(head_position: &Position, tail_position: &Position) -> bool {
    let x_diff = (head_position.x - tail_position.x).abs();
    let y_diff = (head_position.y - tail_position.y).abs();

    x_diff <= 1 && y_diff <= 1
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}
