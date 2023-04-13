use std::fs;

fn max(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}

fn try_insert(max_vals: &mut Vec<i32>, new_val: i32) {
    let mut i = 0;
    while new_val < max_vals[i] {
        i += 1;
        if i >= max_vals.len() {
            return;
        }
    }
    max_vals.insert(i, new_val);
    max_vals.remove(max_vals.len() - 1);
}

fn main() {
    let data = fs::read_to_string("files/in.input").expect("No in.input?");
    let lines = data.lines().collect::<Vec<&str>>();
    //let mut elves = vec!([]);
    let mut cur_count = 0;
    let mut max_count = vec![0, 0, 0];
    for line in lines {
        if line.len() == 0 {
            //elves.push(cur_count);
            try_insert(&mut max_count, cur_count);
            cur_count = 0;
        } else {
            cur_count += line.parse::<i32>().unwrap();
        }
    }
    println!("{:#?}", max_count.iter().sum::<i32>());
}
