advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
            Direction::NE,
            Direction::SE,
            Direction::SW,
            Direction::NW,
        ]
    }
}

#[derive(Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

pub struct Matrix {
    pub cells: Vec<Vec<char>>,
    pub cols: usize,
    pub rows: usize,
}

impl Matrix {
    fn get(&self, point: &Point) -> char {
        self.cells[point.y][point.x]
    }

    fn neighbor(&self, point: &Point, direction: Direction) -> Option<Point> {
        match direction {
            Direction::N => {
                let x = point.x;
                let y = point.y.checked_sub(1)?;
                Some(Point { x, y })
            }
            Direction::E => {
                let x = if point.x + 1 < self.cols {
                    point.x + 1
                } else {
                    return None;
                };
                Some(Point { x, y: point.y })
            }
            Direction::S => {
                let y = if point.y + 1 < self.rows {
                    point.y + 1
                } else {
                    return None;
                };
                Some(Point { x: point.x, y })
            }
            Direction::W => {
                let x = point.x.checked_sub(1)?;
                let y = point.y;
                Some(Point { x, y })
            }
            Direction::NE => {
                let point_east = self.neighbor(point, Direction::E)?;
                self.neighbor(&point_east, Direction::N)
            }
            Direction::NW => {
                let point_west = self.neighbor(point, Direction::W)?;
                self.neighbor(&point_west, Direction::N)
            }
            Direction::SE => {
                let point_east = self.neighbor(point, Direction::E)?;
                self.neighbor(&point_east, Direction::S)
            }
            Direction::SW => {
                let point_west = self.neighbor(point, Direction::W)?;
                self.neighbor(&point_west, Direction::S)
            }
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
            let point = Point { x, y };
            let value = matrix.get(&point);

            if value == 'X' {
                let count: usize = Direction::all()
                    .into_iter()
                    .filter_map(|direction| {
                        let mut current_point = point.clone();
                        let mut counter = 0;

                        for (i, char) in chars.iter().enumerate() {
                            match matrix.neighbor(&current_point, direction) {
                                Some(point) => {
                                    current_point = point.clone();
                                    if *char != matrix.get(&point) {
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

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = parse_input(input);

    let mut sum = 0;

    for y in 0..matrix.cols {
        for x in 0..matrix.rows {
            let value = matrix.get(&Point { x, y });
            if value == 'A' {
                let point = Point { x, y };

                let matches = [
                    [Direction::SE, Direction::NW],
                    [Direction::SW, Direction::NE],
                ]
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
