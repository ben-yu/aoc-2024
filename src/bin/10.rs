advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut total_score = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 0 {
                let score = calculate_trailhead_score(&grid, row, col);
                total_score += score;
            }
        }
    }

    Some(total_score)
}

fn calculate_trailhead_score(grid: &[Vec<u8>], start_row: usize, start_col: usize) -> u64 {
    use std::collections::HashSet;

    let rows = grid.len();
    let cols = grid[0].len();
    let mut reachable_nines = HashSet::new();
    let mut stack = vec![(start_row, start_col, 0)];

    while let Some((row, col, height)) = stack.pop() {
        if height == 9 {
            reachable_nines.insert((row, col));
            continue;
        }

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col] == height + 1 {
                    stack.push((new_row, new_col, height + 1));
                }
            }
        }
    }

    reachable_nines.len() as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut total_rating = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 0 {
                let rating = calculate_trailhead_rating(&grid, row, col);
                total_rating += rating;
            }
        }
    }

    Some(total_rating)
}

fn calculate_trailhead_rating(grid: &[Vec<u8>], start_row: usize, start_col: usize) -> u64 {
    let rows = grid.len();
    let cols = grid[0].len();

    fn count_paths(
        grid: &[Vec<u8>],
        row: usize,
        col: usize,
        height: u8,
        rows: usize,
        cols: usize,
    ) -> u64 {
        if height == 9 {
            return 1;
        }

        let mut path_count = 0;
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col] == height + 1 {
                    path_count += count_paths(grid, new_row, new_col, height + 1, rows, cols);
                }
            }
        }

        path_count
    }

    count_paths(grid, start_row, start_col, 0, rows, cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
