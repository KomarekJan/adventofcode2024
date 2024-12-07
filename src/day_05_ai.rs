use std::collections::{HashSet, HashMap, VecDeque};
use std::fs;

// Function to parse rules
fn parse_rules(rules_content: &str) -> Vec<(usize, usize)> {
    rules_content
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

// Function to parse updates
fn parse_updates(updates_content: &str) -> Vec<Vec<usize>> {
    updates_content
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

// Function to check if a single update is correctly ordered
fn is_update_correct(update: &[usize], rules: &[(usize, usize)]) -> bool {
    let update_set: HashSet<_> = update.iter().cloned().collect();
    for &(before, after) in rules {
        if update_set.contains(&before) && update_set.contains(&after) {
            let pos_before = update.iter().position(|&x| x == before).unwrap();
            let pos_after = update.iter().position(|&x| x == after).unwrap();
            if pos_before > pos_after {
                return false;
            }
        }
    }
    true
}

// Topological sort to order an update correctly according to the rules
fn topological_sort(update: &[usize], rules: &[(usize, usize)]) -> Vec<usize> {
    // Build a graph and in-degree count
    let update_set: HashSet<_> = update.iter().cloned().collect();
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree: HashMap<usize, usize> = HashMap::new();

    // Initialize the graph and in-degree map
    for &page in update {
        graph.entry(page).or_default();
        in_degree.entry(page).or_insert(0);
    }

    // Build the graph based on the rules
    for &(before, after) in rules {
        if update_set.contains(&before) && update_set.contains(&after) {
            graph.get_mut(&before).unwrap().push(after);
            *in_degree.get_mut(&after).unwrap() += 1;
        }
    }

    // Perform topological sort using Kahn's algorithm
    let mut sorted: Vec<usize> = Vec::new();
    let mut zero_in_degree: VecDeque<usize> = in_degree
        .iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&k, _)| k)
        .collect();

    while let Some(node) = zero_in_degree.pop_front() {
        sorted.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(count) = in_degree.get_mut(&neighbor) {
                    *count -= 1;
                    if *count == 0 {
                        zero_in_degree.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted
}

// Function to calculate the middle page number
fn middle_page(update: &[usize]) -> usize {
    update[update.len() / 2]
}

#[allow(unused)]
pub(crate) fn main() {
    // Read and parse files
    let rules_content = fs::read_to_string("data/day_05_rules.txt").expect("Failed to read rules file");
    let updates_content = fs::read_to_string("data/day_05_updates.txt").expect("Failed to read updates file");

    let rules = parse_rules(&rules_content);
    let updates = parse_updates(&updates_content);

    let mut middle_pages_sum = 0;

    // Process each update
    for update in updates {
        if !is_update_correct(&update, &rules) {
            let reordered_update = topological_sort(&update, &rules);
            middle_pages_sum += middle_page(&reordered_update);
        }
    }

    println!("Sum of middle page numbers for reordered updates: {}", middle_pages_sum);
}