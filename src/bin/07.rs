advent_of_code::solution!(7);

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .filter_map(|line| {
            let (result_s, values_s) = line.split_once(':')?;
            let result = result_s.parse().ok()?;
            let values = values_s
                .split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            Some((result, values))
        })
        .collect()
}

fn eval(
    target_value: u64,
    pool: &[Operator],
    values: &[u64],
    index: usize,
    current_value: u64,
    op: Option<Operator>,
) -> bool {
    if index == values.len() {
        return current_value == target_value;
    }

    let next_value = match op {
        Some(op) => {
            let x = current_value;
            let y = values[index];
            match op {
                Operator::Add => x + y,
                Operator::Mul => x * y,
                Operator::Concat => x * 10u64.pow(y.ilog10() + 1) + y,
            }
        }
        None => values[index],
    };

    if next_value > target_value {
        return false;
    }

    pool.iter()
        .any(|op| eval(target_value, pool, values, index + 1, next_value, Some(*op)))
}

fn solve(input: &str, operators: &[Operator]) -> u64 {
    parse(input)
        .into_iter()
        .filter(|(result, values)| eval(*result, operators, values, 0, 0, None))
        .map(|(result, _)| result)
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, &[Operator::Add, Operator::Mul]))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(
        input,
        &[Operator::Add, Operator::Mul, Operator::Concat],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
