use std::collections::HashMap;

advent_of_code::solution!(21);

type Pos = (i32, i32);

fn get_numeric_keypad_pos(key: char) -> Pos {
    match key {
        '7' => (0, 0), '8' => (0, 1), '9' => (0, 2),
        '4' => (1, 0), '5' => (1, 1), '6' => (1, 2),
        '1' => (2, 0), '2' => (2, 1), '3' => (2, 2),
        '0' => (3, 1), 'A' => (3, 2),
        _ => panic!("Invalid numeric key: {}", key),
    }
}

fn get_directional_keypad_pos(key: char) -> Pos {
    match key {
        '^' => (0, 1), 'A' => (0, 2),
        '<' => (1, 0), 'v' => (1, 1), '>' => (1, 2),
        _ => panic!("Invalid directional key: {}", key),
    }
}

fn get_path_moves(from: Pos, to: Pos, gap: Pos) -> Vec<String> {
    let dr = to.0 - from.0;
    let dc = to.1 - from.1;

    let mut vertical = String::new();
    let mut horizontal = String::new();

    for _ in 0..dr.abs() {
        vertical.push(if dr > 0 { 'v' } else { '^' });
    }

    for _ in 0..dc.abs() {
        horizontal.push(if dc > 0 { '>' } else { '<' });
    }

    let mut paths = Vec::new();

    // Try horizontal then vertical
    if (from.0, to.1) != gap {
        let mut path = horizontal.clone();
        path.push_str(&vertical);
        path.push('A');
        paths.push(path);
    }

    // Try vertical then horizontal
    if (to.0, from.1) != gap {
        let mut path = vertical.clone();
        path.push_str(&horizontal);
        path.push('A');
        paths.push(path);
    }

    // Remove duplicates (when moving in only one direction)
    paths.sort();
    paths.dedup();

    paths
}

fn get_numeric_paths(from: char, to: char) -> Vec<String> {
    let from_pos = get_numeric_keypad_pos(from);
    let to_pos = get_numeric_keypad_pos(to);
    let gap = (3, 0);
    get_path_moves(from_pos, to_pos, gap)
}

fn get_directional_paths(from: char, to: char) -> Vec<String> {
    let from_pos = get_directional_keypad_pos(from);
    let to_pos = get_directional_keypad_pos(to);
    let gap = (0, 0);
    get_path_moves(from_pos, to_pos, gap)
}

fn min_length_for_sequence(
    sequence: &str,
    depth: usize,
    is_numeric: bool,
    memo: &mut HashMap<(String, usize, bool), u64>,
) -> u64 {
    if depth == 0 {
        return sequence.len() as u64;
    }

    let key = (sequence.to_string(), depth, is_numeric);
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let mut current = 'A';
    let mut total_length = 0;

    for target in sequence.chars() {
        let paths = if is_numeric {
            get_numeric_paths(current, target)
        } else {
            get_directional_paths(current, target)
        };

        let min_len = paths
            .iter()
            .map(|path| min_length_for_sequence(path, depth - 1, false, memo))
            .min()
            .unwrap();

        total_length += min_len;
        current = target;
    }

    memo.insert(key, total_length);
    total_length
}

fn solve(input: &str, num_directional_robots: usize) -> u64 {
    let mut memo = HashMap::new();
    let mut total_complexity = 0;

    for line in input.lines() {
        let code = line.trim();
        if code.is_empty() {
            continue;
        }

        let length = min_length_for_sequence(code, num_directional_robots + 1, true, &mut memo);

        let numeric_part: u64 = code
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(0);

        let complexity = length * numeric_part;
        total_complexity += complexity;
    }

    total_complexity
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 25))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_some());
    }
}
