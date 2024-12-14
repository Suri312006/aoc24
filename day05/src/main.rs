use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

enum RuleError {
    Failed,
}

fn rule_check(rules: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> Result<(), RuleError> {
    let mut seen = vec![];
    for elem in update.iter() {
        seen.push(elem);
        if let Some(rule) = rules.get(elem) {
            for rule_elem in rule {
                if seen.contains(&rule_elem) {
                    // need to fix their ordering
                    return Err(RuleError::Failed);
                }
            }
        }
    }
    Ok(())
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
    let mut erroneous_updates = vec![];

    for update in updates.lines() {
        let update: Vec<usize> = update
            .split(',')
            .map(|elem| elem.parse().expect("should be num"))
            .collect();
        match rule_check(&rule_map, &update) {
            Ok(_) => {
                let len = update.len();
                let mid = update.get(len / 2).expect("cmon now");
                total += mid;
            }
            Err(_) => {
                // push to an errors one
                erroneous_updates.push(update);
            }
        }
    }

    let mut totalp2 = 0;
    for mut update in erroneous_updates {
        // we want to organize it properly
        //NOTE: this is so cool please show adrien
        update.sort_by(|a, b| {
            if let Some(rule) = rule_map.get(a) {
                for rule_elem in rule {
                    if rule_elem == b {
                        return Ordering::Less;
                    }
                }
            }
            Ordering::Greater
        });

        match rule_check(&rule_map, &update) {
            Ok(_) => {
                let len = update.len();
                let mid = update.get(len / 2).expect("cmon now");
                totalp2 += mid;
            }
            Err(_) => {
                panic!("shouldnt have errored lol");
            }
        }
    }
    println!("Total (part1): {total}");
    println!("Total (part2): {totalp2}");
}
