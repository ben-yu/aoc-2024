advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let sum: u64 = input
        .lines()
        .map(|line| parse_input(line))
        .filter(|(target, nums)| valid_combination(*target, nums))
        .map(|(target, _)| target)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum: u64 = input
        .lines()
        .map(|line| parse_input(line))
        .filter(|(target, nums)| valid_combination_with_concat(*target, nums))
        .map(|(target, _)| target)
        .sum();
    Some(sum)
}

fn parse_input(line: &str) -> (u64, Vec<u64>) {
    let parts: Vec<&str> = line.split(':').collect();
    let target = parts[0].parse::<u64>().unwrap();
    let nums = parts[1]
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    (target, nums)
}

fn valid_combination(target: u64, nums: &[u64]) -> bool {
    if nums.is_empty() {
        return false;
    }
    if nums.len() == 1 {
        return nums[0] == target;
    }

    // Try all combinations of + and * operators
    // For n numbers, we need n-1 operators
    let operator_count = nums.len() - 1;

    // Use bit manipulation: 0 = +, 1 = *
    for combo in 0..(1 << operator_count) {
        let mut result = nums[0];

        for i in 0..operator_count {
            if (combo >> i) & 1 == 0 {
                // Addition
                result += nums[i + 1];
            } else {
                // Multiplication
                result *= nums[i + 1];
            }
        }

        if result == target {
            return true;
        }
    }

    false
}

fn valid_combination_with_concat(target: u64, nums: &[u64]) -> bool {
    if nums.is_empty() {
        return false;
    }
    if nums.len() == 1 {
        return nums[0] == target;
    }

    // Try all combinations of +, *, and || operators
    // For n numbers, we need n-1 operators
    let operator_count = nums.len() - 1;

    // Use base-3 counting: 0 = +, 1 = *, 2 = ||
    for combo in 0..(3_u32.pow(operator_count as u32)) {
        let mut result = nums[0];
        let mut temp_combo = combo;

        for i in 0..operator_count {
            let op = temp_combo % 3;
            temp_combo /= 3;

            match op {
                0 => result += nums[i + 1],                             // Addition
                1 => result *= nums[i + 1],                             // Multiplication
                2 => result = concatenate_numbers(result, nums[i + 1]), // Concatenation
                _ => unreachable!(),
            }
        }

        if result == target {
            return true;
        }
    }

    false
}

fn concatenate_numbers(left: u64, right: u64) -> u64 {
    // Convert numbers to strings, concatenate, then parse back
    let concatenated = format!("{}{}", left, right);
    concatenated.parse().unwrap_or(u64::MAX) // Return MAX on overflow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
