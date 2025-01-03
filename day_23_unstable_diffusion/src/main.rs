use std::collections::HashMap;

fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let grove = read_grove();

    let mut grove = enlarge_grove(grove);

    for round in 0..10 {
        simulate_elves_process_round(&mut grove, round);

        // print_grove(&grove);
    }

    // print_grove(&grove);

    let empty_ground_tiles = count_empty_ground_tiles_in_smallest_rectangle(&grove);

    println!("{empty_ground_tiles}");
}

fn count_empty_ground_tiles_in_smallest_rectangle(grove: &[Vec<char>]) -> u16 {
    let min_row = grove.iter().position(|l| l.contains(&'#')).unwrap();
    let max_row = grove.iter().rposition(|l| l.contains(&'#')).unwrap();
    let min_col = grove
        .iter()
        .map(|l| {
            l.iter()
                .position(|ch| *ch == '#')
                .or(Some(usize::MAX))
                .unwrap()
        })
        .min()
        .unwrap();

    let max_col = grove
        .iter()
        .map(|l| {
            l.iter()
                .rposition(|ch| *ch == '#')
                .or(Some(usize::MIN))
                .unwrap()
        })
        .max()
        .unwrap();

    let mut empty_ground_tiles = 0;
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if grove[row][col] == '.' {
                empty_ground_tiles += 1;
            }
        }
    }

    empty_ground_tiles
}

fn simulate_elves_process_round(grove: &mut [Vec<char>], round: u8) {
    let mut old_positions_per_proposed_position: HashMap<Position, Vec<Position>> = HashMap::new();

    for row in 0..grove.len() {
        for col in 0..grove[row].len() {
            if grove[row][col] != '#' {
                continue;
            }

            if let Some(new_position) = propose_new_position(grove, row, col, round) {
                old_positions_per_proposed_position
                    .entry(new_position)
                    .or_default()
                    .push(Position { row, col });
            }
        }
    }

    for (new_position, old_positions) in old_positions_per_proposed_position.iter() {
        if old_positions.len() == 1 {
            grove[new_position.row][new_position.col] = '#';

            let old_position = old_positions[0];
            grove[old_position.row][old_position.col] = '.';
        }
    }
}

fn propose_new_position(
    grove: &[Vec<char>],
    row: usize,
    col: usize,
    round: u8,
) -> Option<Position> {
    if !can_elf_propose_new_position(grove, row, col) {
        return None;
    }

    let move_directions_to_try = match round % 4 {
        0 => [
            MoveDirection::North,
            MoveDirection::South,
            MoveDirection::West,
            MoveDirection::East,
        ],
        1 => [
            MoveDirection::South,
            MoveDirection::West,
            MoveDirection::East,
            MoveDirection::North,
        ],
        2 => [
            MoveDirection::West,
            MoveDirection::East,
            MoveDirection::North,
            MoveDirection::South,
        ],
        _ => [
            MoveDirection::East,
            MoveDirection::North,
            MoveDirection::South,
            MoveDirection::West,
        ],
    };

    for move_direction in move_directions_to_try {
        match move_direction {
            MoveDirection::North => {
                if grove[row - 1][col - 1] == '.'
                    && grove[row - 1][col] == '.'
                    && grove[row - 1][col + 1] == '.'
                {
                    return Some(Position { row: row - 1, col });
                }
            }
            MoveDirection::South => {
                if grove[row + 1][col - 1] == '.'
                    && grove[row + 1][col] == '.'
                    && grove[row + 1][col + 1] == '.'
                {
                    return Some(Position { row: row + 1, col });
                }
            }
            MoveDirection::West => {
                if grove[row - 1][col - 1] == '.'
                    && grove[row][col - 1] == '.'
                    && grove[row + 1][col - 1] == '.'
                {
                    return Some(Position { row, col: col - 1 });
                }
            }
            MoveDirection::East => {
                if grove[row - 1][col + 1] == '.'
                    && grove[row][col + 1] == '.'
                    && grove[row + 1][col + 1] == '.'
                {
                    return Some(Position { row, col: col + 1 });
                }
            }
        }
    }

    None
}

fn can_elf_propose_new_position(grove: &[Vec<char>], row: usize, col: usize) -> bool {
    grove[row - 1][col - 1] == '#' // NW
        || grove[row - 1][col] == '#' // N
        || grove[row - 1][col + 1] == '#' // NE
        || grove[row][col - 1] == '#' // W
        || grove[row][col + 1] == '#' // E
        || grove[row + 1][col - 1] == '#' // SW
        || grove[row + 1][col] == '#' // S
        || grove[row + 1][col + 1] == '#' // SE
}

fn enlarge_grove(grove: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::with_capacity(22 + grove.len());

    let new_width = 22 + grove[0].len();

    for _ in 0..11 {
        result.push(vec!['.'; new_width]);
    }

    for line in grove {
        result.push([vec!['.'; 11], line, vec!['.'; 11]].concat());
    }

    for _ in 0..11 {
        result.push(vec!['.'; new_width]);
    }

    result
}

fn read_grove() -> Vec<Vec<char>> {
    let mut result = vec![];

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        result.push(trimmed_line.chars().collect());
    }

    result
}

#[allow(dead_code)]
fn print_grove(grove: &[Vec<char>]) {
    for row in 0..grove.len() {
        for col in 0..grove[row].len() {
            print!("{}", grove[row][col]);
        }

        println!();
    }

    println!();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy)]
enum MoveDirection {
    North,
    South,
    West,
    East,
}
