use std::fs;
use std::collections::HashSet;

fn first_n_distinct(data: &str, n: usize) -> Option<usize>{
    for i in 0..data.len()-1-n{
        let cur_slice = &data[i..i+n];
        let unique: HashSet<char> = HashSet::from_iter(cur_slice.chars());
        if unique.len() == n{
            return Some(i+n);
        }
    }
    return None
}

fn main() {
    let data: String = fs::read_to_string("day6.input").unwrap();
    println!("first 4: {}", first_n_distinct(&data, 4).unwrap());
    println!("first 14: {}", first_n_distinct(&data, 14).unwrap());
}
