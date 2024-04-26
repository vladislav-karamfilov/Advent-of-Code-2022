fn main() {
    solve_puzzle1();
}

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

    // let mut line = String::new();

    // std::io::stdin()
    //     .read_line(&mut line)
    //     .expect("Failed to read line");

    let line = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    let jet_pattern = line.trim();

    let mut jet_pattern_index = 0usize;
    let mut stopped_rocks: Vec<Rock> = vec![];

    for i in 0..22usize {
        let rock_shape_offset_coords =
            &rock_shapes_offset_coords[i % rock_shapes_offset_coords.len()];

        perform_rock_falling(
            rock_shape_offset_coords,
            &mut stopped_rocks,
            jet_pattern,
            &mut jet_pattern_index,
        );
    }

    for row in (0..220).rev() {
        print!("|.");
        for col in 0..7 {
            if stopped_rocks
                .iter()
                .any(|r| r.coords.iter().any(|c| c.x == col && c.y == row))
            {
                print!("#");
            } else {
                print!(".");
            }
        }

        print!("|");
        println!();
    }

    let tower_height = stopped_rocks
        .iter()
        .flat_map(|r| r.coords.iter().map(|c| c.y))
        .max()
        .unwrap()
        + 1;

    println!("{tower_height}");
}

fn perform_rock_falling(
    rock_shape_offset_coords: &Vec<Coordinate2D>,
    stopped_rocks: &mut Vec<Rock>,
    jet_pattern: &str,
    jet_pattern_index: &mut usize,
) {
    let max_y: Option<usize> = stopped_rocks
        .iter()
        .flat_map(|r| r.coords.iter().map(|c| c.y))
        .max();

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

    while move_rock_in_direction(&mut rock, direction, stopped_rocks) {
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

    stopped_rocks.push(rock);
}

fn move_rock_in_direction(
    rock: &mut Rock,
    direction: Direction,
    stopped_rocks: &Vec<Rock>,
) -> bool {
    match direction {
        Direction::Down => {
            let min_rock_y = rock.get_min_y();
            if min_rock_y == 0
                || stopped_rocks.iter().any(|r| {
                    r.coords
                        .iter()
                        .any(|c| c.y == min_rock_y - 1 && rock.coords.iter().any(|rc| rc.x == c.x))
                })
            {
                return false;
            }

            for coord in rock.coords.iter_mut() {
                coord.y -= 1;
            }

            true
        }
        Direction::Left => {
            let min_rock_x = rock.get_min_x();
            if min_rock_x > 0
                && !stopped_rocks.iter().any(|r| {
                    r.coords
                        .iter()
                        .any(|c| c.x == min_rock_x - 1 && rock.coords.iter().any(|rc| rc.y == c.y))
                })
            {
                for coord in rock.coords.iter_mut() {
                    coord.x -= 1;
                }
            }

            true
        }
        Direction::Right => {
            let max_rock_x = rock.get_max_x();
            if max_rock_x < 6
                && !stopped_rocks.iter().any(|r| {
                    r.coords
                        .iter()
                        .any(|c| c.x == max_rock_x + 1 && rock.coords.iter().any(|rc| rc.y == c.y))
                })
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
