use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(18);

fn parse_coordinates(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

fn bfs_shortest_path(corrupted: &HashSet<(i32, i32)>, size: i32) -> Option<u32> {
    let start = (0, 0);
    let end = (size, size);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));
    visited.insert(start);

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == end {
            return Some(steps);
        }

        for (dx, dy) in directions.iter() {
            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && nx <= size && ny >= 0 && ny <= size
                && !corrupted.contains(&(nx, ny))
                && !visited.contains(&(nx, ny))
            {
                visited.insert((nx, ny));
                queue.push_back(((nx, ny), steps + 1));
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_with_params(input, 70, 1024)
}

fn part_one_with_params(input: &str, size: i32, num_bytes: usize) -> Option<u32> {
    let coords = parse_coordinates(input);
    let corrupted: HashSet<(i32, i32)> = coords.iter().take(num_bytes).copied().collect();
    bfs_shortest_path(&corrupted, size)
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_with_params(input, 70)
}

fn part_two_with_params(input: &str, size: i32) -> Option<String> {
    let coords = parse_coordinates(input);

    // Binary search for the first byte that blocks the path
    let mut left = 0;
    let mut right = coords.len();

    while left < right {
        let mid = (left + right) / 2;
        let corrupted: HashSet<(i32, i32)> = coords.iter().take(mid + 1).copied().collect();

        if bfs_shortest_path(&corrupted, size).is_none() {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    coords.get(left).map(|(x, y)| format!("{},{}", x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one_with_params(&input, 6, 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two_with_params(&input, 6);
        assert_eq!(result, Some("6,1".to_string()));
    }
}
