use std::{
    cmp::{self, Ordering},
    collections::VecDeque,
};

fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
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
                let right_packet_value = parse_packet_value(&line[1..line.len() - 1]);
                if compare_packet_values(&left_packet_value, &right_packet_value) == Ordering::Less
                {
                    //println!("{pair_index}");

                    sum_of_right_order_indices += pair_index;
                }

                pair_index += 1;

                left = None;
            }
            None => {
                left = Some(parse_packet_value(&line[1..line.len() - 1]));

                is_reading_pair = true;
            }
        }
    }

    println!("{sum_of_right_order_indices}");
}

fn parse_packet_value(str: &str) -> PacketValue {
    if str.is_empty() {
        return PacketValue::List(vec![]);
    }

    if !str.contains('[') {
        return PacketValue::List(
            str.split(',')
                .map(|substr| PacketValue::Integer(substr.parse::<i32>().unwrap()))
                .collect(),
        );
    }

    let mut list = vec![];

    let mut list_start_indices = VecDeque::new();
    let mut integer_start_index = -1;
    let mut is_in_list = false;

    for (i, ch) in str.chars().enumerate() {
        if ch == '[' {
            list_start_indices.push_back(i);
            is_in_list = true;
        } else if ch == ']' {
            let start_index = list_start_indices.pop_back().unwrap();
            list.push(parse_packet_value(&str[start_index + 1..i]));
            is_in_list = false;
        } else if !is_in_list {
            if ch == ',' && integer_start_index > -1 {
                if let Ok(integer) = str[integer_start_index as usize..i].parse() {
                    list.push(PacketValue::Integer(integer));
                }

                integer_start_index = -1;
            } else if ch.is_ascii_digit() && integer_start_index == -1 {
                integer_start_index = i as i32;
            }
        }
    }

    if integer_start_index > -1 {
        if let Ok(integer) = str[integer_start_index as usize..].parse() {
            list.push(PacketValue::Integer(integer));
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
                let length = cmp::min(left_list.len(), right_list.len());
                for i in 0..length {
                    let compare_result = compare_packet_values(&left_list[i], &right_list[i]);
                    if compare_result != Ordering::Equal {
                        return compare_result;
                    }
                }

                left_list.len().cmp(&right_list.len())
            }
        },
    }
}

enum PacketValue {
    Integer(i32),
    List(Vec<PacketValue>),
}
