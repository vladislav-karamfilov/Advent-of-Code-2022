use std::collections::HashSet;
use std::fs::{read_to_string, File};
use std::io::{Error, ErrorKind, Read};

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut knot_positions = [
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
    ];

    let mut tail_visited_positions = HashSet::new();
    tail_visited_positions.insert(knot_positions.last().unwrap().clone());

    for line in read_file() {
        // let mut line = line_raw.to_string();

        // std::io::stdin()
        //     .read_line(&mut line)
        //     .expect("Failed to read line");

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
            &mut knot_positions,
            &mut tail_visited_positions,
        );

        match motion_result {
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }
    }

    println!("{}", tail_visited_positions.len());
}

fn read_file() -> Vec<String> {
    read_to_string("test.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut knot_positions = [Position { x: 0, y: 0 }, Position { x: 0, y: 0 }];

    let mut tail_visited_positions = HashSet::new();
    tail_visited_positions.insert(knot_positions.last().unwrap().clone());

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
            &mut knot_positions,
            &mut tail_visited_positions,
        );

        match motion_result {
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }
    }

    println!("{}", tail_visited_positions.len());
}

fn simulate_motion(
    direction: char,
    steps: i32,
    knot_positions: &mut [Position],
    tail_visited_positions: &mut HashSet<Position>,
) -> Result<(), Error> {
    assert!(knot_positions.len() > 1);

    for _ in 0..steps {
        match direction {
            'U' => {
                for i in 0..knot_positions.len() - 1 {
                    knot_positions[i] = Position {
                        y: knot_positions[i].y + 1,
                        ..knot_positions[i]
                    };

                    if is_tail_touching_head(&knot_positions[i], &knot_positions[i + 1]) {
                        break;
                    }

                    knot_positions[i + 1] =
                        simulate_tail_motion(&knot_positions[i], &knot_positions[i + 1]);
                }
            }

            'D' => {
                for i in 0..knot_positions.len() - 1 {
                    knot_positions[i] = Position {
                        y: knot_positions[i].y - 1,
                        ..knot_positions[i]
                    };

                    if is_tail_touching_head(&knot_positions[i], &knot_positions[i + 1]) {
                        break;
                    }

                    knot_positions[i + 1] =
                        simulate_tail_motion(&knot_positions[i], &knot_positions[i + 1]);
                }
            }

            'L' => {
                for i in 0..knot_positions.len() - 1 {
                    knot_positions[i] = Position {
                        x: knot_positions[i].x - 1,
                        ..knot_positions[i]
                    };

                    if is_tail_touching_head(&knot_positions[i], &knot_positions[i + 1]) {
                        break;
                    }

                    knot_positions[i + 1] =
                        simulate_tail_motion(&knot_positions[i], &knot_positions[i + 1]);
                }
            }

            'R' => {
                for i in 0..knot_positions.len() - 1 {
                    knot_positions[i] = Position {
                        x: knot_positions[i].x + 1,
                        ..knot_positions[i]
                    };

                    if is_tail_touching_head(&knot_positions[i], &knot_positions[i + 1]) {
                        break;
                    }

                    knot_positions[i + 1] =
                        simulate_tail_motion(&knot_positions[i], &knot_positions[i + 1]);
                }
            }

            _ => return Err(Error::new(ErrorKind::Other, "Unknown direction")),
        }

        let new_tail_position = knot_positions.last().unwrap();
        if !tail_visited_positions.contains(new_tail_position) {
            println!("adding ({}, {})", new_tail_position.x, new_tail_position.y);

            tail_visited_positions.insert(new_tail_position.clone());
        }
    }

    Ok(())
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
        // Head is on bottom right diagonal
        Position {
            x: tail_position.x + 1,
            y: tail_position.y - 1,
        }
    } else {
        // Head is on bottom left diagonal
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
