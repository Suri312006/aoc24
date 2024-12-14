use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("wrong path pal");

    let mut list1: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];
    for line in input.lines() {
        let parts = line.split_whitespace();
        for (idx, part) in parts.into_iter().enumerate() {
            match idx {
                0 => {
                    list1.push(part.parse().expect("should be a num"));
                }
                1 => {
                    list2.push(part.parse().expect("should be a num"));
                }
                _ => {}
            }
        }
    }

    list1.sort();
    list2.sort();

    let mut dist = vec![];

    for (item1, item2) in list1.iter().zip(list2.iter()) {
        dist.push(item1.abs_diff(*item2))
    }

    let total_dist: usize = dist.into_iter().sum();

    println!("Total Distance (Part 1): {total_dist}");

    // part 2
    let mut sim_score = 0;
    for item1 in list1 {
        let mut inc_factor = 0;
        for item2 in &list2 {
            if *item2 == item1 {
                inc_factor += 1;
            }
        }

        sim_score += item1 * inc_factor;
    }
    println!("Total Similarity Score (Part 2): {sim_score}");
}
