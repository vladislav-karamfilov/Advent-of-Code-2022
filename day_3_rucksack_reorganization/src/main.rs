fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

fn solve_puzzle2() {
    let mut badge_item_type_priorities_sum = 0;
    let mut group_rucksack_contents = Vec::new();

    loop {
        let mut rucksack_contents = String::new();

        std::io::stdin()
            .read_line(&mut rucksack_contents)
            .expect("Failed to read line");

        let rucksack_contents = rucksack_contents.trim().to_string();
        if rucksack_contents.is_empty() {
            break;
        }

        group_rucksack_contents.push(rucksack_contents);

        if group_rucksack_contents.len() == 3 {
            let badge_item_type = find_badge_item_type(&group_rucksack_contents);

            let badge_item_type_priority = calculate_item_type_priority(badge_item_type);

            badge_item_type_priorities_sum += badge_item_type_priority;

            group_rucksack_contents.clear();
        }
    }

    println!("{badge_item_type_priorities_sum}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut item_type_priorities_sum = 0;

    loop {
        let mut rucksack_contents = String::new();

        std::io::stdin()
            .read_line(&mut rucksack_contents)
            .expect("Failed to read line");

        let rucksack_contents = rucksack_contents.trim();
        if rucksack_contents.is_empty() {
            break;
        }

        let misplaced_item_type = find_misplaced_item_type(rucksack_contents);

        let misplaced_item_type_priority = calculate_item_type_priority(misplaced_item_type);

        item_type_priorities_sum += misplaced_item_type_priority;
    }

    println!("{item_type_priorities_sum}");
}

fn find_badge_item_type(group_rucksack_contents: &[String]) -> char {
    for item_type in group_rucksack_contents[0].chars() {
        if group_rucksack_contents[1].contains(item_type)
            && group_rucksack_contents[2].contains(item_type)
        {
            return item_type;
        }
    }

    unreachable!();
}

fn calculate_item_type_priority(item_type: char) -> i32 {
    let mut item_type_priority = (item_type as i32) - ('A' as i32);
    if item_type_priority > 25 {
        item_type_priority = (item_type as i32) - ('a' as i32) + 1;
    } else {
        item_type_priority += 27;
    }

    item_type_priority
}

fn find_misplaced_item_type(rucksack_contents: &str) -> char {
    let rucksack_contents_chars = rucksack_contents.as_bytes();

    for i in 0..(rucksack_contents.len() / 2) {
        for j in (rucksack_contents.len() / 2)..rucksack_contents.len() {
            if rucksack_contents_chars[i] == rucksack_contents_chars[j] {
                return rucksack_contents_chars[i] as char;
            }
        }
    }

    unreachable!()
}
