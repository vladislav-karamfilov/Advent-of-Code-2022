use std::collections::HashSet;

fn main() {
    solve_puzzle1();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let rock_shapes_offset_coords = [
        vec![
            Coordinate2D { x: 0, y: 0 },
            Coordinate2D { x: 1, y: 0 },
            Coordinate2D { x: 2, y: 0 },
            Coordinate2D { x: 3, y: 0 },
        ],
        vec![
            Coordinate2D { x: 1, y: 0 },
            Coordinate2D { x: 0, y: 1 },
            Coordinate2D { x: 1, y: 1 },
            Coordinate2D { x: 2, y: 1 },
            Coordinate2D { x: 1, y: 2 },
        ],
        vec![
            Coordinate2D { x: 0, y: 0 },
            Coordinate2D { x: 1, y: 0 },
            Coordinate2D { x: 2, y: 0 },
            Coordinate2D { x: 2, y: 1 },
            Coordinate2D { x: 2, y: 2 },
        ],
        vec![
            Coordinate2D { x: 0, y: 0 },
            Coordinate2D { x: 0, y: 1 },
            Coordinate2D { x: 0, y: 2 },
            Coordinate2D { x: 0, y: 3 },
        ],
        vec![
            Coordinate2D { x: 0, y: 0 },
            Coordinate2D { x: 1, y: 0 },
            Coordinate2D { x: 0, y: 1 },
            Coordinate2D { x: 1, y: 1 },
        ],
    ];

    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let jet_pattern = line.trim();

    let mut jet_pattern_index = 0usize;
    let mut filled_coords = HashSet::new();

    for i in 0..2022usize {
        let rock_shape_offset_coords =
            &rock_shapes_offset_coords[i % rock_shapes_offset_coords.len()];

        perform_rock_falling(
            rock_shape_offset_coords,
            &mut filled_coords,
            jet_pattern,
            &mut jet_pattern_index,
        );
    }

    let tower_height = filled_coords.iter().map(|c| c.y).max().unwrap() + 1;

    println!("{tower_height}");
}

fn perform_rock_falling(
    rock_shape_offset_coords: &Vec<Coordinate2D>,
    filled_coords: &mut HashSet<Coordinate2D>,
    jet_pattern: &str,
    jet_pattern_index: &mut usize,
) {
    let max_y = filled_coords.iter().map(|c| c.y).max();

    let mut rock = Rock {
        coords: rock_shape_offset_coords
            .iter()
            .map(|c| Coordinate2D {
                x: c.x + 2,
                y: c.y + max_y.unwrap_or_default() + (if max_y.is_some() { 4 } else { 3 }),
            })
            .collect(),
    };

    let mut direction = if jet_pattern.chars().nth(*jet_pattern_index).unwrap() == '<' {
        Direction::Left
    } else {
        Direction::Right
    };

    *jet_pattern_index = (*jet_pattern_index + 1) % jet_pattern.len();

    while move_rock_in_direction(&mut rock, direction, filled_coords) {
        // Calculate new direction for next move
        direction = if direction == Direction::Down {
            let next_direction = if jet_pattern.chars().nth(*jet_pattern_index).unwrap() == '<' {
                Direction::Left
            } else {
                Direction::Right
            };

            *jet_pattern_index = (*jet_pattern_index + 1) % jet_pattern.len();

            next_direction
        } else {
            Direction::Down
        };
    }

    for coord in rock.coords {
        filled_coords.insert(coord);
    }
}

fn move_rock_in_direction(
    rock: &mut Rock,
    direction: Direction,
    filled_coords: &HashSet<Coordinate2D>,
) -> bool {
    match direction {
        Direction::Down => {
            if rock.get_min_y() == 0
                || rock
                    .coords
                    .iter()
                    .any(|c| filled_coords.contains(&Coordinate2D { x: c.x, y: c.y - 1 }))
            {
                return false;
            }

            for coord in rock.coords.iter_mut() {
                coord.y -= 1;
            }

            true
        }
        Direction::Left => {
            if rock.get_min_x() > 0
                && !rock
                    .coords
                    .iter()
                    .any(|c| filled_coords.contains(&Coordinate2D { x: c.x - 1, y: c.y }))
            {
                for coord in rock.coords.iter_mut() {
                    coord.x -= 1;
                }
            }

            true
        }
        Direction::Right => {
            if rock.get_max_x() < 6
                && !rock
                    .coords
                    .iter()
                    .any(|c| filled_coords.contains(&Coordinate2D { x: c.x + 1, y: c.y }))
            {
                for coord in rock.coords.iter_mut() {
                    coord.x += 1;
                }
            }

            true
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coordinate2D {
    x: usize,
    y: usize,
}

struct Rock {
    coords: Vec<Coordinate2D>,
}

impl Rock {
    fn get_max_x(&self) -> usize {
        self.coords.iter().map(|c| c.x).max().unwrap()
    }

    fn get_min_x(&self) -> usize {
        self.coords.iter().map(|c| c.x).min().unwrap()
    }

    fn get_min_y(&self) -> usize {
        self.coords.iter().map(|c| c.y).min().unwrap()
    }
}
