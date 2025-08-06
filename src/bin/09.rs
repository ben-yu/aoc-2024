advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map: Vec<char> = input.trim().chars().collect();
    let mut file_id = 0;
    let mut file_block_map: Vec<Option<usize>> = Vec::new();

    for i in 0..disk_map.len() {
        let file_size = disk_map[i].to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            let mut file = vec![Some(file_id); file_size];
            file_block_map.append(&mut file);
            file_id += 1;
        } else {
            let mut file = vec![None; file_size];
            file_block_map.append(&mut file);
        }
    }

    for i in (0..file_block_map.len()).rev() {
        for j in 0..file_block_map.len() {
            if j >= i {
                break;
            }
            if file_block_map[j].is_none() && file_block_map[i].is_some() {
                file_block_map.swap(i,j);
                break;
            }
        }
    }

    let mut checksum = 0;
    for i in 0..file_block_map.len() {
        if let Some(id) = file_block_map[i] {
            checksum += id * i;
        }
    }

    Some(checksum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    use std::collections::HashMap;

    let disk_map: Vec<char> = input.trim().chars().collect();
    let mut file_id = 0;
    let mut file_block_map: Vec<Option<usize>> = Vec::new();
    let mut free_spaces: HashMap<usize, usize> = HashMap::new(); // position -> size
    let mut file_positions: HashMap<usize, (usize, usize)> = HashMap::new(); // file_id -> (start, size)

    // Build the initial disk layout and track free spaces and file positions
    let mut pos = 0;
    for i in 0..disk_map.len() {
        let size = disk_map[i].to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            // File
            file_positions.insert(file_id, (pos, size));
            let mut file = vec![Some(file_id); size];
            file_block_map.append(&mut file);
            file_id += 1;
        } else {
            // Free space
            if size > 0 {
                free_spaces.insert(pos, size);
            }
            let mut space = vec![None; size];
            file_block_map.append(&mut space);
        }
        pos += size;
    }

    // Helper function to find leftmost free space that can fit file
    fn find_free_space(free_spaces: &HashMap<usize, usize>, file_size: usize, file_pos: usize) -> Option<usize> {
        free_spaces
            .iter()
            .filter(|(&pos, &size)| pos < file_pos && size >= file_size)
            .min_by_key(|(&pos, _)| pos)
            .map(|(&pos, _)| pos)
    }

    // Helper function to move a whole file
    fn move_file(
        file_block_map: &mut Vec<Option<usize>>,
        free_spaces: &mut HashMap<usize, usize>,
        file_id: usize,
        file_start: usize,
        file_size: usize,
        target_pos: usize,
    ) {
        // Clear old position
        for i in file_start..file_start + file_size {
            file_block_map[i] = None;
        }

        // Place file at new position
        for i in target_pos..target_pos + file_size {
            file_block_map[i] = Some(file_id);
        }

        // Update free space tracking
        let old_free_size = free_spaces.remove(&target_pos).unwrap();
        if old_free_size > file_size {
            // Add remaining free space
            free_spaces.insert(target_pos + file_size, old_free_size - file_size);
        }

        // Add new free space where file used to be
        free_spaces.insert(file_start, file_size);
    }

    // Process files in decreasing order of file ID
    for current_file_id in (0..file_id).rev() {
        if let Some(&(file_start, file_size)) = file_positions.get(&current_file_id) {
            if let Some(target_pos) = find_free_space(&free_spaces, file_size, file_start) {
                move_file(
                    &mut file_block_map,
                    &mut free_spaces,
                    current_file_id,
                    file_start,
                    file_size,
                    target_pos,
                );
            }
        }
    }

    // Calculate checksum
    let mut checksum = 0;
    for i in 0..file_block_map.len() {
        if let Some(id) = file_block_map[i] {
            checksum += id * i;
        }
    }

    Some(checksum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
