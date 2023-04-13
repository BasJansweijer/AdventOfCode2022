use std::fs;

#[derive(Clone, Copy)]
enum Move{
    Rock,
    Paper,
    Scisors
}

impl Move {
    fn value(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scisors => 3
        }
    }
    fn from(c: char) -> Move{
        match c{
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scisors,
            _ => {println!("err"); Move::Rock}
        }
    } 
}

fn main() {
    let data = fs::read_to_string("input.input").expect("expected to be able to read file!");
    let turns: Vec<&str> = data.lines().collect();
    let mut total_score = 0;
    for turn in turns.iter() {
        let (opp_move, our_move) = turn.split_once(' ').unwrap();
        let opp_move: Move = Move::from(opp_move.chars().nth(0).unwrap());
        
        let our_move = match our_move{
            "X" => match &opp_move{
                Move::Rock => Move::Scisors,
                Move::Paper => Move::Rock,
                Move::Scisors => Move::Paper,
            },
            "Y" => opp_move,
            "Z" => match &opp_move{
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scisors,
                Move::Scisors => Move::Rock,
            }
            _ => {println!("err"); Move::Rock}
        };
        
        let mut turn_score = our_move.value();
        turn_score += match (opp_move, our_move) {
            (Move::Rock, Move::Paper) | (Move::Scisors, Move::Rock) | (Move::Paper, Move::Scisors) => 6,
            (Move::Rock, Move::Scisors) | (Move::Paper, Move::Rock) | (Move::Scisors, Move::Paper) => 0,
            (_, _) => 3
        };
        total_score += turn_score;
    }
    println!("{}", total_score);
}
