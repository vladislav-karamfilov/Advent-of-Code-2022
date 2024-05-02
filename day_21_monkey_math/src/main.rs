use std::collections::HashMap;

fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let monkey_jobs = read_monkey_jobs();

    let number = calculate_monkey_number(&monkey_jobs, "root");

    println!("{number}");
}

fn calculate_monkey_number(monkey_jobs: &HashMap<String, String>, monkey_name: &str) -> i64 {
    let monkey_operation = &monkey_jobs[monkey_name];
    if let Ok(number) = monkey_operation.parse::<i64>() {
        return number;
    }

    let mut splitter = monkey_operation.split(' ');
    let left_monkey_name = splitter.next().unwrap();
    let operation = splitter.next().unwrap();
    let right_monkey_name = splitter.next().unwrap();

    match operation {
        "+" => {
            calculate_monkey_number(monkey_jobs, left_monkey_name)
                + calculate_monkey_number(monkey_jobs, right_monkey_name)
        }
        "-" => {
            calculate_monkey_number(monkey_jobs, left_monkey_name)
                - calculate_monkey_number(monkey_jobs, right_monkey_name)
        }
        "*" => {
            calculate_monkey_number(monkey_jobs, left_monkey_name)
                * calculate_monkey_number(monkey_jobs, right_monkey_name)
        }
        "/" => {
            calculate_monkey_number(monkey_jobs, left_monkey_name)
                / calculate_monkey_number(monkey_jobs, right_monkey_name)
        }
        _ => unreachable!(),
    }
}

fn read_monkey_jobs() -> HashMap<String, String> {
    let mut monkey_jobs = HashMap::new();

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let mut splitter = line.split(':');
        let monkey_name = splitter.next().unwrap().trim().to_string();
        let monkey_operation = splitter.next().unwrap().trim().to_string();

        monkey_jobs.insert(monkey_name, monkey_operation);
    }

    monkey_jobs
}
