use std::collections::HashSet;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut datastream_buffer = String::new();

    std::io::stdin()
        .read_line(&mut datastream_buffer)
        .expect("Failed to read line");

    println!(
        "{}",
        calculate_processed_chars_before_marker(&datastream_buffer, 14).unwrap()
    );
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut datastream_buffer = String::new();

    std::io::stdin()
        .read_line(&mut datastream_buffer)
        .expect("Failed to read line");

    println!(
        "{}",
        calculate_processed_chars_before_marker(&datastream_buffer, 4).unwrap()
    );
}

fn calculate_processed_chars_before_marker(
    datastream_buffer: &str,
    chars_count: usize,
) -> Option<usize> {
    let mut processed_chars = Vec::new();

    for (i, char) in datastream_buffer.chars().enumerate() {
        processed_chars.push(char);

        if i < chars_count - 1 {
            continue;
        }

        let unique_chars_count = processed_chars.iter().collect::<HashSet<&char>>().len();
        if unique_chars_count == chars_count {
            return Some(i + 1);
        }

        processed_chars.remove(0);
    }

    None
}
