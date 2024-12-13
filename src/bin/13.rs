advent_of_code::solution!(13);

struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    p_x: i64,
    p_y: i64,
}

fn button_value(str: &str) -> Option<i64> {
    str.split_once('+').and_then(|n| n.1.parse().ok())
}

fn button(line: &str) -> Option<(i64, i64)> {
    let (x_str, y_str) = line.split_once(": ")?.1.split_once(", ")?;
    button_value(x_str).zip(button_value(y_str))
}

fn prize_value(str: &str) -> Option<i64> {
    str.split_once('=').and_then(|n| n.1.parse().ok())
}

fn prize(line: &str) -> Option<(i64, i64)> {
    let (x_str, y_str) = line.split_once(": ")?.1.split_once(", ")?;
    prize_value(x_str).zip(prize_value(y_str))
}

fn parse_machine(value: &str) -> Option<Machine> {
    let mut lines = value.lines();
    let (a_x, a_y) = button(lines.next()?)?;
    let (b_x, b_y) = button(lines.next()?)?;
    let (p_x, p_y) = prize(lines.next()?)?;
    Some(Machine {
        a_x,
        a_y,
        b_x,
        b_y,
        p_x,
        p_y,
    })
}

fn parse(input: &str) -> Vec<Machine> {
    input.split("\n\n").filter_map(parse_machine).collect()
}

fn solve(machine: &Machine) -> Option<i64> {
    let Machine {
        a_x,
        a_y,
        b_x,
        b_y,
        p_x,
        p_y,
    } = machine;

    let det = a_x * b_y - a_y * b_x;

    let x = (p_x * b_y - p_y * b_x) / det;
    let y = (p_y * a_x - p_x * a_y) / det;

    if x * a_x + y * b_x == *p_x && x * a_y + y * b_y == *p_y {
        Some(3 * x + y)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = parse(input);

    Some(
        machines
            .into_iter()
            .filter_map(|machine| solve(&machine))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut machines = parse(input);

    Some(
        machines
            .iter_mut()
            .filter_map(|machine| {
                machine.p_x += 10000000000000;
                machine.p_y += 10000000000000;
                solve(machine)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }
}
