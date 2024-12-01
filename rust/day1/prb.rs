use std::{collections::HashMap, time::Instant};

#[allow(unused_assignments)]

fn main() {
    let input = get_input();
    let repeat = 1000;

    let mut min = 10000;
    let mut tmp = 0;
    for _ in 0..repeat {
        tmp = part2_fold_exp(input.clone());
        if tmp < min {
            min = tmp;
        }
    }
    println!("exp: {}", min);

    tmp = 0;
    min = 10000;

    for _ in 0..repeat {
        tmp = part2_fold(input.clone());
        if tmp < min {
            min = tmp;
        }
    }
    println!("fold: {}", min);

    tmp = 0;
    min = 10000;
    for _ in 0..repeat {
        tmp = part2_sum(input.clone());
        if tmp < min {
            min = tmp;
        }
    }
    println!("sum: {}", min);
}

fn part2_fold_exp(input: (Vec<i32>, Vec<i32>)) -> u128 {
    let n = input.0.len();
    let start = Instant::now();

    let mut freqs = HashMap::with_capacity(n);
    for n in input.1.iter() {
        *freqs.entry(n).or_insert(0) += 1;
    }
    input
        .0
        .into_iter()
        .fold(0, |acc, v| acc + v * freqs.get(&v).unwrap_or(&0));

    start.elapsed().as_micros()
}

fn part2_fold(input: (Vec<i32>, Vec<i32>)) -> u128 {
    let start = Instant::now();
    let freqs: HashMap<i32, i32> = input.1.into_iter().fold(HashMap::new(), |mut h, n| {
        let entry = h.entry(n).or_default();
        *entry += 1;
        h
    });

    input
        .0
        .into_iter()
        .fold(0i32, |s, n| s + n * *freqs.get(&n).unwrap_or(&0));

    start.elapsed().as_micros()
}

fn part2_sum(input: (Vec<i32>, Vec<i32>)) -> u128 {
    let n = input.0.len();
    let start = Instant::now();
    let mut counter = HashMap::with_capacity(n);
    for n in input.1.iter() {
        *counter.entry(n).or_insert(0) += 1;
    }

    input
        .1
        .iter()
        .map(|n| n * counter.get(n).unwrap_or(&0))
        .sum::<i32>() as usize;
    start.elapsed().as_micros()
}

fn get_input() -> (Vec<i32>, Vec<i32>) {
    let content = include_str!("./input");
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
