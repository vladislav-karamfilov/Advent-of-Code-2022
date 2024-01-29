use std::io::{Error, ErrorKind};

fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let mut x_register_values = Vec::with_capacity(220);

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

        match handle_instruction(
            instruction,
            raw_value,
            x_register_value,
            &mut x_register_values,
        ) {
            Ok(v) => x_register_value = v,
            Err(err) => panic!("{err}"),
        }
    }

    let sum = (x_register_values[19] * 20)
        + (x_register_values[59] * 60)
        + (x_register_values[99] * 100)
        + (x_register_values[139] * 140)
        + (x_register_values[179] * 180)
        + (x_register_values[219] * 220);

    println!("{sum}");
}

fn handle_instruction(
    instruction: &str,
    raw_value: &str,
    x_register_value: i32,
    x_register_values: &mut Vec<i32>,
) -> Result<i32, Error> {
    let mut new_x_register_value = x_register_value;

    match instruction {
        "noop" => x_register_values.push(x_register_value),

        "addx" => {
            x_register_values.push(x_register_value);
            x_register_values.push(x_register_value);

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
