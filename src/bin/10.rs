use std::collections::HashSet;

use advent_of_code::{Matrix, Point, CARDINALS};

advent_of_code::solution!(10);

fn parse(input: &str) -> (Matrix<u32>, Vec<Point>) {
    let mut trailheads = vec![];

    let cells: Vec<Vec<u32>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    let val = char.to_digit(10).unwrap();
                    if val == 0 {
                        trailheads.push(Point {
                            x: x as isize,
                            y: y as isize,
                        });
                    }
                    val
                })
                .collect()
        })
        .collect();

    (Matrix::from(cells), trailheads)
}

fn map_path(matrix: &Matrix<u32>, paths: Vec<Point>) -> Vec<Point> {
    paths
        .into_iter()
        .flat_map(|point| {
            let val = matrix.get(&point);
            if val == 9 {
                return vec![point];
            }

            let neighbors: Vec<Point> = CARDINALS
                .iter()
                .filter_map(|direction| {
                    let neighbor = matrix.neighbor(&point, direction)?;
                    if matrix.get(&neighbor) == val + 1 {
                        Some(neighbor)
                    } else {
                        None
                    }
                })
                .collect();

            if neighbors.is_empty() {
                vec![]
            } else {
                map_path(matrix, neighbors)
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (matrix, trailheads) = parse(input);
    Some(
        trailheads
            .into_iter()
            .map(|trailhead| HashSet::<Point>::from_iter(map_path(&matrix, vec![trailhead])).len())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (matrix, trailheads) = parse(input);
    Some(
        trailheads
            .into_iter()
            .map(|trailhead| map_path(&matrix, vec![trailhead]).len())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
