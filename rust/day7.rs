fn main() {
    let (values, ops) = get_input();
    println!("{:?} {:?}", values, ops);
}

fn get_input() -> (Vec<u32>, Vec<Vec<u32>>) {
    let content = include_str!("../inputs/exemple");
    let mut values: Vec<u32> = vec![];
    let mut ops: Vec<Vec<u32>> = vec![];
    content.lines().for_each(|line| {
        let mut splits = line.split(":");
        values.push(splits.next().unwrap().parse::<u32>().unwrap());
        ops.push(
            splits
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|item| item.parse::<u32>().unwrap())
                .collect(),
        );
    });

    (values, ops)
}
