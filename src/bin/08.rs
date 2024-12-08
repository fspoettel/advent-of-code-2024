use std::collections::{HashMap, HashSet};

use advent_of_code::Point;
use itertools::Itertools;

advent_of_code::solution!(8);

type Antennas = HashMap<char, Vec<Point>>;

#[derive(Debug)]
struct Grid {
    antennas: Antennas,
    cols: usize,
    rows: usize,
}

impl Grid {
    fn point_inside(&self, point: &Point) -> bool {
        point.x >= 0
            && point.x <= self.cols as isize
            && point.y >= 0
            && point.y <= self.rows as isize
    }
}

fn parse(input: &str) -> Grid {
    let mut antennas: Antennas = HashMap::new();
    let mut cols = 0;
    let mut rows = 0;

    for (y, line) in input.lines().enumerate() {
        rows = rows.max(y);

        for (x, c) in line.chars().enumerate() {
            cols = cols.max(x);

            if c != '.' {
                let point = Point {
                    x: x as isize,
                    y: y as isize,
                };
                antennas.entry(c).or_default().push(point)
            }
        }
    }

    Grid {
        antennas,
        rows,
        cols,
    }
}

fn process_pair(
    grid: &Grid,
    antinodes: &mut HashSet<Point>,
    a: &Point,
    b: &Point,
    offset: isize,
) -> isize {
    let dist_x = b.x - a.x;
    let dist_y = b.y - a.y;

    let anti_a = Point {
        x: a.x - dist_x * offset,
        y: a.y - dist_y * offset,
    };

    let anti_b = Point {
        x: b.x + dist_x * offset,
        y: b.y + dist_y * offset,
    };

    let mut match_count = 0;

    let a_inside = grid.point_inside(&anti_a);
    if a_inside {
        match_count += 1;
        antinodes.insert(anti_a);
    }

    let b_inside = grid.point_inside(&anti_b);
    if b_inside {
        match_count += 1;
        antinodes.insert(anti_b);
    }

    match_count
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let mut antinodes: HashSet<Point> = HashSet::new();

    for antennas in grid.antennas.values() {
        for pair in antennas.iter().combinations(2) {
            process_pair(&grid, &mut antinodes, pair[0], pair[1], 1);
        }
    }

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let mut antinodes: HashSet<Point> = HashSet::new();

    for antennas in grid.antennas.values() {
        for pair in antennas.iter().combinations(2) {
            let mut offset = 0;
            loop {
                let match_count = process_pair(&grid, &mut antinodes, pair[0], pair[1], offset);
                if match_count == 0 {
                    break;
                } else {
                    offset += 1;
                }
            }
        }
    }

    Some(antinodes.len())
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
