use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

type Pos = (i32, i32);

fn parse_grid(input: &str) -> (Vec<Vec<char>>, Pos, Pos) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (r as i32, c as i32);
            } else if cell == 'E' {
                end = (r as i32, c as i32);
            }
        }
    }

    (grid, start, end)
}

fn bfs_distances(grid: &[Vec<char>], start: Pos) -> HashMap<Pos, u32> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    distances.insert(start, 0);

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((pos, dist)) = queue.pop_front() {
        for (dr, dc) in directions {
            let next = (pos.0 + dr, pos.1 + dc);

            if next.0 >= 0 && next.0 < grid.len() as i32
                && next.1 >= 0 && next.1 < grid[0].len() as i32 {
                let cell = grid[next.0 as usize][next.1 as usize];

                if (cell == '.' || cell == 'E') && !distances.contains_key(&next) {
                    distances.insert(next, dist + 1);
                    queue.push_back((next, dist + 1));
                }
            }
        }
    }

    distances
}

fn find_cheats(distances: &HashMap<Pos, u32>, max_cheat_length: i32, min_savings: u32) -> u32 {
    let mut count = 0;

    // For each position on the path
    for (&start_pos, &start_dist) in distances.iter() {
        // Try all positions within manhattan distance of max_cheat_length
        for dr in -max_cheat_length..=max_cheat_length {
            let remaining = max_cheat_length - dr.abs();
            for dc in -remaining..=remaining {
                let end_pos = (start_pos.0 + dr, start_pos.1 + dc);

                // Check if end position is on the path
                if let Some(&end_dist) = distances.get(&end_pos) {
                    let cheat_dist = dr.abs() + dc.abs();

                    // Calculate time saved
                    if end_dist > start_dist {
                        let normal_time = end_dist - start_dist;
                        let cheat_time = cheat_dist as u32;

                        if normal_time >= cheat_time {
                            let savings = normal_time - cheat_time;
                            if savings >= min_savings {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, start, _end) = parse_grid(input);
    let distances = bfs_distances(&grid, start);

    let count = find_cheats(&distances, 2, 100);

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, start, _end) = parse_grid(input);
    let distances = bfs_distances(&grid, start);

    let count = find_cheats(&distances, 20, 100);

    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (grid, start, _end) = parse_grid(&input);
        let distances = bfs_distances(&grid, start);

        // Test with lower threshold for the example
        let count = find_cheats(&distances, 2, 1);

        // According to the problem, there should be many cheats
        // Let's verify the structure is working
        assert_eq!(count, 44);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0)); // Placeholder
    }
}
