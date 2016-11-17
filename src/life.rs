#[derive(Debug)]
pub struct LifeBoard {
    pub x_size: usize,
    pub y_size: usize,
    board: Box<[bool]>
}

impl LifeBoard {
    pub fn new(x: usize, y: usize) -> LifeBoard {
        return LifeBoard {
            x_size: x,
            y_size: y,
            board: vec![false; x*y].into_boxed_slice()
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        return self.board[x + y*self.x_size];
    }

    pub fn set(&mut self, x: usize, y: usize, v: bool) {
        self.board[x + y*self.x_size] = v;
    }
}