use std::{collections::HashMap, fs::read_to_string};


fn fix_ordering(rules: HashMap<usize, Vec<usize>>, input: Vec<usize>) -> {

}

fn main() {
    let input = read_to_string("input").expect("input should be in root ");

    let lol: Vec<&str> = input.split_terminator("\n\n").collect();
    assert!(lol.len() == 2);
    let rules = lol[0];
    let updates = lol[1];

    let mut rule_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for rule in rules.lines() {
        let (key, val): (usize, usize) = (
            rule.split('|').collect::<Vec<_>>()[0]
                .parse()
                .expect("should be num"),
            rule.split('|').collect::<Vec<_>>()[1]
                .parse()
                .expect("should be num"),
        );

        match rule_map.get_mut(&key) {
            Some(rule) => {
                rule.push(val);
            }
            None => {
                rule_map.insert(key, vec![val]);
            }
        }
    }

    let mut total = 0;
    let mut totalp2 = 0;
    'updateloop: for update in updates.lines() {
        let update: Vec<usize> = update
            .split(',')
            .map(|elem| elem.parse().expect("should be num"))
            .collect();

        let mut seen = vec![];
        for elem in update.iter() {
            seen.push(elem);
            if let Some(rule) = rule_map.get(elem) {
                for rule_elem in rule {
                    if seen.contains(&rule_elem) {
                        // need to fix their ordering

                        continue 'updateloop;
                    }
                }
            }
        }

        let len = update.len();
        let mid = update.get(len / 2).expect("cmon now");
        total += mid;
    }
    println!("Total (part1): {total}");
}
