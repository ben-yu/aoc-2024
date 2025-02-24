advent_of_code::solution!(2);

fn report_matrix(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i64>().ok())
                .collect()
        })
        .collect()
}

fn criteria(input: &Vec<i64>) -> bool {
    (input.windows(2).all(|w| w[0] < w[1]) || // all increasing
     input.windows(2).all(|w| w[0] > w[1])) && // or decreasing
    input.windows(2).all(|w| (w[0] - w[1]).abs() >= 1) && // adj diff
        input.windows(2).all(|w| (w[0] - w[1]).abs() <= 3)
}

fn single_bad_level(level: &Vec<i64>) -> bool {
    if criteria(level) {
        return true;
    }

    for (i, _) in level.iter().enumerate() {
        let mut level_copy = level.clone();
        level_copy.remove(i);
        if criteria(&level_copy) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        report_matrix(input)
            .into_iter()
            .filter(|x| criteria(x))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        report_matrix(input)
            .into_iter()
            .filter(|x| single_bad_level(x))
            .count() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
