use std::collections::{HashMap, HashSet};

use advent_of_code::{Direction, Point};

advent_of_code::solution!(6);

#[derive(Clone)]
struct SparseGrid {
    rows: usize,
    cols: usize,
    entries: HashMap<Point, bool>,
}

impl SparseGrid {
    fn point_inside(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.cols as isize && point.y >= 0 && point.y < self.rows as isize
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Guard {
    position: Point,
    direction: Direction,
}

fn parse(input: &str) -> (SparseGrid, Guard) {
    let mut entries = HashMap::new();
    let mut guard = None;

    let mut cols = 0;
    let mut rows = 0;

    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        rows += 1;

        if cols == 0 {
            cols = line.len();
        }

        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    entries.insert(
                        Point {
                            x: x as isize,
                            y: y as isize,
                        },
                        true,
                    );
                }
                '^' => {
                    guard = Some(Guard {
                        position: Point {
                            x: x as isize,
                            y: y as isize,
                        },
                        direction: Direction::N,
                    });
                }
                _ => {}
            }
        }
    }

    (
        SparseGrid {
            cols,
            rows,
            entries,
        },
        guard.unwrap(),
    )
}

fn walk_path(grid: &SparseGrid, guard: &mut Guard) -> Option<usize> {
    let mut guards: HashSet<Guard> = HashSet::from_iter([*guard]);

    loop {
        let next_position = guard.position.neighbor(guard.direction);

        if !grid.point_inside(&next_position) {
            break;
        } else if grid.entries.contains_key(&next_position) {
            guard.direction = guard.direction.rotate_clockwise();
        } else {
            guard.position = next_position;
            if guards.contains(guard) {
                return None;
            } else {
                guards.insert(*guard);
            }
        }
    }

    let set: HashSet<Point> = HashSet::from_iter(guards.iter().map(|g| g.position));
    Some(set.len())
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, mut guard) = parse(input);
    walk_path(&grid, &mut guard)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, guard) = parse(input);

    let mut cycles = 0;

    for x in 0..grid.cols {
        for y in 0..grid.rows {
            let mut loop_grid = grid.clone();

            loop_grid.entries.insert(
                Point {
                    x: x as isize,
                    y: y as isize,
                },
                true,
            );

            let mut loop_guard = guard;

            if walk_path(&loop_grid, &mut loop_guard).is_none() {
                cycles += 1;
            }
        }
    }

    Some(cycles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
