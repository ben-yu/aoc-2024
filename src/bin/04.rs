advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Some(find_xmas_in_grid(&grid) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Some(find_x_mas_in_grid(&grid) as u32)
}

fn find_xmas_in_grid(grid: &[Vec<char>]) -> usize {
    let target = "XMAS";
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let rows = grid.len();
    if rows == 0 { return 0; }
    let cols = grid[0].len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                if search_word_in_direction(grid, target, row, col, dr, dc, rows, cols) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn search_word_in_direction(
    grid: &[Vec<char>],
    word: &str,
    start_row: usize,
    start_col: usize,
    row_dir: i32,
    col_dir: i32,
    rows: usize,
    cols: usize,
) -> bool {
    let word_chars: Vec<char> = word.chars().collect();

    for (i, &ch) in word_chars.iter().enumerate() {
        let new_row = start_row as i32 + i as i32 * row_dir;
        let new_col = start_col as i32 + i as i32 * col_dir;

        if new_row < 0 || new_row >= rows as i32 || new_col < 0 || new_col >= cols as i32 {
            return false;
        }

        if grid[new_row as usize][new_col as usize] != ch {
            return false;
        }
    }

    true
}

fn find_x_mas_in_grid(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    if rows < 3 { return 0; }
    let cols = grid[0].len();
    if cols < 3 { return 0; }

    let mut count = 0;

    for row in 1..rows-1 {
        for col in 1..cols-1 {
            if grid[row][col] == 'A' && is_x_mas_center(grid, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn is_x_mas_center(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    let top_left = grid[row-1][col-1];
    let top_right = grid[row-1][col+1];
    let bottom_left = grid[row+1][col-1];
    let bottom_right = grid[row+1][col+1];

    let diagonal1_mas = (top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M');
    let diagonal2_mas = (top_right == 'M' && bottom_left == 'S') || (top_right == 'S' && bottom_left == 'M');

    diagonal1_mas && diagonal2_mas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
