fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let snafu_numbers = read_snafu_numbers();

    let decimal_sum: i64 = snafu_numbers
        .iter()
        .map(|n| convert_to_decimal_number(n))
        .sum();

    println!("{decimal_sum}");

    let snafu_sum = convert_to_snafu_number(decimal_sum);

    println!("{snafu_sum}");
}

fn convert_to_decimal_number(snafu_number: &str) -> i64 {
    let mut decimal_number = 0;

    for (i, ch) in snafu_number.chars().enumerate() {
        let digit = match ch {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        };

        let power = snafu_number.len() - i - 1;

        decimal_number += digit * 5i64.pow(power as u32);
    }

    decimal_number
}

fn convert_to_snafu_number(decimal_number: i64) -> String {
    if decimal_number == 0 {
        return "0".to_string();
    }

    let mut snafu_digits = vec![];

    let mut current_decimal_number = decimal_number;
    while current_decimal_number != 0 {
        let digit = match current_decimal_number % 5 {
            4 => "-",
            3 => "=",
            2 => "2",
            1 => "1",
            0 => "0",
            _ => unreachable!(),
        };

        snafu_digits.push(digit);

        current_decimal_number = (current_decimal_number + 2) / 5;
    }

    snafu_digits.reverse();

    snafu_digits.join("")
}

fn read_snafu_numbers() -> Vec<String> {
    let mut snafu_numbers = vec![];

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        snafu_numbers.push(line.to_string());
    }

    snafu_numbers
}
