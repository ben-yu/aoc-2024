use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let sum = updates.iter()
        .filter(|update| is_correctly_ordered(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let sum = updates.iter()
        .filter(|update| !is_correctly_ordered(update, &rules))
        .map(|update| fix_order(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum();
    Some(sum)
}

fn parse_input(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut lines = input.lines();
    let mut rules = HashMap::new();

    // Parse ordering rules
    for line in &mut lines {
        if line.trim().is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split('|').collect();
        let before = parts[0].parse::<u32>().unwrap();
        let after = parts[1].parse::<u32>().unwrap();
        rules.entry(before).or_insert_with(HashSet::new).insert(after);
    }

    // Parse update sequences
    let updates = lines
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn is_correctly_ordered(update: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    for (i, &page) in update.iter().enumerate() {
        if let Some(must_come_after) = rules.get(&page) {
            // Check if any page that should come after this page appears before it
            for &earlier_page in &update[..i] {
                if must_come_after.contains(&earlier_page) {
                    return false;
                }
            }
        }
    }
    true
}

fn fix_order(update: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut fixed = update.to_vec();

    // Use a simple bubble sort approach with the ordering rules
    loop {
        let mut swapped = false;
        for i in 0..fixed.len() - 1 {
            let current = fixed[i];
            let next = fixed[i + 1];

            // If current page should come after next page, swap them
            if let Some(must_come_after) = rules.get(&next) {
                if must_come_after.contains(&current) {
                    fixed.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
        if !swapped {
            break;
        }
    }

    fixed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
