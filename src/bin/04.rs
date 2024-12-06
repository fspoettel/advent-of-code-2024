use advent_of_code::{Direction, Point, ALL_DIRECTIONS};

advent_of_code::solution!(4);

pub struct Matrix {
    pub cells: Vec<Vec<char>>,
    pub cols: usize,
    pub rows: usize,
}

impl Matrix {
    fn get(&self, point: &Point) -> char {
        self.cells[point.y as usize][point.x as usize]
    }

    fn point_inside(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.cols as isize && point.y >= 0 && point.y < self.rows as isize
    }

    fn neighbor(&self, point: &Point, direction: Direction) -> Option<Point> {
        let neighbor = point.neighbor(direction);
        if self.point_inside(&neighbor) {
            Some(neighbor)
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Matrix {
    let cells: Vec<Vec<char>> = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.chars().collect())
            }
        })
        .collect();

    let rows = cells.len();
    let cols = cells[0].len();

    Matrix { cells, rows, cols }
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse_input(input);

    let mut sum = 0;
    let chars: [char; 3] = ['M', 'A', 'S'];

    for y in 0..matrix.cols {
        for x in 0..matrix.rows {
            let point = Point { x: x as isize, y: y as isize };
            let value = matrix.get(&point);

            if value == 'X' {
                let count: usize = ALL_DIRECTIONS
                    .into_iter()
                    .filter_map(|direction| {
                        let mut current_point = point;
                        let mut counter = 0;

                        for (i, char) in chars.iter().enumerate() {
                            match matrix.neighbor(&current_point, direction) {
                                Some(point) => {
                                    current_point = point;

                                    if *char != matrix.get(&current_point) {
                                        return None;
                                    }
                                }
                                None => {
                                    return None;
                                }
                            }

                            if i == chars.len() - 1 {
                                counter += 1;
                            }
                        }

                        Some(counter)
                    })
                    .sum();

                sum += count;
            }
        }
    }

    Some(sum)
}

const MATCHES: [[Direction; 2]; 2] = [
    [Direction::SE, Direction::NW],
    [Direction::SW, Direction::NE],
];

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = parse_input(input);

    let mut sum = 0;

    for y in 0..matrix.cols {
        for x in 0..matrix.rows {
            let point = Point { x: x as isize, y: y as isize };
            let value = matrix.get(&point);
            if value == 'A' {
                let matches = MATCHES
                    .iter()
                    .filter(|pair| {
                        let mut chars: Vec<char> = pair
                            .iter()
                            .filter_map(|dir| Some(matrix.get(&matrix.neighbor(&point, *dir)?)))
                            .collect();

                        chars.sort_unstable();
                        chars.len() == 2 && chars[0] == 'M' && chars[1] == 'S'
                    })
                    .count()
                    == 2;

                if matches {
                    sum += 1
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
