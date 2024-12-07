fn main() {
    let input = get_input();
    let res = part1(input.0.clone(), input.1.clone());
    println!("part 1{:?}", res);
}

fn get_input() -> (Vec<u32>, Vec<Vec<u32>>) {
    let content = include_str!("../../inputs/exemple");
    let mut values: Vec<u32> = Vec::new();
    let mut ops: Vec<Vec<u32>> = Vec::new();
    content.lines().for_each(|line| {
        let mut splits = line.split(":");
        values.push(splits.next().unwrap().parse::<u32>().unwrap());
        ops.push(
            splits
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        )
    });
    (values, ops)
}

fn part1(values: Vec<u32>, ops: Vec<Vec<u32>>) -> u32 {
    values.into_iter().zip(ops).fold(0, |acc, (value, op)| {
        acc + get_result(value, op.into_iter(), 0)
    })
}

fn get_result<T: &mut Iterator<Item = u32>>(v: u32, op: T, result: u32) -> u32 {
    if !op.peekable().peek().is_some() {
        return result;
    }
    let operand = op.next().unwrap();
    let res1 = get_result(v, op, result + operand);
    if res1 == v {
        return v;
    }

    let res2 = get_result(v, op, result + operand);
    if res2 == v {
        return v;
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    fn it_part1() {
        let (values, ops) = get_input();
        let res = part1(values.clone(), ops.clone());

        assert_eq!(res, 3749)
    }
}
