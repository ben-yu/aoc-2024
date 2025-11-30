use std::collections::HashMap;

advent_of_code::solution!(22);

fn next_secret(mut secret: u64) -> u64 {
    // Step 1: Multiply by 64, mix, prune
    secret ^= secret * 64;
    secret %= 16777216;

    // Step 2: Divide by 32, mix, prune
    secret ^= secret / 32;
    secret %= 16777216;

    // Step 3: Multiply by 2048, mix, prune
    secret ^= secret * 2048;
    secret %= 16777216;

    secret
}

fn generate_nth_secret(initial: u64, n: usize) -> u64 {
    let mut secret = initial;
    for _ in 0..n {
        secret = next_secret(secret);
    }
    secret
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum: u64 = input
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .map(|initial| generate_nth_secret(initial, 2000))
        .sum();

    Some(sum)
}

fn get_price(secret: u64) -> i32 {
    (secret % 10) as i32
}

fn find_best_sequence(input: &str) -> u64 {
    let buyers: Vec<u64> = input
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();

    // Map from sequence of 4 changes to total bananas
    let mut sequence_totals: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();

    for &initial in &buyers {
        let mut secret = initial;
        let mut prices = vec![get_price(secret)];

        // Generate 2000 prices
        for _ in 0..2000 {
            secret = next_secret(secret);
            prices.push(get_price(secret));
        }

        // Calculate changes
        let changes: Vec<i32> = prices.windows(2).map(|w| w[1] - w[0]).collect();

        // Track which sequences we've already seen for this buyer
        let mut seen = HashMap::new();

        // Find all 4-change sequences
        for i in 0..changes.len().saturating_sub(3) {
            let sequence = (changes[i], changes[i + 1], changes[i + 2], changes[i + 3]);

            // Only count the first occurrence of each sequence for this buyer
            if !seen.contains_key(&sequence) {
                let price = prices[i + 4]; // Price after the 4 changes
                seen.insert(sequence, price);
                *sequence_totals.entry(sequence).or_insert(0) += price;
            }
        }
    }

    // Find the sequence with the maximum total
    sequence_totals.values().max().copied().unwrap_or(0) as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(find_best_sequence(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_secret() {
        let mut secret = 123u64;
        let expected = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        for &exp in &expected {
            secret = next_secret(secret);
            assert_eq!(secret, exp);
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_some());
    }
}
