use std::collections::HashMap;

advent_of_code::solution!(11);

fn count_digits(n: u64) -> u32 {
    // checked_ilog10 returns Some(floor(log10(n))) or None for 0.
    // unwrap_or(0) handles the case for n=0 (which has 1 digit).
    // Adding 1 gives the total number of digits.
    (n.checked_ilog10().unwrap_or(0) + 1) as u32
}

fn solve(input: &str, blinks: u32) -> Option<u64> {
    // Use a HashMap to track counts of each unique stone value
    // Key: stone value, Value: count of stones with that value
    let mut stone_counts: HashMap<u64, u64> = HashMap::new();

    // Initialize the map with input stones
    for num in input.lines().next()?.split_whitespace() {
        let stone = num.parse::<u64>().ok()?;
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    // Process each blink
    for _ in 0..blinks {
        let mut new_counts: HashMap<u64, u64> = HashMap::new();

        for (&stone, &count) in &stone_counts {
            let total_digits = count_digits(stone);

            if stone == 0 {
                // Rule 1: 0 becomes 1
                *new_counts.entry(1).or_insert(0) += count;
            } else if total_digits % 2 == 0 {
                // Rule 2: Split stone with even number of digits
                let divisor = 10_u64.pow(total_digits / 2);
                let first_half = stone / divisor;
                let second_half = stone % divisor;

                *new_counts.entry(first_half).or_insert(0) += count;
                *new_counts.entry(second_half).or_insert(0) += count;
            } else {
                // Rule 3: Multiply by 2024
                *new_counts.entry(stone * 2024).or_insert(0) += count;
            }
        }

        stone_counts = new_counts;
    }

    // Sum up all the counts
    Some(stone_counts.values().sum())
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 25)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
