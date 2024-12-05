use std::collections::HashMap;

fn main() {
    //let (input, rules) = get_input();
    let (rules, input) = get_input();

    println!("{:?}\n{:?}", rules, input);
    //println!("Part1: {}", part1(input.clone(), rules.clone()));
    //println!("Part2: {}", part2(input.clone(), rules.clone()));
}

fn part1(input: Vec<Vec<u32>>, rules: HashMap<u32, u32>) -> u32 {
    0
}

fn get_input() -> (HashMap<u32, u32>, Vec<Vec<u32>>) {
    let content = include_str!("../../inputs/exemple");
    let chunks: Vec<&str> = content.split("\n\n").collect::<Vec<&str>>();

    let mut rules = HashMap::new();
    let chunk_rules = chunks[0].split('\n').collect::<Vec<&str>>();
    for chunk in chunk_rules {
        let r: Vec<&str> = chunk.split("|").collect::<Vec<&str>>();
        rules.insert(r[0].parse::<u32>().unwrap(), r[1].parse::<u32>().unwrap());
    }
    let mut input: Vec<Vec<u32>> = vec![];

    let chunk_input = chunks[1].split('\n').collect::<Vec<&str>>();
    for chunk in chunk_input {
        let row = chunk
            .split(',')
            .filter(|value| !value.is_empty())
            .map(|value| value.parse::<u32>().unwrap())
            .collect();
        input.push(row);
    }
    (
        rules,
        input
            .into_iter()
            .filter(|value| !value.is_empty())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_part1() {
        let (input, part1) = get_input();
        let res = part1(input, part1);
    }
}
