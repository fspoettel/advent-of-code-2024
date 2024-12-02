advent_of_code::solution!(2);

fn parse(line: &str) -> Vec<u32> {
    line.split_ascii_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect()
}

fn validate(values: Vec<u32>, fault_correct: bool) -> bool {
    if values.len() < 2 {
        return false;
    }

    let increasing = values[1] > values[0];

    let is_safe = values.windows(2).all(|pairs| {
        let x = pairs[0];
        let y = pairs[1];

        if (increasing && y <= x) || (!increasing && y >= x) {
            return false;
        }

        (1..=3).contains(&x.abs_diff(y))
    });

    if !is_safe && fault_correct {
        (0..values.len())
            .map(|i| {
                let mut arr = values.clone();
                arr.remove(i);
                arr
            })
            .any(|v| validate(v, false))
    } else {
        is_safe
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| validate(parse(line), false))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| validate(parse(line), true))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
