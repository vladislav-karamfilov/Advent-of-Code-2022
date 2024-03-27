use std::collections::HashSet;

fn main() {
    solve_puzzle1(10);
}

fn solve_puzzle1(target_y: i32) {
    let sensor_and_closes_beacon_coordinates = read_sensor_and_closest_beacon_coordinates();

    let mut covered_coordinates = vec![];
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    for (sensor_coord, beacon_coord) in sensor_and_closes_beacon_coordinates {
        println!("{:?}", sensor_coord);
        println!("{:?}", beacon_coord);

        let manhattan_path_coordinates =
            get_coordinates_on_manhattan_path(sensor_coord, beacon_coord);

        println!("{}", manhattan_path_coordinates.len());

        covered_coordinates.extend_from_slice(&manhattan_path_coordinates);

        println!("{}", covered_coordinates.len());

        let (sensor_x, _) = sensor_coord;
        let (beacon_x, _) = beacon_coord;
        if sensor_x < min_x {
            min_x = sensor_x;
        }

        if beacon_x < min_x {
            min_x = beacon_x;
        }

        if sensor_x > max_x {
            max_x = sensor_x;
        }

        if beacon_x > max_x {
            max_x = beacon_x;
        }
    }

    let covered_x: HashSet<i32> = HashSet::from_iter(
        covered_coordinates
            .iter()
            .filter(|coord| coord.1 == target_y)
            .map(|coord| coord.0),
    );

    let mut uncovered_count = 0;
    for x in min_x..=max_x {
        if !covered_x.contains(&x) {
            uncovered_count += 1;
        }
    }

    println!("{uncovered_count}");
}

// TODO: https://stackoverflow.com/questions/75128474/how-to-generate-all-of-the-coordinates-that-are-within-a-manhattan-distance-r-of
fn get_coordinates_on_manhattan_path(start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    let mut coordinates = vec![];

    let (x1, y1) = start;
    let (x2, y2) = end;

    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();

    let mut x = x1;
    let mut y = y1;

    while x != x2 || y != y2 {
        coordinates.push((x, y));

        let next_x = x + dx;
        let next_y = y + dy;

        let dist_x = (next_x - x2).abs();
        let dist_y = (next_y - y2).abs();

        if dist_x == 0 && dist_y == 0 {
            break;
        }

        if dist_x < dist_y {
            x = next_x;
        } else {
            y = next_y;
        }
    }

    // Add the end coordinate
    coordinates.push((x2, y2));

    coordinates
}

fn read_sensor_and_closest_beacon_coordinates() -> Vec<((i32, i32), (i32, i32))> {
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

        result.push(((sensor_x, sensor_y), (beacon_x, beacon_y)));
    }

    result
}
