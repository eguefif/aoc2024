use std::collections::HashMap;

fn main() {
    let arr = [1, 1, 3, 4, 5, 5, 3, 3, 3];
    let f: HashMap<i32, i32> = arr.iter().fold(HashMap::new(), |mut h, v| {
        let entry = h.entry(*v).or_default();
        *entry += 1;
        h
    });
    println!("{:?}", f);
}
