use std::fs;

fn main() {
    let data = fs::read_to_string("day10.input").unwrap();
    let mut reg_x = 1;
    let mut sum = 0;
    let mut commands = data.lines();
    let mut cycles_left = 0;
    let mut cur_cmd : &str;
    let mut last_cmd_effect = 0;

    for cycle in 1.. {
        if cycles_left == 0 {
            reg_x += last_cmd_effect;
            last_cmd_effect = 0;

            cur_cmd = match commands.next(){
                Some(cmd) => cmd,
                None => break,
            };
            if cur_cmd == "noop" {
                cycles_left = 1;
            } else {
                cycles_left = 2;
                let (_, amount) = cur_cmd.split_once(" ").unwrap();
                last_cmd_effect = amount.parse::<i32>().unwrap();
            }
        }

        let pixel_index = (cycle - 1) % 40;
        if pixel_index == 0{
            println!("");
        }

        if (pixel_index-reg_x).abs() <= 1{
            print!("#");
        }else{
            print!(".");
        }

        if (cycle - 20) % 40 == 0 {
            sum += cycle * reg_x;
        }
        cycles_left -= 1;
    }
    println!("\n{}", sum);
}
