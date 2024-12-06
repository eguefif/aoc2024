fn main() {
    let (rules, input) = get_input();

    println!("Part1: {}", part1(input.clone(), rules.clone()));
    println!("Part2: {}", part2(input.clone(), rules.clone()));
}

fn part1(input: Vec<Vec<u32>>, rules: Vec<(u32, u32)>) -> u32 {
    input.iter().fold(0, |mut acc, row| {
        if is_sorted(row, &rules) {
            let n = row.len() / 2 as usize;
            acc += row[n];
        }
        acc
    })
}

fn is_sorted(row: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    row.into_iter()
        .zip(row[1..row.len()].iter())
        .all(|(a, b)| rules.iter().any(|(x, y)| *x == *a && *y == *b))
}

fn part2(input: Vec<Vec<u32>>, rules: Vec<(u32, u32)>) -> u32 {
    let input_p2 = get_unsorted(input, &rules);
    input_p2.iter().fold(0, |acc, row| {
        let n = row.len().div_euclid(2);
        let sorted = sort_row(&mut row.clone(), &rules);
        acc + sorted[n]
    })
}

fn sort_row(row: &mut Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    for i in 0..row.len() {
        for j in (i + 1)..row.len() {
            if rules.iter().any(|(x, y)| *x == row[i] && *y == row[j]) {
                let tmp = row[i];
                row[i] = row[j];
                row[j] = tmp;
            }
        }
    }
    row.clone()
}

fn get_unsorted(input: Vec<Vec<u32>>, rules: &Vec<(u32, u32)>) -> Vec<Vec<u32>> {
    let mut retval: Vec<Vec<u32>> = vec![];
    input.iter().for_each(|row| {
        if !is_sorted(row, &rules) {
            retval.push(row.clone());
        }
    });
    retval
}

fn get_input() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let content = include_str!("../../inputs/d5");
    let mut chunks = content.split("\n\n");

    let mut rules = Vec::new();
    for chunk in chunks.next().unwrap().lines() {
        let mut parts = chunk.split("|");
        rules.push((
            parts.next().unwrap().parse::<u32>().unwrap(),
            parts.next().unwrap().parse::<u32>().unwrap(),
        ));
    }
    let mut input: Vec<Vec<u32>> = vec![];

    for chunk in chunks.next().unwrap().lines() {
        input.push(
            chunk
                .split(',')
                .filter(|value| !value.is_empty())
                .map(|value| value.parse::<u32>().unwrap())
                .collect(),
        );
    }
    (rules, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (input, rules) = get_input();
        let res = part1(rules, input);

        assert_eq!(6505, res)
    }

    #[test]
    fn test_part2() {
        let (input, rules) = get_input();
        let res = part2(rules, input);

        assert_eq!(6897, res)
    }
}
