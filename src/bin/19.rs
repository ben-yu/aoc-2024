use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    fn insert(&mut self, pattern: &str) {
        let mut node = &mut self.root;
        for ch in pattern.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.is_end = true;
    }

    fn can_construct(&self, design: &str) -> bool {
        let chars: Vec<char> = design.chars().collect();
        let n = chars.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 0..n {
            if !dp[i] {
                continue;
            }

            let mut node = &self.root;
            for j in i..n {
                if let Some(next) = node.children.get(&chars[j]) {
                    node = next;
                    if node.is_end {
                        dp[j + 1] = true;
                    }
                } else {
                    break;
                }
            }
        }

        dp[n]
    }

    fn count_arrangements(&self, design: &str) -> u64 {
        let chars: Vec<char> = design.chars().collect();
        let n = chars.len();
        let mut dp = vec![0u64; n + 1];
        dp[0] = 1;

        for i in 0..n {
            if dp[i] == 0 {
                continue;
            }

            let mut node = &self.root;
            for j in i..n {
                if let Some(next) = node.children.get(&chars[j]) {
                    node = next;
                    if node.is_end {
                        dp[j + 1] += dp[i];
                    }
                } else {
                    break;
                }
            }
        }

        dp[n]
    }
}

fn parse_input(input: &str) -> (Trie, Vec<&str>) {
    let mut lines = input.lines();

    let mut trie = Trie::new();
    if let Some(patterns_line) = lines.next() {
        for pattern in patterns_line.split(", ") {
            trie.insert(pattern.trim());
        }
    }

    lines.next();

    let designs: Vec<&str> = lines.collect();

    (trie, designs)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (trie, designs) = parse_input(input);

    let count = designs
        .iter()
        .filter(|design| trie.can_construct(design))
        .count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (trie, designs) = parse_input(input);

    let total = designs
        .iter()
        .map(|design| trie.count_arrangements(design))
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
