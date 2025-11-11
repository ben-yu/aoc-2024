advent_of_code::solution!(12);

use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn flood_fill(
    grid: &Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> (u64, u64) {
    let rows = grid.len();
    let cols = grid[0].len();
    let target_char = grid[start_row][start_col];

    let mut stack = vec![(start_row, start_col)];
    let mut area = 0u64;
    let mut perimeter = 0u64;

    while let Some((row, col)) = stack.pop() {
        if visited.contains(&(row, col)) {
            continue;
        }

        visited.insert((row, col));
        area += 1;

        // Check all 4 neighbors
        let neighbors = [
            (row.wrapping_sub(1), col, row > 0),
            (row + 1, col, row + 1 < rows),
            (row, col.wrapping_sub(1), col > 0),
            (row, col + 1, col + 1 < cols),
        ];

        for (next_row, next_col, in_bounds) in neighbors {
            if !in_bounds {
                // Out of bounds = perimeter edge
                perimeter += 1;
            } else if grid[next_row][next_col] != target_char {
                // Different character = perimeter edge
                perimeter += 1;
            } else if !visited.contains(&(next_row, next_col)) {
                // Same character and not visited = part of region
                stack.push((next_row, next_col));
            }
        }
    }

    (area, perimeter)
}

fn flood_fill_with_edges(
    grid: &Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> (u64, Vec<(usize, usize, Direction)>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let target_char = grid[start_row][start_col];

    let mut stack = vec![(start_row, start_col)];
    let mut area = 0u64;
    let mut edges = Vec::new();

    while let Some((row, col)) = stack.pop() {
        if visited.contains(&(row, col)) {
            continue;
        }

        visited.insert((row, col));
        area += 1;

        // Check all 4 neighbors and record edges
        // North
        if row == 0 || grid[row - 1][col] != target_char {
            edges.push((row, col, Direction::North));
        } else if !visited.contains(&(row - 1, col)) {
            stack.push((row - 1, col));
        }

        // South
        if row + 1 >= rows || grid[row + 1][col] != target_char {
            edges.push((row, col, Direction::South));
        } else if !visited.contains(&(row + 1, col)) {
            stack.push((row + 1, col));
        }

        // West
        if col == 0 || grid[row][col - 1] != target_char {
            edges.push((row, col, Direction::West));
        } else if !visited.contains(&(row, col - 1)) {
            stack.push((row, col - 1));
        }

        // East
        if col + 1 >= cols || grid[row][col + 1] != target_char {
            edges.push((row, col, Direction::East));
        } else if !visited.contains(&(row, col + 1)) {
            stack.push((row, col + 1));
        }
    }

    (area, edges)
}

fn count_sides(edges: Vec<(usize, usize, Direction)>) -> u64 {
    let mut sides_by_direction: HashMap<Direction, Vec<(usize, usize)>> = HashMap::new();

    // Group edges by direction
    for (row, col, dir) in edges {
        sides_by_direction.entry(dir).or_insert_with(Vec::new).push((row, col));
    }

    let mut total_sides = 0u64;

    // Count continuous segments for each direction
    for (dir, positions) in sides_by_direction {
        match dir {
            Direction::North | Direction::South => {
                // Horizontal edges: group by row, sort by col, count segments
                let mut by_row: HashMap<usize, Vec<usize>> = HashMap::new();
                for (row, col) in positions {
                    by_row.entry(row).or_insert_with(Vec::new).push(col);
                }

                for (_row, mut cols) in by_row {
                    cols.sort_unstable();
                    let mut segments = 1;
                    for i in 1..cols.len() {
                        if cols[i] != cols[i - 1] + 1 {
                            segments += 1;
                        }
                    }
                    total_sides += segments;
                }
            }
            Direction::East | Direction::West => {
                // Vertical edges: group by col, sort by row, count segments
                let mut by_col: HashMap<usize, Vec<usize>> = HashMap::new();
                for (row, col) in positions {
                    by_col.entry(col).or_insert_with(Vec::new).push(row);
                }

                for (_col, mut rows) in by_col {
                    rows.sort_unstable();
                    let mut segments = 1;
                    for i in 1..rows.len() {
                        if rows[i] != rows[i - 1] + 1 {
                            segments += 1;
                        }
                    }
                    total_sides += segments;
                }
            }
        }
    }

    total_sides
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return Some(0);
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut total_cost = 0u64;

    for row in 0..rows {
        for col in 0..cols {
            if !visited.contains(&(row, col)) {
                let (area, perimeter) = flood_fill(&grid, row, col, &mut visited);
                let cost = area * perimeter;
                total_cost += cost;
            }
        }
    }

    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return Some(0);
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut total_cost = 0u64;

    for row in 0..rows {
        for col in 0..cols {
            if !visited.contains(&(row, col)) {
                let (area, edges) = flood_fill_with_edges(&grid, row, col, &mut visited);
                let sides = count_sides(edges);
                let cost = area * sides;
                total_cost += cost;
            }
        }
    }

    Some(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
