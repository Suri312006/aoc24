use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

const BLINKS: usize = 75;

fn main() {
    let input = read_to_string("input").expect("file should be here");
    // let input = read_to_string("test").expect("file should be here");
    let mut all_stones: Vec<HashMap<u128, u128>> = vec![HashMap::new()];
    input.split_whitespace().for_each(|e| {
        all_stones[0].insert(e.to_string().parse().expect("should be a num"), 1);
    });

    for blink_i in 0..BLINKS + 1 {
        let stones = all_stones.pop().unwrap();

        println!("blink count: {blink_i}");
        let total: u128 = stones.iter().map(|(_, occur)| *occur).sum();
        println!("len: {:?}\n", total);
        // println!("{:?}\n", stones);

        let mut next = HashMap::new();
        for (stone, occurrences) in stones.into_iter() {
            let string_stone = stone.to_string();
            if stone == 0 {
                // next.insert(1, occurrences);
                if let Some(val) = next.get(&(1)) {
                    next.insert(1, occurrences + val);
                } else {
                    next.insert(1, occurrences);
                }
            } else if string_stone.len() % 2 == 0 {
                let (lhs, rhs) = string_stone.split_at(string_stone.len() / 2);
                let left = lhs.parse().expect("should be valid num");
                let right = rhs.parse().expect("should be valid num");

                // time to insert these guys
                if let Some(val) = next.get(&(left)) {
                    next.insert(left, occurrences + val);
                } else {
                    next.insert(left, occurrences);
                }

                if let Some(val) = next.get(&(right)) {
                    next.insert(right, occurrences + val);
                } else {
                    next.insert(right, occurrences);
                }
            } else {
                if let Some(val) = next.get(&(stone * 2024)) {
                    next.insert(stone * 2024, occurrences + val);
                } else {
                    next.insert(stone * 2024, occurrences);
                }
            }
        }

        all_stones.push(next);
    }
}
