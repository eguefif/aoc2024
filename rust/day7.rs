fn main() {
    let (values, ops) = get_input();
    println!("part1: {}", part1(values.clone(), ops.clone()));
    println!("part2: {}", part2(values.clone(), ops.clone()));
}

fn part1(values: Vec<u128>, ops: Vec<Vec<u128>>) -> u128 {
    values.iter().zip(ops).fold(0, |acc, (value, operands)| {
        acc + calculate(&value, operands.clone(), operands[0], 1)
    })
}

fn calculate(value: &u128, operands: Vec<u128>, result: u128, index: usize) -> u128 {
    if index >= operands.len() {
        return result;
    }
    let res1 = calculate(value, operands.clone(), result + operands[index], index + 1);
    if res1 == *value {
        return *value;
    }

    let res2 = calculate(value, operands.clone(), result * operands[index], index + 1);
    if res2 == *value {
        return *value;
    }
    0
}

fn part2(values: Vec<u128>, ops: Vec<Vec<u128>>) -> u128 {
    values.iter().zip(ops).fold(0, |acc, (value, operands)| {
        acc + calculate2(&value, operands.clone(), operands[0], 1)
    })
}

fn combine(v1: u128, v2: u128) -> u128 {
    let tmp = format!("{}{}", v1, v2);
    tmp.parse::<u128>().unwrap()
}

fn calculate2(value: &u128, operands: Vec<u128>, result: u128, index: usize) -> u128 {
    if index >= operands.len() {
        return result;
    }
    let res1 = calculate2(value, operands.clone(), result + operands[index], index + 1);
    if res1 == *value {
        return *value;
    }

    let res2 = calculate2(value, operands.clone(), result * operands[index], index + 1);
    if res2 == *value {
        return *value;
    }

    let res3 = calculate2(
        value,
        operands.clone(),
        combine(result, operands[index]),
        index + 1,
    );
    if res3 == *value {
        return *value;
    }
    0
}

fn get_input() -> (Vec<u128>, Vec<Vec<u128>>) {
    let content = include_str!("../inputs/d7");
    let mut values: Vec<u128> = vec![];
    let mut ops: Vec<Vec<u128>> = vec![];
    content.lines().for_each(|line| {
        let mut splits = line.split(":");
        values.push(splits.next().unwrap().parse::<u128>().unwrap());
        ops.push(
            splits
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|item| item.parse::<u128>().unwrap())
                .collect(),
        );
    });

    (values, ops)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_part1() {
        let (values, ops) = get_input();
        let res = part1(values.clone(), ops.clone());

        assert_eq!(res, 1289579105366)
    }

    #[test]
    fn it_part2() {
        let (values, ops) = get_input();
        let res = part2(values.clone(), ops.clone());

        assert_eq!(res, 92148721834692)
    }

    #[test]
    fn it_combine() {
        let res = combine(12, 28);

        assert_eq!(res, 1228)
    }
}
