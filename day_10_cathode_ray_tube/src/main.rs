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

        let (instruction, value) = line.split_at(4);

        match instruction {
            "noop" => x_register_values.push(x_register_value),

            "addx" => {
                x_register_values.push(x_register_value);
                x_register_values.push(x_register_value);

                x_register_value += value
                    .trim()
                    .parse::<i32>()
                    .unwrap_or_else(|_| panic!("Register value must be number but is: {}", value));
            }

            _ => panic!("Unknown instruction: {}", instruction),
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
