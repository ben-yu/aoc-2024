advent_of_code::solution!(6);

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Some(find_total_positions(&grid) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (start_row, start_col, _) = find_starting_position(&grid);
    let mut loop_count = 0;

    // Try placing an obstacle at each empty position
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            // Skip if position is not empty or is the starting position
            if grid[row][col] != '.' || (row == start_row && col == start_col) {
                continue;
            }

            // Place obstacle
            grid[row][col] = '#';

            // Check if this creates a loop
            if creates_loop(&grid) {
                loop_count += 1;
            }

            // Remove obstacle
            grid[row][col] = '.';
        }
    }

    Some(loop_count)
}

fn find_total_positions(grid: &[Vec<char>]) -> usize {

    let (start_row, start_col, start_dir) = find_starting_position(grid);
    let mut visited = HashSet::new();
    let mut row = start_row as i32;
    let mut col = start_col as i32;
    let mut direction = start_dir as (i32, i32);

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    loop {
        // Mark current position as visited
        visited.insert((row, col));

        // Calculate next position
        let next_row = row + direction.0;
        let next_col = col + direction.1;

        // Check if next position is out of bounds
        if next_row < 0 || next_row >= rows || next_col < 0 || next_col >= cols {
            break;
        }

        // Check if next position is an obstacle
        if grid[next_row as usize][next_col as usize] == '#' {
            // Turn right 90 degrees
            direction = match direction {
                (-1, 0) => (0, 1),  // up -> right
                (0, 1) => (1, 0),   // right -> down
                (1, 0) => (0, -1),  // down -> left
                (0, -1) => (-1, 0), // left -> up
                _ => panic!("Invalid direction"),
            };
        } else {
            // Move forward
            row = next_row;
            col = next_col;
        }
    }

    visited.len()
}

fn creates_loop(grid: &[Vec<char>]) -> bool {

    let (start_row, start_col, start_dir) = find_starting_position(grid);
    let mut visited_states = HashSet::new();
    let mut row = start_row as i32;
    let mut col = start_col as i32;
    let mut direction = start_dir;

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    loop {
        // Check if we've seen this state (position + direction) before
        let state = (row, col, direction);
        if visited_states.contains(&state) {
            return true; // Found a loop
        }
        visited_states.insert(state);

        // Calculate next position
        let next_row = row + direction.0;
        let next_col = col + direction.1;

        // Check if next position is out of bounds
        if next_row < 0 || next_row >= rows || next_col < 0 || next_col >= cols {
            return false; // Guard exits, no loop
        }

        // Check if next position is an obstacle
        if grid[next_row as usize][next_col as usize] == '#' {
            // Turn right 90 degrees
            direction = match direction {
                (-1, 0) => (0, 1),  // up -> right
                (0, 1) => (1, 0),   // right -> down
                (1, 0) => (0, -1),  // down -> left
                (0, -1) => (-1, 0), // left -> up
                _ => panic!("Invalid direction"),
            };
        } else {
            // Move forward
            row = next_row;
            col = next_col;
        }
    }
}

fn find_starting_position(grid: &[Vec<char>]) -> (usize, usize, (i32, i32)) {
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if matches!(ch, '^' | 'v' | '<' | '>') {
                let direction = match ch {
                    '^' => (-1, 0),
                    'v' => (1, 0),
                    '<' => (0, -1),
                    '>' => (0, 1),
                    _ => panic!("Invalid starting character"),
                };

                return (row, col, direction);
            }
        }
    }
    panic!("No starting position found");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
