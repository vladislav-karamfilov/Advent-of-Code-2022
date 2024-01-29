use std::collections::HashSet;
use std::io::{Error, ErrorKind};

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    println!("{}", calculate_tail_visited_positions(10));
}

#[allow(dead_code)]
fn solve_puzzle1() {
    println!("{}", calculate_tail_visited_positions(2));
}

fn calculate_tail_visited_positions(knots_count: usize) -> usize {
    let mut knot_positions = vec![KnotPosition { x: 0, y: 0 }; knots_count];

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

    tail_visited_positions.len()
}

fn simulate_motion(
    direction: char,
    steps: i32,
    knot_positions: &mut [KnotPosition],
    tail_visited_positions: &mut HashSet<KnotPosition>,
) -> Result<(), Error> {
    assert!(knot_positions.len() > 1);

    for _ in 0..steps {
        match direction {
            'U' => {
                knot_positions[0] = KnotPosition {
                    y: knot_positions[0].y + 1,
                    ..knot_positions[0]
                };
            }

            'D' => {
                knot_positions[0] = KnotPosition {
                    y: knot_positions[0].y - 1,
                    ..knot_positions[0]
                };
            }

            'L' => {
                knot_positions[0] = KnotPosition {
                    x: knot_positions[0].x - 1,
                    ..knot_positions[0]
                };
            }

            'R' => {
                knot_positions[0] = KnotPosition {
                    x: knot_positions[0].x + 1,
                    ..knot_positions[0]
                };
            }

            _ => return Err(Error::new(ErrorKind::Other, "Unknown direction")),
        }

        let mut i = 1;
        while i < knot_positions.len()
            && !is_tail_touching_head(&knot_positions[i - 1], &knot_positions[i])
        {
            knot_positions[i] = simulate_tail_motion(&knot_positions[i - 1], &knot_positions[i]);

            i += 1;
        }

        let new_tail_position = knot_positions.last().unwrap();
        if !tail_visited_positions.contains(new_tail_position) {
            tail_visited_positions.insert(new_tail_position.clone());
        }
    }

    Ok(())
}

fn simulate_tail_motion(
    head_position: &KnotPosition,
    tail_position: &KnotPosition,
) -> KnotPosition {
    // On same row
    if head_position.x == tail_position.x {
        return if head_position.y > tail_position.y {
            KnotPosition {
                y: head_position.y - 1,
                ..*head_position
            }
        } else {
            KnotPosition {
                y: head_position.y + 1,
                ..*head_position
            }
        };
    }

    // On same col
    if head_position.y == tail_position.y {
        return if head_position.x > tail_position.x {
            KnotPosition {
                x: head_position.x - 1,
                ..*head_position
            }
        } else {
            KnotPosition {
                x: head_position.x + 1,
                ..*head_position
            }
        };
    }

    if head_position.y > tail_position.y {
        if head_position.x > tail_position.x {
            // Head is on upper right diagonal
            KnotPosition {
                x: tail_position.x + 1,
                y: tail_position.y + 1,
            }
        } else {
            // Head is on upper left diagonal
            KnotPosition {
                x: tail_position.x - 1,
                y: tail_position.y + 1,
            }
        }
    } else if head_position.x > tail_position.x {
        // Head is on bottom right diagonal
        KnotPosition {
            x: tail_position.x + 1,
            y: tail_position.y - 1,
        }
    } else {
        // Head is on bottom left diagonal
        KnotPosition {
            x: tail_position.x - 1,
            y: tail_position.y - 1,
        }
    }
}

fn is_tail_touching_head(head_position: &KnotPosition, tail_position: &KnotPosition) -> bool {
    let x_diff = (head_position.x - tail_position.x).abs();
    let y_diff = (head_position.y - tail_position.y).abs();

    x_diff <= 1 && y_diff <= 1
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct KnotPosition {
    x: i32,
    y: i32,
}
