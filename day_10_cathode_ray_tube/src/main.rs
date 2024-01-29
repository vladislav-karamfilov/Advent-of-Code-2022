use std::io::{Error, ErrorKind};

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut result_image = String::with_capacity(240);

    let mut x_register_value = 1;
    let mut drawn_col_position = 0;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let (instruction, raw_value) = line.split_at(4);

        let instruction_result =
            handle_instruction(instruction, raw_value, x_register_value, &mut || {
                if drawn_col_position == x_register_value - 1
                    || drawn_col_position == x_register_value
                    || drawn_col_position == x_register_value + 1
                {
                    result_image.push('#');
                } else {
                    result_image.push('.');
                }

                if drawn_col_position == 39 {
                    drawn_col_position = 0;

                    result_image.push('\n');
                } else {
                    drawn_col_position += 1;
                }
            });

        match instruction_result {
            Ok(v) => x_register_value = v,
            Err(err) => panic!("{err}"),
        }
    }

    println!("{result_image}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut signal_strengths = Vec::with_capacity(220);

    let mut x_register_value = 1;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let (instruction, raw_value) = line.split_at(4);

        let instruction_result =
            handle_instruction(instruction, raw_value, x_register_value, &mut || {
                signal_strengths.push(x_register_value * (signal_strengths.len() as i32 + 1))
            });

        match instruction_result {
            Ok(v) => x_register_value = v,
            Err(err) => panic!("{err}"),
        }
    }

    let sum = signal_strengths[19]
        + signal_strengths[59]
        + signal_strengths[99]
        + signal_strengths[139]
        + signal_strengths[179]
        + signal_strengths[219];

    println!("{sum}");
}

fn handle_instruction<F>(
    instruction: &str,
    raw_value: &str,
    x_register_value: i32,
    cycle_action: &mut F,
) -> Result<i32, Error>
where
    F: FnMut(),
{
    let mut new_x_register_value = x_register_value;

    match instruction {
        "noop" => cycle_action(),

        "addx" => {
            cycle_action();
            cycle_action();

            let value = match raw_value.trim().parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Register value must be number but is: {}", raw_value),
                    ))
                }
            };

            new_x_register_value += value;
        }

        _ => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Unknown instruction: {}", instruction),
            ))
        }
    }

    Ok(new_x_register_value)
}
