use std::fs;

struct Range{
    start: i32,
    end: i32
}

impl Range{
    fn contains(&self, r: &Range) -> bool{
        return self.start <= r.start && self.end >= r.end;
    }
    fn overlaps(&self, r: &Range) -> bool{
        return self.start <= r.end && self.end >= r.start;
    }
}

fn read_line(line: &str) -> (Range, Range){
    let (left, right) = line.split_once(',').unwrap();
    let (s, e) = left.split_once('-').unwrap();
    let left_range = Range{start: s.parse().unwrap(), end: e.parse().unwrap()};
    let (s, e) = right.split_once('-').unwrap();
    let right_range = Range{start: s.parse().unwrap(), end: e.parse().unwrap()};
    return (left_range, right_range);
}

fn read_data() -> Vec<(Range, Range)>{
    let data = fs::read_to_string("day4.input").unwrap();
    let mut ranges: Vec<(Range, Range)> = Vec::new();
    for line in data.lines(){
        ranges.push(read_line(line));
    }
    return ranges;
}


fn main() {
    let ranges = read_data();

    let mut fully_contained_count = 0;
    let mut overlap_count = 0;
    for (left, right) in ranges.iter(){
        if left.contains(&right) | right.contains(&left){
            fully_contained_count += 1;
        }
        if left.overlaps(&right){
            overlap_count += 1;
        }
    }
    println!("full overlap {}", fully_contained_count);
    println!("partial overlap {}", overlap_count);
}
