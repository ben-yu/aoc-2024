advent_of_code::solution!(15);

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let grid: Vec<Vec<char>> = parts[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let movements: Vec<char> = parts[1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    (grid, movements)
}

fn find_robot(grid: &[Vec<char>]) -> (usize, usize) {
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == '@' {
                return (row, col);
            }
        }
    }
    panic!("Robot not found");
}

fn direction_delta(dir: char) -> (i32, i32) {
    match dir {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => (0, 0),
    }
}

fn simulate_move(grid: &mut Vec<Vec<char>>, robot: &mut (usize, usize), dir: char) {
    let (dr, dc) = direction_delta(dir);
    let (robot_row, robot_col) = *robot;

    let new_row = (robot_row as i32 + dr) as usize;
    let new_col = (robot_col as i32 + dc) as usize;

    match grid[new_row][new_col] {
        '.' => {
            // Empty space, just move
            grid[robot_row][robot_col] = '.';
            grid[new_row][new_col] = '@';
            *robot = (new_row, new_col);
        }
        'O' => {
            // Box - try to push it
            // Find the end of the chain of boxes
            let mut check_row = new_row;
            let mut check_col = new_col;

            while grid[check_row][check_col] == 'O' {
                check_row = (check_row as i32 + dr) as usize;
                check_col = (check_col as i32 + dc) as usize;
            }

            // If there's empty space after boxes, push them
            if grid[check_row][check_col] == '.' {
                // Move the last box to the empty space
                grid[check_row][check_col] = 'O';
                // Robot moves into first box's position
                grid[new_row][new_col] = '@';
                // Clear robot's old position
                grid[robot_row][robot_col] = '.';
                *robot = (new_row, new_col);
            }
            // If it's a wall, don't move anything
        }
        '#' => {
            // Wall, don't move
        }
        _ => {}
    }
}

fn calculate_gps_sum(grid: &[Vec<char>]) -> u64 {
    let mut sum = 0u64;
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'O' {
                sum += (100 * row + col) as u64;
            }
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut grid, movements) = parse_input(input);
    let mut robot = find_robot(&grid);

    for movement in movements {
        simulate_move(&mut grid, &mut robot, movement);
    }

    Some(calculate_gps_sum(&grid))
}

fn widen_grid(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    grid.iter()
        .map(|row| {
            row.iter()
                .flat_map(|&ch| match ch {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    _ => vec![ch, ch],
                })
                .collect()
        })
        .collect()
}

fn can_push_wide_box(
    grid: &[Vec<char>],
    box_row: usize,
    box_col: usize,
    dr: i32,
    dc: i32,
) -> bool {
    // box_col is the position of '[', ']' is at box_col + 1
    if dc != 0 {
        // Horizontal push - simpler, just check in line
        let check_col = if dc > 0 {
            (box_col as i32 + 2 * dc) as usize // Check after ']'
        } else {
            (box_col as i32 + dc) as usize // Check before '['
        };

        match grid[box_row][check_col] {
            '.' => true,
            '#' => false,
            '[' => can_push_wide_box(grid, box_row, check_col, dr, dc),
            ']' => can_push_wide_box(grid, box_row, check_col - 1, dr, dc),
            _ => false,
        }
    } else {
        // Vertical push - need to check both columns
        let new_row = (box_row as i32 + dr) as usize;
        let left_col = box_col;
        let right_col = box_col + 1;

        let left_char = grid[new_row][left_col];
        let right_char = grid[new_row][right_col];

        // Both positions must be pushable
        let left_ok = match left_char {
            '.' => true,
            '#' => false,
            '[' => can_push_wide_box(grid, new_row, left_col, dr, dc),
            ']' => can_push_wide_box(grid, new_row, left_col - 1, dr, dc),
            _ => false,
        };

        if !left_ok {
            return false;
        }

        let right_ok = match right_char {
            '.' => true,
            '#' => false,
            '[' => can_push_wide_box(grid, new_row, right_col, dr, dc),
            ']' => true, // Already checked via left_char if it's the same box
            _ => false,
        };

        left_ok && right_ok
    }
}

fn push_wide_box(grid: &mut Vec<Vec<char>>, box_row: usize, box_col: usize, dr: i32, dc: i32) {
    if dc != 0 {
        // Horizontal push
        let check_col = if dc > 0 {
            (box_col as i32 + 2 * dc) as usize
        } else {
            (box_col as i32 + dc) as usize
        };

        match grid[box_row][check_col] {
            '[' => push_wide_box(grid, box_row, check_col, dr, dc),
            ']' => push_wide_box(grid, box_row, check_col - 1, dr, dc),
            _ => {}
        }

        // Move this box
        let new_left = (box_col as i32 + dc) as usize;
        let new_right = new_left + 1;
        grid[box_row][box_col] = '.';
        grid[box_row][box_col + 1] = '.';
        grid[box_row][new_left] = '[';
        grid[box_row][new_right] = ']';
    } else {
        // Vertical push - push all boxes that need to move first
        let new_row = (box_row as i32 + dr) as usize;
        let left_col = box_col;
        let right_col = box_col + 1;

        // Recursively push boxes above/below (using a set to avoid duplicates)
        use std::collections::HashSet;
        let mut boxes_to_push = HashSet::new();

        match grid[new_row][left_col] {
            '[' => {
                boxes_to_push.insert((new_row, left_col));
            }
            ']' => {
                boxes_to_push.insert((new_row, left_col - 1));
            }
            _ => {}
        }

        match grid[new_row][right_col] {
            '[' => {
                boxes_to_push.insert((new_row, right_col));
            }
            _ => {}
        }

        for &(r, c) in &boxes_to_push {
            push_wide_box(grid, r, c, dr, dc);
        }

        // Move this box
        grid[box_row][left_col] = '.';
        grid[box_row][right_col] = '.';
        grid[new_row][left_col] = '[';
        grid[new_row][right_col] = ']';
    }
}

fn simulate_wide_move(grid: &mut Vec<Vec<char>>, robot: &mut (usize, usize), dir: char) {
    let (dr, dc) = direction_delta(dir);
    let (robot_row, robot_col) = *robot;

    let new_row = (robot_row as i32 + dr) as usize;
    let new_col = (robot_col as i32 + dc) as usize;

    match grid[new_row][new_col] {
        '.' => {
            grid[robot_row][robot_col] = '.';
            grid[new_row][new_col] = '@';
            *robot = (new_row, new_col);
        }
        '[' => {
            if can_push_wide_box(grid, new_row, new_col, dr, dc) {
                push_wide_box(grid, new_row, new_col, dr, dc);
                grid[robot_row][robot_col] = '.';
                grid[new_row][new_col] = '@';
                *robot = (new_row, new_col);
            }
        }
        ']' => {
            let box_left_col = new_col - 1;
            if can_push_wide_box(grid, new_row, box_left_col, dr, dc) {
                push_wide_box(grid, new_row, box_left_col, dr, dc);
                grid[robot_row][robot_col] = '.';
                grid[new_row][new_col] = '@';
                *robot = (new_row, new_col);
            }
        }
        '#' => {}
        _ => {}
    }
}

fn calculate_wide_gps_sum(grid: &[Vec<char>]) -> u64 {
    let mut sum = 0u64;
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == '[' {
                sum += (100 * row + col) as u64;
            }
        }
    }
    sum
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, movements) = parse_input(input);
    let mut wide_grid = widen_grid(&grid);
    let mut robot = find_robot(&wide_grid);

    for movement in movements {
        simulate_wide_move(&mut wide_grid, &mut robot, movement);
    }

    Some(calculate_wide_gps_sum(&wide_grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
