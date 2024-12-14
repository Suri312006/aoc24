use std::fs::read_to_string;

#[derive(PartialEq)]
enum State {
    Do,
    Dont,
}

fn char_to_num(num: &[char]) -> Option<u32> {
    let num = num.to_vec();
    match num.len() {
        1 => num[0].to_digit(10),
        2 => {
            let first = num[0].to_digit(10)?;
            let second = num[1].to_digit(10)?;
            Some(first * 10 + second)
        }
        3 => {
            let first = num[0].to_digit(10)?;
            let second = num[1].to_digit(10)?;
            let third = num[2].to_digit(10)?;
            Some(first * 100 + second * 10 + third)
        }
        _ => None,
    }
}

fn parse_state(buff: &[char]) -> Option<State> {
    if buff[0..4] == ['d', 'o', '(', ')'] {
        return Some(State::Do);
    };

    if buff == ['d', 'o', 'n', '\'', 't', '(', ')'] {
        return Some(State::Dont);
    };

    None
}

fn main() {
    let input = read_to_string("input").expect("input should be in root");
    let mut chars = vec![];
    for char in input.chars() {
        chars.push(char);
    }

    let mut result1 = 0;
    let mut result2 = 0;

    let mut state = State::Do;

    for window in chars.windows(12) {
        if let Some(new) = parse_state(&window[0..7]) {
            state = new;
        }

        if window[0..4] == ['m', 'u', 'l', '('] {
            let candidate = window[4..12].to_vec();
            let end = candidate.iter().enumerate().find(|(_, char)| **char == ')');

            if end.is_none() {
                continue;
            }

            let (idx, _) = end.unwrap();

            let (useful, _) = candidate.split_at(idx);

            let comma = useful.iter().enumerate().find(|(_, char)| **char == ',');
            if comma.is_none() {
                continue;
            }

            let (idx, _) = comma.unwrap();

            let (num1, mut num2) = useful.split_at(idx);
            let mut x = num2.to_vec();
            x.remove(0);

            num2 = x.as_slice();

            let num1 = match char_to_num(num1) {
                Some(num) => num,
                None => continue,
            };

            let num2 = match char_to_num(num2) {
                Some(num) => num,
                None => continue,
            };

            result1 += num1 * num2;
            if state == State::Do {
                result2 += num1 * num2;
            }
        }
    }
    println!("Part1 Result : {result1}");

    println!("Part2 Result : {result2}");
}
