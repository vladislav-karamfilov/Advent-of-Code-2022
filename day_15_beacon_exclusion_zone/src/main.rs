fn main() {
    solve_puzzle1(10);
    // solve_puzzle1(2000000);
}

fn solve_puzzle1(target_y: i32) {
    let (sensor_coords, beacon_coords) = read_sensor_and_closest_beacon_coordinates();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut all_covered_areas = vec![];

    for (i, sensor_coord) in sensor_coords.iter().enumerate() {
        let beacon_coord = beacon_coords[i];

        let manhattan_distance = calculate_manhattan_distance(*sensor_coord, beacon_coord);

        let covered_area_edges =
            calculate_covered_area_edges(sensor_coord.0, sensor_coord.1, manhattan_distance);

        let min_covered_x = covered_area_edges
            .iter()
            .map(|coord| coord.0)
            .min()
            .unwrap();

        if min_x > min_covered_x {
            min_x = min_covered_x;
        }

        let max_covered_x = covered_area_edges
            .iter()
            .map(|coord| coord.0)
            .max()
            .unwrap();

        if max_x < max_covered_x {
            max_x = max_covered_x;
        }

        all_covered_areas.push(covered_area_edges);

        print_area(&sensor_coords, &beacon_coords, &all_covered_areas);
        println!();
        println!();
    }

    // print_area(&sensor_coords, &beacon_coords, &all_covered_areas);

    let mut covered_x = 0;

    for x in min_x..=max_x {
        let coord = (x, target_y);
        if sensor_coords.contains(&coord) || beacon_coords.contains(&coord) {
            continue;
        }

        if is_coordinate_inside_area(&coord, &all_covered_areas) {
            covered_x += 1;
        }
    }

    println!("{covered_x}");
}

fn print_area(
    sensor_coords: &[(i32, i32)],
    beacon_coords: &[(i32, i32)],
    all_covered_areas: &[Vec<(i32, i32)>],
) {
    for y in -10..32 {
        for x in -20..32 {
            let coord = (x, y);
            if sensor_coords.contains(&coord) {
                print!("S");
            } else if beacon_coords.contains(&coord) {
                print!("B");
            } else if all_covered_areas
                .iter()
                .any(|coords| coords.contains(&coord))
            {
                print!("V");
            } else if is_coordinate_inside_area(&coord, all_covered_areas) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

fn is_coordinate_inside_area(coord: &(i32, i32), all_covered_areas: &[Vec<(i32, i32)>]) -> bool {
    for covered_area_edges in all_covered_areas {
        if covered_area_edges.contains(coord) {
            return true;
        }

        // TODO: Check if on a line?
        let mut inside = false;
        let mut i = 0;
        let mut j = covered_area_edges.len() - 1;

        while i < covered_area_edges.len() {
            if (covered_area_edges[i].1 > coord.1) != (covered_area_edges[j].1 > coord.1)
                && (coord.0 as i64)
                    < (covered_area_edges[j].0 - covered_area_edges[i].0) as i64
                        * (coord.1 - covered_area_edges[i].1) as i64
                        / (covered_area_edges[j].1 - covered_area_edges[i].1) as i64
                        + covered_area_edges[i].0 as i64
            {
                inside = !inside;
            }

            j = i;
            i += 1;
        }

        if inside {
            return true;
        }
    }

    false
}

fn calculate_manhattan_distance(start: (i32, i32), end: (i32, i32)) -> u32 {
    start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
}

fn calculate_covered_area_edges(
    start_x: i32,
    start_y: i32,
    manhattan_distance: u32,
) -> Vec<(i32, i32)> {
    let mut result = Vec::with_capacity(5);

    // Approach: https://stackoverflow.com/questions/75128474/how-to-generate-all-of-the-coordinates-that-are-within-a-manhattan-distance-r-of#answer-75129338
    let max_offset = manhattan_distance as i32;
    result.push((start_x + max_offset, start_y));
    result.push((start_x, start_y - max_offset));
    result.push((start_x - max_offset, start_y));
    result.push((start_x, start_y + max_offset));
    result.push((start_x + max_offset, start_y));

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
