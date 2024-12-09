use std::collections::HashSet;

advent_of_code::solution!(9);

#[derive(Clone, Copy)]
enum Allocation {
    File(File),
    Free(u32),
}

#[derive(Clone, Copy)]
struct File {
    id: u32,
    size: u32,
}

#[derive(Debug)]
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

    let mut defragged = allocations.clone();

    allocations.iter().rev().for_each(|to_move| {
        if let Allocation::File(file) = to_move {
            let target = defragged.iter().enumerate().find_map(|(idx, alloc)| {
                if let Allocation::Free(size) = alloc {
                    if file.size <= *size {
                        let free_space_left = size - file.size;
                        let mut to_insert = vec![*to_move];

                        if free_space_left > 0 {
                            to_insert.push(Allocation::Free(free_space_left));
                        }

                        return Some((idx, to_insert));
                    }
                }

                None
            });

            if let Some((target_idx, to_insert)) = target {
                defragged.splice(target_idx..=target_idx, to_insert);
            }
        }
    });

    let mut seen: HashSet<u32> = HashSet::new();

    for allocation in &mut defragged {
        if let Allocation::File(file) = allocation {
            if seen.contains(&file.id) {
                *allocation = Allocation::Free(file.size);
            } else {
                seen.insert(file.id);
            }
        }
    }

    Some(checksum(&allocate_disk(&defragged)))
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
