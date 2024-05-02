fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let mut numbers = read_input_numbers();

    mix_numbers(&mut numbers);

    let zero_position = numbers.iter().position(|n| n.value == 0).unwrap();

    let first_grove_coordinate = numbers[(zero_position + 1000) % numbers.len()].value;
    let second_grove_coordinate = numbers[(zero_position + 2000) % numbers.len()].value;
    let third_grove_coordinate = numbers[(zero_position + 3000) % numbers.len()].value;

    let sum_of_grove_coordinates =
        first_grove_coordinate + second_grove_coordinate + third_grove_coordinate;

    println!("{sum_of_grove_coordinates}");
}

fn mix_numbers(numbers: &mut Vec<Number>) {
    let numbers_count = numbers.len();
    for i in 0..numbers_count {
        let current_position = numbers
            .iter()
            .position(|n| n.initial_position == i)
            .unwrap();

        let value = numbers[current_position].value;
        if value == 0 {
            continue;
        }

        let mut from = current_position;
        let move_forward = value > 0;
        let move_iterations = value.abs();
        for _ in 0..move_iterations {
            if move_forward {
                if from == numbers_count - 1 {
                    let number = numbers.remove(from);
                    numbers.insert(0, number);

                    from = 0;
                }

                let to = from + 1;
                numbers.swap(from, to);

                from = to;
            } else {
                if from == 0 {
                    let number = numbers.remove(0);
                    numbers.push(number);

                    from = numbers_count - 1;
                }

                let to = from - 1;
                numbers.swap(from, to);

                from = to;
            }
        }
    }
}

fn read_input_numbers() -> Vec<Number> {
    let mut numbers = vec![];

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        numbers.push(Number {
            value: line.parse().unwrap(),
            initial_position: numbers.len(),
        });
    }

    numbers
}

#[derive(Clone)]
struct Number {
    value: i32,
    initial_position: usize,
}
