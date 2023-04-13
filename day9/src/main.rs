use std::collections::HashSet;
use std::fs;

fn update_tail(node_pos: &mut (i32, i32), infront_pos: (i32, i32)) {
    let relative_pos = (infront_pos.0 - node_pos.0, infront_pos.1 - node_pos.1);
    
    if relative_pos.0 <= 1 && relative_pos.0 >= -1 && relative_pos.1 <= 1 && relative_pos.1 >= -1{
        // tail within range!
        return;
    }

    match relative_pos {
        (0, dist) => {
            if dist < 0 {
                node_pos.1 -= 1;
            }else{
                node_pos.1 += 1;
            }
        }
        (dist, 0) => {
            if dist < 0 {
                node_pos.0 -= 1;
            }else{
                node_pos.0 += 1;
            }
        }
        (x_dist, y_dist) => {
            if x_dist < 0{
                node_pos.0 -= 1;
            }else{
                node_pos.0 += 1;
            }

            if y_dist < 0{
                node_pos.1 -= 1;
            }else{
                node_pos.1 += 1;
            }
        }
    };
}

fn tail_coord_count(rope_len: u32, instructions: &str) -> usize{
    let mut seen_coords: HashSet<(i32, i32)> = HashSet::new();

    let mut rope: Vec<(i32, i32)> = Vec::new();
    for _ in 0..rope_len{
        rope.push((0,0));
    }
    seen_coords.insert(rope.last().unwrap().clone());

    for line in instructions.lines() {
        let (dir, steps) = line.split_once(" ").unwrap();
        let dir_vec = match dir {
            "L" => (-1, 0),
            "R" => (1, 0),
            "D" => (0, -1),
            "U" => (0, 1),
            _ => panic!("Dir: {} Not valid!", dir),
        };
        for _ in 0..steps.parse::<i32>().unwrap() {

            // update!
            rope[0] = (rope[0].0 + dir_vec.0, rope[0].1 + dir_vec.1);
            for i in 0..rope.len()-1{
                let front = rope[i].clone();
                let back = &mut rope[i+1];
                update_tail(back, front);

            }
            seen_coords.insert(rope.last().unwrap().clone());
        }
    }
    return seen_coords.len();
}

fn main() {
    let data = fs::read_to_string("day9.input").unwrap();
    
    println!("Length of 2 gives {} tail positions", tail_coord_count(2, &data));
    println!("Length of 10 gives {} tail positions", tail_coord_count(10, &data));
    /*for length in (2..500).step_by(1){
        tail_coord_count(length, &data);
        //println!("Length of {} gives {} tail positions", length, tail_coord_count(length, &data));
    }*/
}
