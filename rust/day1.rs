use std::collections::HashMap;

#[allow(unused_assignments)]

fn main() {
    let input = get_input();
    println!("Part1: {}", part1(input.clone()));
    println!("Part2: {}", part2(input.clone()));
}

fn part1(input: (Vec<i32>, Vec<i32>)) -> i32 {
    input
        .0
        .iter()
        .zip(input.1)
        .fold(0, |x, (v1, v2)| x + (v1 - v2).abs())
}

fn part2(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let n = input.0.len();

    let mut freqs = HashMap::with_capacity(n);
    for n in input.1.iter() {
        freqs.entry(n).and_modify(|x| *x += 1).or_insert(1);
    }
    input
        .0
        .into_iter()
        .fold(0, |acc, v| acc + v * freqs.get(&v).unwrap_or(&0))
}

fn get_input() -> (Vec<i32>, Vec<i32>) {
    let content = include_str!("../../inputs/d1");
    let mut array1: Vec<i32> = vec![];
    let mut array2: Vec<i32> = vec![];

    content.lines().for_each(|line| {
        let mut split = line.split_ascii_whitespace();

        array1.push(split.next().unwrap().parse::<i32>().unwrap());
        array2.push(split.next().unwrap().parse::<i32>().unwrap());
    });
    array1.sort();
    array2.sort();
    (array1, array2)
}
