use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_line(line: &str) -> Option<(u32, u32)> {
    let mut values = line.split_ascii_whitespace().map(|x| x.parse().ok());
    let x = values.next()??;
    let y = values.next()??;
    Some((x, y))
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines().filter_map(parse_line).unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a, mut b) = parse(input);

    a.sort_unstable();
    b.sort_unstable();

    Some(
        a.into_iter()
            .zip(b)
            .fold(0, |acc, (x, y)| acc + x.abs_diff(y)),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (a, b) = parse(input);

    let counts = b.into_iter().fold(HashMap::new(), |mut acc, val| {
        *acc.entry(val).or_default() += 1;
        acc
    });

    Some(a.iter().map(|x| x * counts.get(x).unwrap_or(&0)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
