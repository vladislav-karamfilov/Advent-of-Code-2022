use std::cmp::Ordering;

fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    parse_packet_value("[[1],[2,3,4]]");

    return;

    let mut left = None;
    let mut is_reading_pair = true;

    let mut pair_index = 1;
    let mut sum_of_right_order_indices = 0;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            match is_reading_pair {
                true => {
                    is_reading_pair = false;
                    continue;
                }
                false => break,
            }
        }

        match left {
            Some(left_packet_value) => {
                let right_packet_value = parse_packet_value(line);
                if compare_packet_values(&left_packet_value, &right_packet_value) == Ordering::Less
                {
                    sum_of_right_order_indices += pair_index;
                }

                pair_index += 1;

                left = None;
            }
            None => {
                left = Some(parse_packet_value(line));

                is_reading_pair = true;
            }
        }
    }

    println!("{sum_of_right_order_indices}");
}

fn parse_packet_value(str: &str) -> PacketValue {
    if let Ok(integer) = str.parse() {
        return PacketValue::Integer(integer);
    }

    let mut list = vec![];

    let mut start_index = 0;
    for (i, ch) in str.chars().enumerate() {
        if ch == '[' {
            start_index = i;
        } else if ch == ']' {
            list.push(parse_packet_value(&str[start_index..i + 1]));
        } else if ch == ',' {
            list.push(PacketValue::Integer(str[start_index..i].parse().unwrap()));
            start_index = 0;
        } else if start_index == 0 {
            start_index = i;
        }
    }

    PacketValue::List(list)
}

fn compare_packet_values(left: &PacketValue, right: &PacketValue) -> Ordering {
    match left {
        PacketValue::Integer(left_integer) => match right {
            PacketValue::Integer(right_integer) => left_integer.cmp(right_integer),
            PacketValue::List(_) => compare_packet_values(
                &PacketValue::List(vec![PacketValue::Integer(*left_integer)]),
                right,
            ),
        },
        PacketValue::List(left_list) => match right {
            PacketValue::Integer(right_integer) => compare_packet_values(
                left,
                &PacketValue::List(vec![PacketValue::Integer(*right_integer)]),
            ),

            PacketValue::List(right_list) => {
                if right_list.len() < left_list.len() {
                    return Ordering::Greater;
                }

                for i in 0..left_list.len() {
                    if compare_packet_values(&left_list[i], &right_list[i]) == Ordering::Greater {
                        return Ordering::Greater;
                    }
                }

                if right_list.len() == left_list.len() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
        },
    }
}

enum PacketValue {
    Integer(i32),
    List(Vec<PacketValue>),
}
