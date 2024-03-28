use std::collections::HashSet;

fn main() {
    // solve_puzzle1(10);
    solve_puzzle1(2000000);
}

fn solve_puzzle1(target_y: i32) {
    let (sensor_coords, beacon_coords) = read_sensor_and_closest_beacon_coordinates();

    let mut covered_coordinates: HashSet<(i32, i32)> = HashSet::new();
    for (i, sensor_coord) in sensor_coords.iter().enumerate() {
        let beacon_coord = beacon_coords[i];

        let manhattan_distance = calculate_manhattan_distance(*sensor_coord, beacon_coord);

        // The whole "manhattan area" won't cross the target Y
        let min_y = sensor_coord.1.min(beacon_coord.1) - manhattan_distance as i32;
        let max_y = sensor_coord.1.max(beacon_coord.1) + manhattan_distance as i32;
        if min_y > target_y || max_y < target_y {
            continue;
        }

        let manhattan_area_coordinates =
            get_coordinates_in_manhattan_area(sensor_coord.0, sensor_coord.1, manhattan_distance);

        covered_coordinates.extend(&manhattan_area_coordinates);
    }

    let covered_x = covered_coordinates
        .iter()
        .filter(|coord| {
            coord.1 == target_y && !sensor_coords.contains(coord) && !beacon_coords.contains(coord)
        })
        .count();

    println!("{covered_x}");
}

fn calculate_manhattan_distance(start: (i32, i32), end: (i32, i32)) -> u32 {
    start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
}

fn get_coordinates_in_manhattan_area(
    start_x: i32,
    start_y: i32,
    manhattan_distance: u32,
) -> Vec<(i32, i32)> {
    let mut result = Vec::with_capacity(4 * manhattan_distance as usize);

    let manhattan_distance = manhattan_distance as i32;

    // Approach: https://stackoverflow.com/questions/75128474/how-to-generate-all-of-the-coordinates-that-are-within-a-manhattan-distance-r-of#answer-75129338
    for offset in 0..manhattan_distance {
        let inverse_offset = manhattan_distance - offset;
        result.push((start_x + offset, start_y + inverse_offset));
        result.push((start_x + inverse_offset, start_y - offset));
        result.push((start_x - offset, start_y - inverse_offset));
        result.push((start_x - inverse_offset, start_y + offset));
    }

    let min_y = result.iter().map(|coord| coord.1).min().unwrap();
    let max_y = result.iter().map(|coord| coord.1).max().unwrap();

    // Fill in the star-like shape
    for y in min_y + 1..max_y {
        let min_x = result
            .iter()
            .filter(|coord| coord.1 == y)
            .map(|coord| coord.0)
            .min()
            .unwrap();

        let max_x = result
            .iter()
            .filter(|coord| coord.1 == y)
            .map(|coord| coord.0)
            .max()
            .unwrap();

        for x in min_x + 1..max_x {
            result.push((x, y));
        }
    }

    result
}

fn read_sensor_and_closest_beacon_coordinates() -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut result = (vec![], vec![]);

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let sensor_x_start = line.find("x=").unwrap();
        let sensor_x_end = line.find(',').unwrap();
        let sensor_x = line[sensor_x_start + 2..sensor_x_end]
            .parse::<i32>()
            .unwrap();

        let sensor_y_start = line.find("y=").unwrap();
        let sensor_y_end = line.find(':').unwrap();
        let sensor_y = line[sensor_y_start + 2..sensor_y_end]
            .parse::<i32>()
            .unwrap();

        let beacon_x_start = sensor_y_end + line[sensor_y_end..].find("x=").unwrap();
        let beacon_x_end = beacon_x_start + line[beacon_x_start..].find(',').unwrap();
        let beacon_x = line[beacon_x_start + 2..beacon_x_end]
            .parse::<i32>()
            .unwrap();

        let beacon_y_start = sensor_y_end + line[sensor_y_end..].find("y=").unwrap();
        let beacon_y_end = line.len();
        let beacon_y = line[beacon_y_start + 2..beacon_y_end]
            .parse::<i32>()
            .unwrap();

        result.0.push((sensor_x, sensor_y));
        result.1.push((beacon_x, beacon_y));
    }

    result
}
