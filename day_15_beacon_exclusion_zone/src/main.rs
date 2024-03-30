fn main() {
    // solve_puzzle1(10);
    // solve_puzzle1(2_000_000);

    // solve_puzzle2(20);
    solve_puzzle2(4_000_000);
}

#[allow(dead_code)]
fn solve_puzzle2(max_coord_value: i64) {
    let (sensor_coords, beacon_coords) = read_sensor_and_closest_beacon_coordinates();

    let all_covered_areas = calculate_all_covered_areas(&sensor_coords, &beacon_coords);

    for y in 0..=max_coord_value {
        for x in 0..=max_coord_value {
            let coord = Coordinate2D { x, y };
            if sensor_coords.contains(&coord)
                || beacon_coords.contains(&coord)
                || is_coordinate_inside_covered_area_or_on_border(coord, &all_covered_areas)
            {
                continue;
            }

            println!("{:?}", coord);
            println!("{}", x * 4_000_000 + y);
            return;
        }
    }
}

#[allow(dead_code)]
fn solve_puzzle1(target_y: i64) {
    let (sensor_coords, beacon_coords) = read_sensor_and_closest_beacon_coordinates();

    let all_covered_areas = calculate_all_covered_areas(&sensor_coords, &beacon_coords);

    let min_x = all_covered_areas
        .iter()
        .flat_map(|coords| coords.iter().map(|coord| coord.x))
        .min()
        .unwrap();

    let max_x = all_covered_areas
        .iter()
        .flat_map(|coords| coords.iter().map(|coord| coord.x))
        .max()
        .unwrap();

    let mut covered_x_count = 0;

    for x in min_x..=max_x {
        let coord = Coordinate2D { x, y: target_y };
        if sensor_coords.contains(&coord) || beacon_coords.contains(&coord) {
            continue;
        }

        if is_coordinate_inside_covered_area_or_on_border(coord, &all_covered_areas) {
            covered_x_count += 1;
        }
    }

    println!("{covered_x_count}");
}

fn calculate_all_covered_areas(
    sensor_coords: &[Coordinate2D],
    beacon_coords: &[Coordinate2D],
) -> Vec<Vec<Coordinate2D>> {
    let mut all_covered_areas = vec![];
    for (i, sensor_coord) in sensor_coords.iter().enumerate() {
        let beacon_coord = beacon_coords[i];

        let manhattan_distance = calculate_manhattan_distance(*sensor_coord, beacon_coord);

        let covered_area_vertices =
            calculate_covered_area_vertices(*sensor_coord, manhattan_distance);

        all_covered_areas.push(covered_area_vertices);
    }

    all_covered_areas
}

fn is_coordinate_on_segment(coord: Coordinate2D, start: Coordinate2D, end: Coordinate2D) -> bool {
    calculate_distance_between_coordinates(start, coord)
        + calculate_distance_between_coordinates(end, coord)
        == calculate_distance_between_coordinates(start, end)
}

fn calculate_distance_between_coordinates(a: Coordinate2D, b: Coordinate2D) -> f64 {
    ((a.x - b.x).pow(2) as f64 + (a.y - b.y).pow(2) as f64).sqrt()
}

fn is_coordinate_inside_covered_area_or_on_border(
    coord: Coordinate2D,
    all_covered_areas: &[Vec<Coordinate2D>],
) -> bool {
    for covered_area_vertices in all_covered_areas {
        let mut inside = false;
        let mut i = 0;
        let mut j = covered_area_vertices.len() - 1;

        while i < covered_area_vertices.len() {
            if is_coordinate_on_segment(coord, covered_area_vertices[i], covered_area_vertices[j]) {
                return true; // On border
            }

            // Approach: Point Inclusion in Polygon (https://wrfranklin.org/Research/Short_Notes/pnpoly.html)
            if (covered_area_vertices[i].y > coord.y) != (covered_area_vertices[j].y > coord.y)
                && coord.x
                    < (covered_area_vertices[j].x - covered_area_vertices[i].x)
                        * (coord.y - covered_area_vertices[i].y)
                        / (covered_area_vertices[j].y - covered_area_vertices[i].y)
                        + covered_area_vertices[i].x
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

fn calculate_manhattan_distance(start: Coordinate2D, end: Coordinate2D) -> u64 {
    start.x.abs_diff(end.x) + start.y.abs_diff(end.y)
}

fn calculate_covered_area_vertices(
    sensor_coord: Coordinate2D,
    manhattan_distance: u64,
) -> Vec<Coordinate2D> {
    let mut result = Vec::with_capacity(4);

    // Approach: https://stackoverflow.com/questions/75128474/how-to-generate-all-of-the-coordinates-that-are-within-a-manhattan-distance-r-of#answer-75129338
    let max_offset = manhattan_distance as i64;
    result.push(Coordinate2D {
        x: sensor_coord.x + max_offset,
        y: sensor_coord.y,
    });

    result.push(Coordinate2D {
        x: sensor_coord.x,
        y: sensor_coord.y - max_offset,
    });

    result.push(Coordinate2D {
        x: sensor_coord.x - max_offset,
        y: sensor_coord.y,
    });

    result.push(Coordinate2D {
        x: sensor_coord.x,
        y: sensor_coord.y + max_offset,
    });

    result
}

fn read_sensor_and_closest_beacon_coordinates() -> (Vec<Coordinate2D>, Vec<Coordinate2D>) {
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
            .parse::<i64>()
            .unwrap();

        let sensor_y_start = line.find("y=").unwrap();
        let sensor_y_end = line.find(':').unwrap();
        let sensor_y = line[sensor_y_start + 2..sensor_y_end]
            .parse::<i64>()
            .unwrap();

        let beacon_x_start = sensor_y_end + line[sensor_y_end..].find("x=").unwrap();
        let beacon_x_end = beacon_x_start + line[beacon_x_start..].find(',').unwrap();
        let beacon_x = line[beacon_x_start + 2..beacon_x_end]
            .parse::<i64>()
            .unwrap();

        let beacon_y_start = sensor_y_end + line[sensor_y_end..].find("y=").unwrap();
        let beacon_y_end = line.len();
        let beacon_y = line[beacon_y_start + 2..beacon_y_end]
            .parse::<i64>()
            .unwrap();

        result.0.push(Coordinate2D {
            x: sensor_x,
            y: sensor_y,
        });

        result.1.push(Coordinate2D {
            x: beacon_x,
            y: beacon_y,
        });
    }

    result
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coordinate2D {
    x: i64,
    y: i64,
}
