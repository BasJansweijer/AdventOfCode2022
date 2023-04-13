use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn char_to_val(c: char) -> i32 {
    let b = c.to_string().into_bytes()[0];
    let letter_val = match b {
        _ if b < 96 => b - 38,
        _ => b - 96,
    };
    i32::from(letter_val)
}

fn find_compartment_duplicate(line: &str) -> i32 {
    let mid: usize = (line.len() / 2).try_into().unwrap();
    let left = &line[0..mid];
    let right = &line[mid..line.len()];
    let mut left_used: HashMap<char, bool> = HashMap::new();
    let mut right_used: HashMap<char, bool> = HashMap::new();

    // loop through both sides and find duplicate char
    let mut i: usize = 0;
    let mut duplicate_char: char = '\0';
    while i < mid {
        let left_char: char = left.chars().nth(i).unwrap();
        let right_char: char = right.chars().nth(i).unwrap();
        left_used.insert(left_char, true);
        right_used.insert(right_char, true);

        match (left_used.get(&right_char), right_used.get(&left_char)) {
            (Option::Some(true), _) => {
                duplicate_char = right_char;
                break;
            }
            (_, Option::Some(true)) => {
                duplicate_char = left_char;
                break;
            }
            _ => {
                i += 1;
            }
        }
    }
    char_to_val(duplicate_char)
}

fn find_sets_duplicate(sets: Vec<&str>) -> i32 {
    let mut duplicates: HashSet<char> = HashSet::from_iter(sets[0].chars());
    for set in sets {
        let set: HashSet<char> = HashSet::from_iter(set.chars());
        duplicates.retain(|x| set.contains(x));
    }
    char_to_val(*duplicates.iter().nth(0).unwrap())
}

fn main() {
    let data = fs::read_to_string("input.input").unwrap();
    let lines = data.lines().collect::<Vec<&str>>();
    let lines_sets = lines.chunks(3);
    let mut total_sum = 0;
    for line in lines.iter() {
        total_sum += find_compartment_duplicate(line);
    }
    println!("{}", total_sum);
    total_sum = 0;
    for line in lines_sets {
        let c = find_sets_duplicate(line.to_vec());
        total_sum += c;
    }
    println!("{}", total_sum);
}
