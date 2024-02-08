use std::collections::HashSet;

use priority_queue::PriorityQueue;

fn main() {
    solve_puzzle1();
}

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

    let min_steps =
        calculate_min_steps_to_destination(&heightmap, (start_row, start_col), (end_row, end_col));

    println!("{min_steps}");
}

fn calculate_min_steps_to_destination(
    heightmap: &[Vec<u8>],
    start_location: (usize, usize),
    destination_location: (usize, usize),
) -> i32 {
    let mut start_square = SquareToVisit {
        location: start_location,
        steps: 0,
        estimated_distance_to_destination: 0,
    };

    start_square.set_estimated_distance_to_destination(destination_location);

    let start_square_score = start_square.get_score();

    let mut squares_to_visit = PriorityQueue::new();
    squares_to_visit.push(start_square, start_square_score);

    let mut visited = HashSet::new();
    let mut neighbour_squares_in_queue = HashSet::new();

    while let Some((current_square, _)) = squares_to_visit.pop() {
        if current_square.location == destination_location {
            return current_square.steps;
        }

        visited.insert(current_square.location);

        let neighbour_squares =
            get_neighbour_squares(heightmap, &current_square, destination_location);

        for neighbour_square in neighbour_squares {
            if visited.contains(&neighbour_square.location)
                || neighbour_squares_in_queue.contains(&neighbour_square.location)
            {
                continue;
            }

            neighbour_squares_in_queue.insert(neighbour_square.location);

            let neighbour_square_score = neighbour_square.get_score();
            squares_to_visit.push(neighbour_square, neighbour_square_score);
        }
    }

    -1
}

fn get_neighbour_squares(
    heightmap: &[Vec<u8>],
    current_square: &SquareToVisit,
    destination_location: (usize, usize),
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
            estimated_distance_to_destination: 0,
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
            estimated_distance_to_destination: 0,
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
            estimated_distance_to_destination: 0,
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
            estimated_distance_to_destination: 0,
        });
    }

    for neighbour_square in result.iter_mut() {
        neighbour_square.set_estimated_distance_to_destination(destination_location);
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

#[derive(Hash, Eq, PartialEq)]
struct SquareToVisit {
    location: (usize, usize),
    steps: i32,
    estimated_distance_to_destination: i32,
}

impl SquareToVisit {
    fn get_score(&self) -> i32 {
        self.estimated_distance_to_destination + self.steps
    }

    fn set_estimated_distance_to_destination(&mut self, destination: (usize, usize)) {
        self.estimated_distance_to_destination = (destination.0 as i32 - self.location.0 as i32)
            .abs()
            + (destination.1 as i32 - self.location.1 as i32).abs();
    }
}
