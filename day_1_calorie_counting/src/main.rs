fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut elf_calories = Vec::new();
    let mut current_elf_calories = 0;

    let mut is_reading_elf_calories = false;
    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            if !is_reading_elf_calories {
                break;
            }

            elf_calories.push(current_elf_calories);

            is_reading_elf_calories = false;
            current_elf_calories = 0;
            continue;
        }

        let calories: i32 = match trimmed_line.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Elf calories number expected");
                return;
            }
        };

        is_reading_elf_calories = true;
        current_elf_calories += calories;
    }

    elf_calories.sort();

    let sum_of_max_elf_calories: i32 = elf_calories.iter().rev().take(3).sum();

    println!("{sum_of_max_elf_calories}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut max_elf_calories = std::i32::MIN;
    let mut current_elf_calories = 0;

    let mut is_reading_elf_calories = false;
    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            if !is_reading_elf_calories {
                break;
            }

            if current_elf_calories > max_elf_calories {
                max_elf_calories = current_elf_calories;
            }

            is_reading_elf_calories = false;
            current_elf_calories = 0;
            continue;
        }

        let calories: i32 = match trimmed_line.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Elf calories number expected");
                return;
            }
        };

        is_reading_elf_calories = true;
        current_elf_calories += calories;
    }

    println!("{max_elf_calories}");
}
