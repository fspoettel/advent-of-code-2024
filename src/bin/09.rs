use std::collections::{HashMap, HashSet};

advent_of_code::solution!(9);

enum Allocation {
    File(File),
    Free(u32),
}

struct File {
    id: u32,
    size: u32,
}

enum Block {
    File(u32),
    Free,
}

fn parse(input: &str) -> Vec<Allocation> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .fold(vec![], |mut acc, (i, char)| {
            let size = char.to_digit(10).unwrap();
            if i % 2 == 0 {
                acc.push(Allocation::File(File {
                    id: (i as u32 / 2),
                    size,
                }))
            } else {
                acc.push(Allocation::Free(size));
            }
            acc
        })
}

fn allocate_disk(allocations: &[Allocation]) -> Vec<Block> {
    let mut disk = vec![];

    for alloc in allocations {
        match alloc {
            Allocation::Free(size) => disk.extend((0..*size).map(|_| Block::Free)),
            Allocation::File(file) => {
                disk.extend((0..file.size).map(|_| Block::File(file.id)));
            }
        }
    }

    disk
}

fn checksum(disk: &[Block]) -> u64 {
    disk.iter()
        .enumerate()
        .fold(0, |acc, (index, block)| match block {
            Block::File(id) => acc + index as u64 * *id as u64,
            Block::Free => acc,
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = allocate_disk(&parse(input));

    let mut p_front = 0;
    let mut p_back = disk.len() - 1;

    while p_back > p_front {
        if let Block::Free = &disk[p_front] {
            while p_back > p_front {
                if let Block::File(_) = &disk[p_back] {
                    disk.swap(p_front, p_back);
                    break;
                } else {
                    p_back -= 1;
                }
            }
        } else {
            p_front += 1;
        }
    }

    Some(checksum(&disk))
}

pub fn part_two(input: &str) -> Option<u64> {
    let allocations = parse(input);

    let file_size_map = allocations
        .iter()
        .fold(HashMap::new(), |mut acc, allocation| {
            if let Allocation::File(file) = allocation {
                *acc.entry(file.id).or_default() = file.size;
            }
            acc
        });

    let mut disk = allocate_disk(&allocations);
    let mut moved: HashSet<u32> = HashSet::new();

    let mut i = disk.len() - 1;

    while i > 0 {
        if let Block::File(id) = &disk[i] {
            if moved.contains(id) {
                i -= 1;
                continue;
            }

            let size = file_size_map.get(id).copied().unwrap() as usize;

            let file_start = i + 1 - size;
            let file_end = i;

            let mut j = 0;
            let mut current_free = 0;

            while j < file_start && current_free < size {
                match disk[j] {
                    Block::Free => current_free += 1,
                    Block::File(_) => current_free = 0,
                }
                j += 1;
            }

            if current_free == size {
                moved.insert(*id);
                for k in 0..size {
                    disk.swap(j - size + k, file_end - k);
                }
            }

            if size <= i {
                i -= size;
            } else {
                break;
            }
        } else {
            i -= 1;
        }
    }

    Some(checksum(&disk))
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
