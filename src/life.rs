use std::collections::HashSet;

#[derive(Debug)]
pub struct LifeBoard {
    pub x_size: usize,
    pub y_size: usize,
    board: Box<[bool]>,
    pub active: HashSet<(usize, usize)>
}

impl LifeBoard {
    pub fn new(x: usize, y: usize) -> LifeBoard {
        return LifeBoard {
            x_size: x,
            y_size: y,
            board: vec![false; x*y].into_boxed_slice(),
            active: HashSet::new()
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        // return self.active.contains(&(x,y));
        return self.board[x + y*self.x_size];
    }

    pub fn set(&mut self, x: usize, y: usize, v: bool) {
        if v {
            self.active.insert((x,y));
        } else {
            self.active.remove(&(x,y));
        }
        self.board[x + y*self.x_size] = v;
    }
}