use std::collections::HashMap;
use std::iter::zip;

advent_of_code::solution!(1);

fn solve(input: &str) -> (i32, i32) {
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut counts: HashMap<_, i32> = HashMap::new();

    for line in input.lines() {
        let mut result = line.split_whitespace();
        let res_0 = result.next().unwrap().parse::<i32>().unwrap();
        let res_1 = result.next().unwrap().parse::<i32>().unwrap();

        a.push(res_0);
        b.push(res_1);
        counts
            .entry(res_1)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    a.sort();
    b.sort();

    let mut dist = 0;
    let mut sum = 0;

    for (x, y) in zip(a, b) {
        dist += (x - y).abs();
        match counts.get(&x) {
            Some(&count) => sum += x * count,
            _ => (),
        }
    }

    (dist, sum)
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(solve(input).0)
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(solve(input).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
