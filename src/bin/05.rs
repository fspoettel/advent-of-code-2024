use std::collections::HashMap;

advent_of_code::solution!(5);

type Graph = HashMap<usize, Vec<usize>>;

fn parse(input: &str) -> Option<(Graph, Vec<Vec<usize>>)> {
    let (head, tail) = input.split_once("\n\n")?;

    let graph: Graph = head
        .split("\n")
        .filter_map(|line| {
            let (x, y) = line.split_once('|')?;
            Some((x.parse().ok()?, y.parse().ok()?))
        })
        .fold(HashMap::new(), |mut acc, (pre, post)| {
            acc.entry(post).or_default().push(pre);
            acc
        });

    let updates = tail
        .lines()
        .map(|line| line.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();

    Some((graph, updates))
}

fn validate_update(graph: &Graph, update: &[usize]) -> bool {
    for (i, x) in update.iter().enumerate() {
        let rest = &update[i..];

        if let Some(pre) = graph.get(x) {
            if rest.iter().any(|y| pre.contains(y)) {
                return false;
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let (graph, updates) = parse(input)?;
    Some(
        updates
            .into_iter()
            .filter(|update| validate_update(&graph, update))
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
