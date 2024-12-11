use cached::proc_macro::cached;

advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|x| x.parse().ok())
        .collect()
}

#[cached]
fn blink(stone: u64, times: u64) -> u64 {
    if times == 0 {
        return 1;
    }

    if stone == 0 {
        return blink(1, times - 1);
    }

    let digits = (stone as f32).log10().floor() as u32 + 1;

    if digits % 2 == 0 {
        let n = 10_u64.pow(digits / 2);
        return blink(stone / n, times - 1) + blink(stone % n, times - 1);
    }

    return blink(stone * 2024, times - 1);
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse(input).into_iter().map(|x| blink(x, 25)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse(input).into_iter().map(|x| blink(x, 75)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
