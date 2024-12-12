use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let blinks = args[1].parse().unwrap();
    println!("Result: {} for, {}", get(blinks), blinks);
}

fn get(blinks: usize) -> usize {
    let content = String::from("28591 78 0 3159881 4254 524155 598 1");
    let initial = content
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut stones: HashMap<usize, usize> = initial.into_iter().map(|x| (x, 1)).collect();
    for _ in 0..blinks {
        stones = update_freqs(stones);
    }
    stones.into_iter().fold(0, |acc, (_, frequ)| acc + frequ)
}

fn update_freqs(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones: HashMap<usize, usize> = HashMap::with_capacity(stones.capacity());

    for (stone, freq) in stones.into_iter() {
        if stone == 0 {
            new_stones
                .entry(1)
                .and_modify(|x| *x += freq)
                .or_insert(freq);
        } else if is_pair(stone) % 2 == 0 {
            let (p1, p2) = get_pair(stone);
            new_stones
                .entry(p1)
                .and_modify(|x| *x += freq)
                .or_insert(freq);
            new_stones
                .entry(p2)
                .and_modify(|x| *x += freq)
                .or_insert(freq);
        } else {
            new_stones
                .entry(stone * 2024)
                .and_modify(|x| *x += freq)
                .or_insert(freq);
        }
    }
    new_stones
}

fn is_pair(stone: usize) -> usize {
    let mut st = stone;
    let mut counter = 1;
    while st / 10 > 0 {
        st /= 10;
        counter += 1;
    }
    counter
}

fn get_pair(stone: usize) -> (usize, usize) {
    let ten: usize = 10;
    let div = ten.pow(is_pair(stone) as u32 / 2 as u32);
    let p1 = stone / div;
    let p2 = stone % div;
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_day11() {
        let res = get(25);

        assert_eq!(res, 220722)
    }
}
