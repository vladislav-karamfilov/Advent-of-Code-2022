use std::str::Split;

fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let mut is_reading_rearrangement_steps = false;
    let mut stacks_of_crates_lines = Vec::new();
    let mut stacks_of_crates: Vec<Vec<char>> = Vec::new();

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            if is_reading_rearrangement_steps {
                print_result(&stacks_of_crates);
                break;
            }

            is_reading_rearrangement_steps = true;
            continue;
        }

        if is_reading_rearrangement_steps {
            let mut rearrangement_step_splitter = trimmed_line.split(' ');

            let count = parse_next_number_from_rearrangement_step_splitter(
                &mut rearrangement_step_splitter,
            );

            let from_stack_index = parse_next_number_from_rearrangement_step_splitter(
                &mut rearrangement_step_splitter,
            ) - 1;

            let to_stack_index = parse_next_number_from_rearrangement_step_splitter(
                &mut rearrangement_step_splitter,
            ) - 1;

            rearrange_stacks(
                &mut stacks_of_crates,
                count,
                from_stack_index,
                to_stack_index,
            );
        } else {
            let line_chars = line.chars();
            if line_chars.clone().any(|char| char.is_ascii_digit()) {
                for (i, char) in line_chars.enumerate() {
                    if char.is_ascii_digit() {
                        let crates_stack: Vec<char> = stacks_of_crates_lines
                            .iter()
                            .rev()
                            .map(|crates_line: &String| crates_line.chars().nth(i).unwrap())
                            .filter(|char| char.is_ascii_alphabetic())
                            .collect();

                        stacks_of_crates.push(crates_stack);
                    }
                }
            } else {
                stacks_of_crates_lines.push(line);
            }
        }
    }
}

fn print_result(stacks_of_crates: &[Vec<char>]) {
    for stack in stacks_of_crates {
        print!("{}", stack.last().unwrap());
    }

    println!();
}

fn parse_next_number_from_rearrangement_step_splitter(
    rearrangement_step_splitter: &mut Split<'_, char>,
) -> usize {
    rearrangement_step_splitter.next();

    let number: usize = rearrangement_step_splitter
        .next()
        .unwrap()
        .parse()
        .expect("Number expected");

    number
}

fn rearrange_stacks(
    stacks_of_crates: &mut [Vec<char>],
    count: usize,
    from_stack_index: usize,
    to_stack_index: usize,
) {
    for _ in 0..count {
        let crate_to_rearrange = stacks_of_crates[from_stack_index].pop().unwrap();

        stacks_of_crates[to_stack_index].push(crate_to_rearrange);
    }
}
