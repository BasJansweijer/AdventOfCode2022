use std::fs;

#[derive(Debug)]
struct Pile<T> {
    contents: Vec<T>,
}

impl<T> Pile<T> {
    fn pop(&mut self) -> Option<T> {
        self.contents.pop()
    }

    fn insert_bottom(&mut self, c: T) {
        self.contents.insert(0, c);
    }

    fn push(&mut self, c: T) {
        self.contents.push(c);
    }
}

// returns initialized start piles and the input lines for the instructions
fn parse_input <'a> (data: &'a String) -> (Vec<Pile<char>>, Vec<&'a str>) {
   
    let (start_piles, instructions) = data.split_once("\n\n").unwrap();

    // initialize our piles
    let mut start_lines = start_piles.lines().collect::<Vec<&str>>();
    let pile_nums = start_lines
        .pop()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();
    let mut piles: Vec<Pile<char>> = Vec::new();
    for _ in pile_nums {
        piles.push(Pile {
            contents: Vec::new(),
        });
    }

    let mut pile_char_col = 1;
    for pile in piles.iter_mut() {
        start_lines
            .iter()
            .map(|s| s.chars().nth(pile_char_col))
            .for_each(|res| match res {
                Some(c) => {if c != ' '{pile.insert_bottom(c)}},
                None => (),
            });
        pile_char_col += 4;
    }
    return (piles, instructions.lines().collect::<Vec<&str>>());
}

fn enact_instruction(line: &str,  piles: &mut Vec<Pile<char>>){
    let mut nums: Vec<i32> = Vec::new();
    line.split_whitespace().for_each(|x| match x.parse::<i32>(){
        Ok(num) => nums.push(num),
        Err(_) => ()
    });
    //nums contains n, source, dest
    let source: usize = (nums[1]-1).try_into().unwrap();
    let dest: usize = (nums[2]-1).try_into().unwrap();
    for _ in 0..nums[0] {
        let c = piles[source].pop().unwrap();
        piles[dest].push(c);
    }
}

fn enact_instruction_multi_pickup(line: &str,  piles: &mut Vec<Pile<char>>){
    let mut nums: Vec<i32> = Vec::new();
    line.split_whitespace().for_each(|x| match x.parse::<i32>(){
        Ok(num) => nums.push(num),
        Err(_) => ()
    });
    //nums contains n, source, dest
    let source: usize = (nums[1]-1).try_into().unwrap();
    let dest: usize = (nums[2]-1).try_into().unwrap();

    let mut temp_stack: Vec<char> = Vec::new();
    for _ in 0..nums[0] {
        temp_stack.push(piles[source].pop().unwrap())
    }
    for _ in 0..nums[0] {
        piles[dest].push(temp_stack.pop().unwrap())
    }

}

fn main() {
    let data = fs::read_to_string("day5.input").unwrap();
    let (mut piles, moves) = parse_input(&data);
    for instruction in moves{
        //enact_instruction(instruction, &mut piles);
        enact_instruction_multi_pickup(instruction, &mut piles);
    }

    for pile in piles.iter_mut(){
        println!("{}", pile.pop().unwrap());
    }
}
