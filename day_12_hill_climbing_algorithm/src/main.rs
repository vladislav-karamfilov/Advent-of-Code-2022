use std::collections::{HashSet, VecDeque};

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut heightmap = read_heightmap();

    let start = b'S';
    let start_row = heightmap.iter().position(|x| x.contains(&start)).unwrap();
    let start_col = heightmap[start_row]
        .iter()
        .position(|x| *x == start)
        .unwrap();

    heightmap[start_row][start_col] = b'a';

    let mut start_locations = vec![];

    for (row, line) in heightmap.iter().enumerate() {
        for (col, _) in line.iter().enumerate() {
            if line[col] == b'a' {
                start_locations.push((row, col));
            }
        }
    }

    let end = b'E';
    let end_row = heightmap.iter().position(|x| x.contains(&end)).unwrap();
    let end_col = heightmap[end_row].iter().position(|x| *x == end).unwrap();

    heightmap[end_row][end_col] = b'z';

    let min_steps =
        calculate_min_steps_to_destination(&heightmap, &start_locations, (end_row, end_col));

    println!("{min_steps}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut heightmap = read_heightmap();

    let start = b'S';
    let start_row = heightmap.iter().position(|x| x.contains(&start)).unwrap();
    let start_col = heightmap[start_row]
        .iter()
        .position(|x| *x == start)
        .unwrap();

    let end = b'E';
    let end_row = heightmap.iter().position(|x| x.contains(&end)).unwrap();
    let end_col = heightmap[end_row].iter().position(|x| *x == end).unwrap();

    heightmap[start_row][start_col] = b'a';
    heightmap[end_row][end_col] = b'z';

    let min_steps = calculate_min_steps_to_destination(
        &heightmap,
        &[(start_row, start_col)],
        (end_row, end_col),
    );

    println!("{min_steps}");
}

// Implementation of A* search algorithm: https://en.wikipedia.org/wiki/A*_search_algorithm
fn calculate_min_steps_to_destination(
    heightmap: &[Vec<u8>],
    start_locations: &[(usize, usize)],
    destination_location: (usize, usize),
) -> i32 {
    let mut squares_to_visit = VecDeque::new();
    start_locations.iter().for_each(|l| {
        squares_to_visit.push_back(SquareToVisit {
            location: *l,
            steps: 0,
        })
    });

    let mut visited = HashSet::new();

    while !squares_to_visit.is_empty() {
        let current_square = squares_to_visit.pop_front().unwrap();
        if current_square.location == destination_location {
            return current_square.steps;
        }

        if !visited.insert(current_square.location) {
            continue;
        }

        let neighbour_squares = get_neighbour_squares(heightmap, &current_square);

        for neighbour_square in neighbour_squares {
            if visited.contains(&neighbour_square.location) {
                continue;
            }

            squares_to_visit.push_back(neighbour_square);
        }
    }

    -1
}

fn get_neighbour_squares(
    heightmap: &[Vec<u8>],
    current_square: &SquareToVisit,
) -> Vec<SquareToVisit> {
    let mut result = vec![];

    let max_allowed_square_elevation =
        heightmap[current_square.location.0][current_square.location.1] + 1;

    // Up
    if current_square.location.0 > 0
        && heightmap[current_square.location.0 - 1][current_square.location.1]
            <= max_allowed_square_elevation
    {
        result.push(SquareToVisit {
            location: (current_square.location.0 - 1, current_square.location.1),
            steps: current_square.steps + 1,
        });
    }

    // Down
    if current_square.location.0 < heightmap.len() - 1
        && heightmap[current_square.location.0 + 1][current_square.location.1]
            <= max_allowed_square_elevation
    {
        result.push(SquareToVisit {
            location: (current_square.location.0 + 1, current_square.location.1),
            steps: current_square.steps + 1,
        });
    }

    // Left
    if current_square.location.1 > 0
        && heightmap[current_square.location.0][current_square.location.1 - 1]
            <= max_allowed_square_elevation
    {
        result.push(SquareToVisit {
            location: (current_square.location.0, current_square.location.1 - 1),
            steps: current_square.steps + 1,
        });
    }

    // Right
    if current_square.location.1 < heightmap[0].len() - 1
        && heightmap[current_square.location.0][current_square.location.1 + 1]
            <= max_allowed_square_elevation
    {
        result.push(SquareToVisit {
            location: (current_square.location.0, current_square.location.1 + 1),
            steps: current_square.steps + 1,
        });
    }

    result
}

fn read_heightmap() -> Vec<Vec<u8>> {
    let mut result = vec![];

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        result.push(line.bytes().collect());
    }

    result
}

struct SquareToVisit {
    location: (usize, usize),
    steps: i32,
}
