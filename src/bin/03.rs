use regex::Regex;
use once_cell::sync::Lazy;

advent_of_code::solution!(3);

enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

static INSTRUCTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(mul|do|don't)\(((?:\d|,)*?)\)").unwrap()
});


fn parse(input: &str) -> Vec<Instruction> {
    INSTRUCTION_REGEX
        .captures_iter(input)
        .filter_map(|c| {
            let (_, [instruction, value]) = c.extract();

            match instruction {
                "mul" => {
                    let (x, y) = value.split_once(",")?;
                    Some(Instruction::Mul(x.parse().ok()?, y.parse().ok()?))
                }
                "do" => Some(Instruction::Do),
                "don't" => Some(Instruction::Dont),
                _ => None,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
        .into_iter()
        .filter_map(|instruction| match instruction {
            Instruction::Mul(x, y) => Some(x * y),
            _ => None,
        })
        .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    let groups = parse(input);

    let mut enabled = true;

    for group in groups {
        match group {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(x, y) => {
                if enabled {
                    sum += x * y;
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
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
