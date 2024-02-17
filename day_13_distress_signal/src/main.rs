use std::{
    cmp::{self, Ordering},
    collections::VecDeque,
};

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let first_divider_packet = PacketValue::List(vec![PacketValue::Integer(2)]);
    let second_diviver_packet = PacketValue::List(vec![PacketValue::Integer(6)]);

    let mut packet_values = vec![first_divider_packet.clone(), second_diviver_packet.clone()];

    let mut is_last_line_empty = false;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            match is_last_line_empty {
                true => break,
                false => {
                    is_last_line_empty = true;
                    continue;
                }
            }
        }

        packet_values.push(parse_packet_value(&line[1..line.len() - 1]));
        is_last_line_empty = false;
    }

    packet_values.sort();

    let first_divider_index = packet_values
        .iter()
        .position(|v| *v == first_divider_packet)
        .unwrap();

    let second_divider_index = packet_values
        .iter()
        .position(|v| *v == second_diviver_packet)
        .unwrap();

    let decoder_key = (first_divider_index + 1) * (second_divider_index + 1);

    println!("{decoder_key}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut left: Option<PacketValue> = None;
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
                if left_packet_value.cmp(&right_packet_value) == Ordering::Less {
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

    for (i, ch) in str.chars().enumerate() {
        if ch == '[' {
            list_start_indices.push_back(i);
        } else if ch == ']' {
            let start_index = list_start_indices.pop_back().unwrap();
            if list_start_indices.is_empty() {
                list.push(parse_packet_value(&str[start_index + 1..i]));
            }
        } else if list_start_indices.is_empty() {
            if ch.is_ascii_digit() && integer_start_index == -1 {
                integer_start_index = i as i32;
            } else if ch == ',' && integer_start_index > -1 {
                if let Ok(integer) = str[integer_start_index as usize..i].parse() {
                    list.push(PacketValue::Integer(integer));
                }

                integer_start_index = -1;
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

#[derive(PartialEq, Eq, Clone)]
enum PacketValue {
    Integer(i32),
    List(Vec<PacketValue>),
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            PacketValue::Integer(left_integer) => match other {
                PacketValue::Integer(right_integer) => left_integer.cmp(right_integer),
                PacketValue::List(_) => {
                    PacketValue::List(vec![PacketValue::Integer(*left_integer)]).cmp(other)
                }
            },
            PacketValue::List(left_list) => match other {
                PacketValue::Integer(right_integer) => {
                    self.cmp(&PacketValue::List(vec![PacketValue::Integer(
                        *right_integer,
                    )]))
                }

                PacketValue::List(right_list) => {
                    let length = cmp::min(left_list.len(), right_list.len());
                    for i in 0..length {
                        let compare_result = left_list[i].cmp(&right_list[i]);
                        if compare_result != Ordering::Equal {
                            return compare_result;
                        }
                    }

                    left_list.len().cmp(&right_list.len())
                }
            },
        }
    }
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
