use std::cmp::Ordering;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut monkeys = read_monkeys_input();

    let modulo: i64 = monkeys.iter().map(|m| m.test_divisible_by).product();

    let monkey_business_level =
        calculate_monkey_business_level(&mut monkeys, 10_000, &mut |worry_level| {
            worry_level % modulo
        });

    println!("{monkey_business_level}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut monkeys = read_monkeys_input();

    let monkey_business_level =
        calculate_monkey_business_level(&mut monkeys, 20, &mut |worry_level| worry_level / 3);

    println!("{monkey_business_level}");
}

fn calculate_monkey_business_level<F>(
    monkeys: &mut [Monkey],
    rounds: i32,
    reduce_worry_level_fn: &mut F,
) -> i64
where
    F: FnMut(i64) -> i64,
{
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            execute_monkey_turn(i, monkeys, reduce_worry_level_fn);
        }
    }

    monkeys.sort_by(|a, b| {
        if a.times_inspected_item < b.times_inspected_item {
            return Ordering::Greater;
        }

        if b.times_inspected_item < a.times_inspected_item {
            return Ordering::Less;
        }

        Ordering::Equal
    });

    let monkey_business_level = monkeys
        .iter()
        .take(2)
        .map(|m| m.times_inspected_item as i64)
        .product();

    monkey_business_level
}

fn execute_monkey_turn<F>(
    monkey_index: usize,
    monkeys: &mut [Monkey],
    reduce_worry_level_fn: &mut F,
) where
    F: FnMut(i64) -> i64,
{
    while let Some(item_worry_level) = monkeys[monkey_index].item_worry_levels.pop() {
        let operand = if monkeys[monkey_index].operation_operand == "old" {
            item_worry_level
        } else {
            monkeys[monkey_index].operation_operand.parse().unwrap()
        };

        let mut worry_level = match monkeys[monkey_index].operation_type {
            OperationType::Unknown => panic!("Unknown operation type"),
            OperationType::Add => item_worry_level + operand,
            OperationType::Multiply => item_worry_level * operand,
        };

        worry_level = reduce_worry_level_fn(worry_level);

        let monkey_index_to_throw_to = if worry_level % monkeys[monkey_index].test_divisible_by == 0
        {
            monkeys[monkey_index].monkey_to_throw_to_on_true
        } else {
            monkeys[monkey_index].monkey_to_throw_to_on_false
        };

        monkeys[monkey_index_to_throw_to]
            .item_worry_levels
            .push(worry_level);

        monkeys[monkey_index].times_inspected_item += 1;
    }
}

fn read_monkeys_input() -> Vec<Monkey> {
    let mut result = vec![];

    let mut is_reading_monkey = false;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            if is_reading_monkey {
                is_reading_monkey = false;
                continue;
            }

            break;
        }

        if line.starts_with("Monkey ") {
            is_reading_monkey = true;
            result.push(Monkey {
                item_worry_levels: vec![],
                times_inspected_item: 0,
                test_divisible_by: 0,
                operation_type: OperationType::Unknown,
                operation_operand: String::new(),
                monkey_to_throw_to_on_true: 0,
                monkey_to_throw_to_on_false: 0,
            });

            continue;
        }

        let monkey_being_read = result.last_mut().unwrap();
        if line.starts_with("Starting items:") {
            let items_start_index = line.find(':').unwrap();
            line.split_at(items_start_index + 1)
                .1
                .split(',')
                .map(|i| i.trim().parse::<i64>().unwrap())
                .for_each(|i| monkey_being_read.item_worry_levels.push(i));
        } else if line.starts_with("Operation:") {
            let operation_info_start_index = line.find("new = old ").unwrap();
            let operation_parts = line
                .split_at(operation_info_start_index + "new = old ".len())
                .1
                .split_at(1);

            match operation_parts.0 {
                "+" => monkey_being_read.operation_type = OperationType::Add,
                "*" => monkey_being_read.operation_type = OperationType::Multiply,
                _ => panic!(
                    "{}",
                    format!("Unknown operation type: {}", operation_parts.0)
                ),
            }

            monkey_being_read.operation_operand = operation_parts.1.trim().to_string();
        } else if line.starts_with("Test:") {
            monkey_being_read.test_divisible_by =
                line.split(' ').last().unwrap().parse::<i64>().unwrap();
        } else if line.starts_with("If true:") {
            monkey_being_read.monkey_to_throw_to_on_true =
                line.split(' ').last().unwrap().parse::<usize>().unwrap();
        } else if line.starts_with("If false:") {
            monkey_being_read.monkey_to_throw_to_on_false =
                line.split(' ').last().unwrap().parse::<usize>().unwrap();
        } else {
            panic!("{}", format!("Couldn't parse line: {}", line));
        }
    }

    result
}

struct Monkey {
    item_worry_levels: Vec<i64>,
    times_inspected_item: i32,
    test_divisible_by: i64,
    operation_type: OperationType,
    operation_operand: String,
    monkey_to_throw_to_on_true: usize,
    monkey_to_throw_to_on_false: usize,
}

enum OperationType {
    Unknown,
    Add,
    Multiply,
}
