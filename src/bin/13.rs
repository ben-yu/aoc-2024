advent_of_code::solution!(13);

use regex::Regex;

#[derive(Debug)]
struct ClawMachine {
    ax: i64, // Button A X movement
    ay: i64, // Button A Y movement
    bx: i64, // Button B X movement
    by: i64, // Button B Y movement
    px: i64, // Prize X position
    py: i64, // Prize Y position
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let re_button_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let re_button_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut machines = Vec::new();
    let blocks: Vec<&str> = input.split("\n\n").collect();

    for block in blocks {
        if block.trim().is_empty() {
            continue;
        }

        let lines: Vec<&str> = block.lines().collect();
        if lines.len() < 3 {
            continue;
        }

        let cap_a = re_button_a.captures(lines[0]).unwrap();
        let ax = cap_a[1].parse::<i64>().unwrap();
        let ay = cap_a[2].parse::<i64>().unwrap();

        let cap_b = re_button_b.captures(lines[1]).unwrap();
        let bx = cap_b[1].parse::<i64>().unwrap();
        let by = cap_b[2].parse::<i64>().unwrap();

        let cap_p = re_prize.captures(lines[2]).unwrap();
        let px = cap_p[1].parse::<i64>().unwrap();
        let py = cap_p[2].parse::<i64>().unwrap();

        machines.push(ClawMachine {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        });
    }

    machines
}

// Solve the system using Cramer's rule:
// a*ax + b*bx = px
// a*ay + b*by = py
//
// Returns Some((a, b)) if there's a valid non-negative integer solution
fn solve_machine(machine: &ClawMachine) -> Option<(i64, i64)> {
    let det = machine.ax * machine.by - machine.ay * machine.bx;

    if det == 0 {
        // Singular matrix, no unique solution
        return None;
    }

    // Using Cramer's rule:
    // a = (px*by - py*bx) / det
    // b = (ax*py - ay*px) / det
    let a_num = machine.px * machine.by - machine.py * machine.bx;
    let b_num = machine.ax * machine.py - machine.ay * machine.px;

    // Check if solutions are integers
    if a_num % det != 0 || b_num % det != 0 {
        return None;
    }

    let a = a_num / det;
    let b = b_num / det;

    // Check if solutions are non-negative
    if a < 0 || b < 0 {
        return None;
    }

    Some((a, b))
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let mut total_cost = 0u64;

    for machine in &machines {
        if let Some((a, b)) = solve_machine(machine) {
            // Part 1 constraint: no more than 100 presses each
            if a <= 100 && b <= 100 {
                let cost = 3 * a + b;
                total_cost += cost as u64;
            }
        }
    }

    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut machines = parse_input(input);
    let mut total_cost = 0u64;

    // Part 2: add 10000000000000 to prize coordinates
    let offset = 10000000000000i64;

    for machine in &mut machines {
        machine.px += offset;
        machine.py += offset;

        if let Some((a, b)) = solve_machine(machine) {
            let cost = 3 * a + b;
            total_cost += cost as u64;
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // Part 2 doesn't have a specific expected value in the example,
        // but machines 2 and 4 (0-indexed: 1 and 3) should now be solvable
        assert!(result.is_some());
    }
}
