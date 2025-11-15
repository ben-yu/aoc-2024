advent_of_code::solution!(16);

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    row: usize,
    col: usize,
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                start = (row, col);
            } else if ch == 'E' {
                end = (row, col);
            }
        }
    }

    (grid, start, end)
}

fn dijkstra(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> Option<u64> {
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<(usize, usize, Direction), u64> = HashMap::new();

    // Start facing East
    heap.push(State {
        cost: 0,
        row: start.0,
        col: start.1,
        dir: Direction::East,
    });
    dist.insert((start.0, start.1, Direction::East), 0);

    while let Some(State { cost, row, col, dir }) = heap.pop() {
        // Reached the end
        if (row, col) == end {
            return Some(cost);
        }

        // Skip if we've found a better path
        if let Some(&d) = dist.get(&(row, col, dir)) {
            if cost > d {
                continue;
            }
        }

        // Try moving forward
        let (dr, dc) = dir.delta();
        let new_row = (row as i32 + dr) as usize;
        let new_col = (col as i32 + dc) as usize;

        if grid[new_row][new_col] != '#' {
            let new_cost = cost + 1;
            let key = (new_row, new_col, dir);
            if !dist.contains_key(&key) || new_cost < dist[&key] {
                dist.insert(key, new_cost);
                heap.push(State {
                    cost: new_cost,
                    row: new_row,
                    col: new_col,
                    dir,
                });
            }
        }

        // Try turning left
        let left_dir = dir.turn_left();
        let new_cost = cost + 1000;
        let key = (row, col, left_dir);
        if !dist.contains_key(&key) || new_cost < dist[&key] {
            dist.insert(key, new_cost);
            heap.push(State {
                cost: new_cost,
                row,
                col,
                dir: left_dir,
            });
        }

        // Try turning right
        let right_dir = dir.turn_right();
        let new_cost = cost + 1000;
        let key = (row, col, right_dir);
        if !dist.contains_key(&key) || new_cost < dist[&key] {
            dist.insert(key, new_cost);
            heap.push(State {
                cost: new_cost,
                row,
                col,
                dir: right_dir,
            });
        }
    }

    None
}

fn dijkstra_all_distances(
    grid: &[Vec<char>],
    start: (usize, usize),
) -> HashMap<(usize, usize, Direction), u64> {
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<(usize, usize, Direction), u64> = HashMap::new();

    // Start facing East
    heap.push(State {
        cost: 0,
        row: start.0,
        col: start.1,
        dir: Direction::East,
    });
    dist.insert((start.0, start.1, Direction::East), 0);

    while let Some(State { cost, row, col, dir }) = heap.pop() {
        // Skip if we've found a better path
        if let Some(&d) = dist.get(&(row, col, dir)) {
            if cost > d {
                continue;
            }
        }

        // Try moving forward
        let (dr, dc) = dir.delta();
        let new_row = (row as i32 + dr) as usize;
        let new_col = (col as i32 + dc) as usize;

        if grid[new_row][new_col] != '#' {
            let new_cost = cost + 1;
            let key = (new_row, new_col, dir);
            if !dist.contains_key(&key) || new_cost < dist[&key] {
                dist.insert(key, new_cost);
                heap.push(State {
                    cost: new_cost,
                    row: new_row,
                    col: new_col,
                    dir,
                });
            }
        }

        // Try turning left
        let left_dir = dir.turn_left();
        let new_cost = cost + 1000;
        let key = (row, col, left_dir);
        if !dist.contains_key(&key) || new_cost < dist[&key] {
            dist.insert(key, new_cost);
            heap.push(State {
                cost: new_cost,
                row,
                col,
                dir: left_dir,
            });
        }

        // Try turning right
        let right_dir = dir.turn_right();
        let new_cost = cost + 1000;
        let key = (row, col, right_dir);
        if !dist.contains_key(&key) || new_cost < dist[&key] {
            dist.insert(key, new_cost);
            heap.push(State {
                cost: new_cost,
                row,
                col,
                dir: right_dir,
            });
        }
    }

    dist
}

fn dijkstra_reverse(
    grid: &[Vec<char>],
    end: (usize, usize),
) -> HashMap<(usize, usize, Direction), u64> {
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<(usize, usize, Direction), u64> = HashMap::new();

    // Start from end in all directions
    for dir in [Direction::North, Direction::South, Direction::East, Direction::West] {
        heap.push(State {
            cost: 0,
            row: end.0,
            col: end.1,
            dir,
        });
        dist.insert((end.0, end.1, dir), 0);
    }

    while let Some(State { cost, row, col, dir }) = heap.pop() {
        if let Some(&d) = dist.get(&(row, col, dir)) {
            if cost > d {
                continue;
            }
        }

        // Try moving backward (opposite of forward)
        let (dr, dc) = dir.delta();
        let new_row = (row as i32 - dr) as usize;
        let new_col = (col as i32 - dc) as usize;

        if grid[new_row][new_col] != '#' {
            let new_cost = cost + 1;
            let key = (new_row, new_col, dir);
            if !dist.contains_key(&key) || new_cost < dist[&key] {
                dist.insert(key, new_cost);
                heap.push(State {
                    cost: new_cost,
                    row: new_row,
                    col: new_col,
                    dir,
                });
            }
        }

        // Try turning (reverse turns have same cost)
        let left_dir = dir.turn_left();
        let new_cost = cost + 1000;
        let key = (row, col, left_dir);
        if !dist.contains_key(&key) || new_cost < dist[&key] {
            dist.insert(key, new_cost);
            heap.push(State {
                cost: new_cost,
                row,
                col,
                dir: left_dir,
            });
        }

        let right_dir = dir.turn_right();
        let new_cost = cost + 1000;
        let key = (row, col, right_dir);
        if !dist.contains_key(&key) || new_cost < dist[&key] {
            dist.insert(key, new_cost);
            heap.push(State {
                cost: new_cost,
                row,
                col,
                dir: right_dir,
            });
        }
    }

    dist
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, start, end) = parse_input(input);
    dijkstra(&grid, start, end)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, start, end) = parse_input(input);

    // Get distances from start to all states
    let dist_from_start = dijkstra_all_distances(&grid, start);

    // Get distances from all states to end (using reverse Dijkstra)
    let dist_to_end = dijkstra_reverse(&grid, end);

    // Find the optimal cost
    let optimal_cost = [Direction::North, Direction::South, Direction::East, Direction::West]
        .iter()
        .filter_map(|&dir| dist_from_start.get(&(end.0, end.1, dir)))
        .min()
        .copied()?;

    // Find all tiles on optimal paths
    let mut optimal_tiles: HashSet<(usize, usize)> = HashSet::new();

    for (&(row, col, dir), &cost_from_start) in &dist_from_start {
        if let Some(&cost_to_end) = dist_to_end.get(&(row, col, dir)) {
            if cost_from_start + cost_to_end == optimal_cost {
                optimal_tiles.insert((row, col));
            }
        }
    }

    Some(optimal_tiles.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
