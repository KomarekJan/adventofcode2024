use std::collections::HashMap;
fn load_rules() -> HashMap<u32, Vec<u32>> {
    let mut rules: Vec<(u32, u32)> = include_str!("../data/day_05_rules.txt")
        .split_whitespace()
        .map(|x| {
            let mut parts = x.split("|");
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    rules.sort_by_key(|x| x.0);
    let mut rules_hash: HashMap<u32, Vec<u32>> = HashMap::new();

    for rule in rules {
        rules_hash
            .entry(rule.0)
            .or_insert_with(Vec::new)
            .push(rule.1);
    }

    rules_hash
}

fn load_updates() -> Vec<Vec<u32>> {
    include_str!("../data/day_05_updates.txt")
        .split("\n")
        .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn check_update(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
    for (value_pos, value) in update.iter().enumerate() {
        if let Some(rule) = rules.get(&value) {
            for i in rule {
                let rule_pos = update.iter().position(|x| *x == *i);
                if let Some(rule_pos) = rule_pos {
                    if value_pos > rule_pos {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn get_middle(update: &Vec<u32>) -> u32 {
    update[update.len() / 2]
}

fn find_update_fix(
    rules: &HashMap<u32, Vec<u32>>,
    update: &mut Vec<u32>,
) -> Option<(usize, usize)> {
    for (value_pos, value) in update.iter().enumerate() {
        if let Some(rule) = rules.get(&value) {
            for i in rule {
                if let Some(rule_pos) = update.iter().position(|x| *x == *i) {
                    if value_pos > rule_pos {
                        return Some((value_pos, rule_pos));
                    }
                }
            }
        }
    }
    None
}

fn fix_invalid_update(rules: &HashMap<u32, Vec<u32>>, update: &mut Vec<u32>) {
    while let Some(fix_pos) = find_update_fix(rules, update) {
        update.swap(fix_pos.0, fix_pos.1);
    }
}

#[allow(unused)]
pub fn solution() {
    let rules = load_rules();
    let updates = load_updates();
    let valid_updates: Vec<Vec<u32>> = updates
        .clone()
        .into_iter()
        .filter(|x| check_update(&rules, x))
        .collect();
    let mut invalid_updates: Vec<Vec<u32>> = updates
        .clone()
        .into_iter()
        .filter(|x| !check_update(&rules, x))
        .collect();

    assert_eq!(updates.len(), valid_updates.len() + invalid_updates.len());

    let mut invalid_updates: Vec<Vec<u32>> = invalid_updates
        .into_iter()
        .map(|mut x| {
            fix_invalid_update(&rules, &mut x);
            x
        })
        .collect();

    let result = invalid_updates.iter().fold(0, |acc, x| acc + get_middle(x));

    println!("Invalid updates: {}", result);

    let result = valid_updates.iter().fold(0, |acc, x| acc + get_middle(x));

    println!("Valid updates: {}", result);
}
