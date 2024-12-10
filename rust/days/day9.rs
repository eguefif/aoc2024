use std::time::Instant;

#[derive(Debug)]
struct Index {
    pub size: usize,
    pub index: usize,
}
fn main() {
    let input = get_input();
    let start = Instant::now();
    let res = part2(input);
    println!("Timing: {}", start.elapsed().as_millis());
    println!("part2: {}", res);
}

fn part2(input: Vec<u8>) -> usize {
    let mut disk = get_disk(&input);
    let files = get_files_index(&input);
    remap(&mut disk, files);
    calculate(disk)
}

fn remap(disk: &mut Vec<i64>, files: Vec<Index>) {
    let mut spaces = get_spaces(&disk);

    for file in files.into_iter().rev() {
        if let Some(value) = look_for_space(&file, &spaces) {
            copy(value, disk, &file);
            spaces = get_spaces(disk);
        }
    }
}

fn copy(idx: usize, disk: &mut [i64], file: &Index) {
    let idx2 = file.index;
    for i in 0..file.size {
        disk[idx + i] = disk[idx2 + i];
        disk[idx2 + i] = -1;
    }
}

fn look_for_space(file: &Index, spaces: &[Index]) -> Option<usize> {
    for space in spaces {
        if space.index >= file.index {
            return None;
        }
        if space.size >= file.size {
            return Some(space.index);
        }
    }
    None
}

fn get_spaces(disk: &[i64]) -> Vec<Index> {
    let mut retval: Vec<Index> = vec![];

    let mut i: usize = disk[1] as usize;
    while i < disk.len() {
        if disk[i] == -1 {
            let size = get_size(disk, i);
            retval.push(Index { size, index: i });
            i += size;
        } else {
            i += 1;
        }
    }
    retval
}

fn get_size(disk: &[i64], i: usize) -> usize {
    let mut counter: usize = 0;
    while (i + counter) < disk.len() && disk[i + counter] == -1 {
        counter += 1;
    }
    counter
}

fn get_files_index(index: &Vec<u8>) -> Vec<Index> {
    let mut retval: Vec<Index> = vec![];

    let mut idx = 0;
    let mut i = 0;
    while i < index.len() {
        let file = Index {
            size: index[i] as usize,
            index: idx,
        };
        retval.push(file);
        if i + 1 < index.len() {
            idx += index[i] as usize + index[i + 1] as usize;
            i += 2;
        } else {
            idx += index[i] as usize;
            i += 1;
        }
    }

    retval
}

fn calculate(disk: Vec<i64>) -> usize {
    let mut acc = 0;
    for (i, v) in disk.iter().enumerate() {
        if *v != -1 {
            acc += i * *v as usize
        }
    }
    acc
}

fn get_disk(input: &[u8]) -> Vec<i64> {
    let mut retval: Vec<i64> = vec![];
    let mut idx = 0;
    for (i, v) in input.iter().enumerate() {
        if i % 2 == 0 {
            let slice = vec![idx; *v as usize];
            retval.extend_from_slice(&slice);
            idx += 1;
        } else {
            let slice = vec![-1; *v as usize];
            retval.extend_from_slice(&slice);
        }
    }
    retval
}

fn get_input() -> Vec<u8> {
    let content = include_str!("../../inputs/d9");
    String::from(content)
        .strip_suffix("\n")
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_d9p2() {
        let input = get_input();
        let res = part2(input);
        assert_eq!(res, 6237075041489)
    }
}
