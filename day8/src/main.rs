use std::fs;

struct Forest {
    //(height, width)
    shape: (usize, usize),
    data: Vec<u32>,
}

// performs split at and removes the value at the index from either return slice
fn break_at<T>(v: &[T], index: usize) -> (&[T], &[T]) {
    let (l, r) = v.split_at(index);
    let r = &r[1..];
    return (l, r);
}

impl Forest {
    fn get_pos(&self, x: usize, y: usize) -> Option<u32> {
        if y >= self.shape.0 || x >= self.shape.1 {
            //Out of bounds!
            return None;
        }
        let index = y * self.shape.0 + x;
        Some(self.data[index])
    }

    // returns all the directions in which x,y could see/be seen.
    // (col_up, col_down, row_left, row_right)
    // always orderd so each side has the firs elem closest to x,y.
    fn get_visibility_lines (&self, x: usize, y: usize) -> [Vec<u32>; 4]
    {
        let row = &self.data[y * self.shape.0..y * self.shape.0 + self.shape.1];
        let (row_left, row_right) = break_at(row, x);
        let row_left = row_left.iter().cloned().rev().collect::<Vec<u32>>();
        let row_right = row_right.to_vec();

        let col: Vec<u32> = self.data.chunks(self.shape.1).map(|row| row[x]).collect();
        let (col_up, col_down) = break_at(&col, y);
        let col_up = col_up.iter().cloned().rev().collect::<Vec<u32>>();
        let col_down = col_down.to_vec();
        return [col_up, col_down, row_left, row_right];
    }

    fn tree_visible(&self, x: usize, y: usize) -> bool {
        let height = self.get_pos(x, y).unwrap();

        
        for side in self.get_visibility_lines(x,y) {
            if side.iter().filter(|&&h| h >= height).count() == 0 {
                return true;
            }
        }
        false
    }

    fn pos_from_index(&self, i: usize) -> (usize, usize) {
        (i % self.shape.0, i / self.shape.1)
    }

    fn count_visible_trees(&self) -> i32 {
        let mut vis_count = 0;
        for i in 0..self.data.len() {
            let (x, y) = self.pos_from_index(i);
            if self.tree_visible(x, y) {
                vis_count += 1;
            }
        }
        vis_count
    }

    fn scenic_score(&self, x: usize, y: usize) -> i32{
        let mut score = 1;
        let own_h = self.get_pos(x,y).unwrap();
        for side in self.get_visibility_lines(x,y){
            let mut side_iter = side.iter();
            let mut side_count = 0;
            while let Some(h) = side_iter.next(){
                side_count += 1;
                if *h >= own_h{
                    break;
                }
            }
            score *= side_count
        }
        score
    }

    fn best_scenic_score(&self) -> i32{
        let mut best = 0;
        self.data.iter().enumerate().for_each(|(i, &x)| {
            let (x,y) = self.pos_from_index(i);
            let s = self.scenic_score(x,y);
            if s > best{
                best = s
            }
        });
        best
    }
}

fn get_forest(data: &str) -> Forest {
    let mut rows: Vec<&str> = data.lines().collect();
    let height = rows.len();
    let width = rows[0].len();
    let mut forest_data = Vec::new();
    rows.iter_mut().for_each(|row| {
        row.chars()
            .for_each(|c| forest_data.push(c.to_digit(10).unwrap()))
    });

    return Forest {
        shape: (height, width),
        data: forest_data,
    };
}

fn main() {
    let data = fs::read_to_string("day8.input").unwrap();
    let forest = get_forest(&data);
    println!("{}", forest.count_visible_trees());
    println!("{}", forest.best_scenic_score());
}
