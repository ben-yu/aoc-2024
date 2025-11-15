advent_of_code::solution!(14);

use regex::Regex;

#[derive(Debug, Clone)]
struct Robot {
    px: i64, // position x
    py: i64, // position y
    vx: i64, // velocity x
    vy: i64, // velocity y
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = Vec::new();

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            robots.push(Robot {
                px: caps[1].parse().unwrap(),
                py: caps[2].parse().unwrap(),
                vx: caps[3].parse().unwrap(),
                vy: caps[4].parse().unwrap(),
            });
        }
    }

    robots
}

fn simulate_robots(robots: &[Robot], width: i64, height: i64, seconds: i64) -> Vec<(i64, i64)> {
    robots
        .iter()
        .map(|robot| {
            // Calculate final position with modulo wrapping
            let final_x = ((robot.px + robot.vx * seconds) % width + width) % width;
            let final_y = ((robot.py + robot.vy * seconds) % height + height) % height;
            (final_x, final_y)
        })
        .collect()
}

fn calculate_safety_factor(positions: &[(i64, i64)], width: i64, height: i64) -> u64 {
    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut quadrants = [0u64; 4];

    for &(x, y) in positions {
        // Skip robots on middle lines
        if x == mid_x || y == mid_y {
            continue;
        }

        // Determine quadrant
        let quadrant = match (x < mid_x, y < mid_y) {
            (true, true) => 0,   // top-left
            (false, true) => 1,  // top-right
            (true, false) => 2,  // bottom-left
            (false, false) => 3, // bottom-right
        };

        quadrants[quadrant] += 1;
    }

    quadrants.iter().product()
}

fn solve_part_one(input: &str, width: i64, height: i64) -> u64 {
    let robots = parse_input(input);
    let positions = simulate_robots(&robots, width, height, 100);
    calculate_safety_factor(&positions, width, height)
}

pub fn part_one(input: &str) -> Option<u64> {
    // Real input uses 101x103 grid
    Some(solve_part_one(input, 101, 103))
}

fn render_grid(positions: &[(i64, i64)], width: i64, height: i64) -> String {
    use std::collections::HashSet;
    let pos_set: HashSet<(i64, i64)> = positions.iter().cloned().collect();

    let mut result = String::new();
    for y in 0..height {
        for x in 0..width {
            if pos_set.contains(&(x, y)) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }
    result
}

fn count_adjacent_robots(positions: &[(i64, i64)]) -> usize {
    use std::collections::HashSet;
    let pos_set: HashSet<(i64, i64)> = positions.iter().cloned().collect();

    let mut count = 0;
    for &(x, y) in positions {
        // Check 8 neighbors (including diagonals)
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if pos_set.contains(&(x + dx, y + dy)) {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let robots = parse_input(input);
    let width = 101i64;
    let height = 103i64;

    // Pattern repeats after width * height seconds (both are prime)
    let max_time = width * height;

    let mut best_time = 0;
    let mut max_adjacent = 0;

    // Find the time with maximum adjacent robots
    for t in 0..max_time {
        let positions = simulate_robots(&robots, width, height, t);
        let adjacent = count_adjacent_robots(&positions);

        if adjacent > max_adjacent {
            max_adjacent = adjacent;
            best_time = t;
        }
    }

    // Display the tree
    let positions = simulate_robots(&robots, width, height, best_time);
    println!("=== Christmas Tree at Time: {} ===", best_time);
    println!("{}", render_grid(&positions, width, height));

    Some(best_time as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        // Example uses 11x7 grid
        let result = solve_part_one(&input, 11, 7);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
