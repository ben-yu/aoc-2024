advent_of_code::solution!(8);

use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let ch = grid[row][col];
            if ch != '.' {
                // Only store antenna positions, not empty spaces
                nodes.entry(ch).or_insert_with(Vec::new).push((row, col));
            }
        }
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    // For each frequency, check all unique pair combinations of antennas
    for (_frequency, positions) in &nodes {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (r1, c1) = (positions[i].0 as i32, positions[i].1 as i32);
                let (r2, c2) = (positions[j].0 as i32, positions[j].1 as i32);

                // Calculate the vector between the two antennas
                let dr = r2 - r1;
                let dc = c2 - c1;

                // Generate the two antinode positions
                let antinode1 = (r1 - dr, c1 - dc); // Before first antenna
                let antinode2 = (r2 + dr, c2 + dc); // After second antenna

                // Check bounds and add valid antinodes
                if antinode1.0 >= 0 && antinode1.0 < rows && antinode1.1 >= 0 && antinode1.1 < cols
                {
                    antinodes.insert(antinode1);
                }
                if antinode2.0 >= 0 && antinode2.0 < rows && antinode2.1 >= 0 && antinode2.1 < cols
                {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    Some(antinodes.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let ch = grid[row][col];
            if ch != '.' {
                // Only store antenna positions, not empty spaces
                nodes.entry(ch).or_insert_with(Vec::new).push((row, col));
            }
        }
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    // For each frequency, check all unique pair combinations of antennas
    for (_frequency, positions) in &nodes {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (r1, c1) = (positions[i].0 as i32, positions[i].1 as i32);
                let (r2, c2) = (positions[j].0 as i32, positions[j].1 as i32);

                // Calculate the vector between the two antennas
                let dr = r2 - r1;
                let dc = c2 - c1;

                // Generate all antinodes in the direction from r1 towards r2 and beyond
                let mut k = 0;
                loop {
                    let antinode = (r1 + k * dr, c1 + k * dc);
                    if antinode.0 >= 0 && antinode.0 < rows && antinode.1 >= 0 && antinode.1 < cols
                    {
                        antinodes.insert(antinode);
                        k += 1;
                    } else {
                        break;
                    }
                }

                // Generate all antinodes in the opposite direction from r1
                k = -1;
                loop {
                    let antinode = (r1 + k * dr, c1 + k * dc);
                    if antinode.0 >= 0 && antinode.0 < rows && antinode.1 >= 0 && antinode.1 < cols
                    {
                        antinodes.insert(antinode);
                        k -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    Some(antinodes.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
