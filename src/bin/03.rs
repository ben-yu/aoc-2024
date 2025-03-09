advent_of_code::solution!(3);

use regex::Regex;

fn parse_mul(input: &str) -> Vec<i32> {
    // Create a regex pattern
    let pattern = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();

    // Try to find a match in the input string
    pattern
        .captures_iter(input)
        .filter_map(|captures| {
            // Parse the captured numbers into i32
            let x = captures[1].parse::<i32>().unwrap();
            let y = captures[2].parse::<i32>().unwrap();

            Some(x * y)
        })
        .collect()
}

fn parse_cond_mul(input: &str) -> Vec<i32> {
    // Create a regex pattern
    let pattern = Regex::new(r"(mul\((-?\d+),(-?\d+)\)|do\(\)|don't\(\))").unwrap();

    let mut mul_enabled = true;

    // Try to find a match in the input string
    pattern
        .captures_iter(input)
        .filter_map(|captures| {
            let full_match = &captures[1];

            // Determine which pattern matched and create appropriate Operation
            match full_match {
                s if s.starts_with("mul") => {
                    // For multiplication, we need to parse the numbers
                    // caps[2] and caps[3] contain the numbers for multiplication
                    if mul_enabled {
                        let x = captures[2].parse::<i32>().ok()?;
                        let y = captures[3].parse::<i32>().ok()?;
                        return Some(x * y);
                    }
                    return None;
                }
                "do()" => {
                    mul_enabled = true;
                    None
                }
                "don't()" => {
                    mul_enabled = false;
                    None
                }
                _ => None,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(parse_mul(input).iter().sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(parse_cond_mul(input).iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
