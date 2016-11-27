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
            active: HashSet::with_capacity(x*y/2)
        }
    }

    pub fn get(&self, x: isize, y: isize) -> bool {
        // return self.active.contains(&(x,y));
        let wrapped_x = wrap(x, self.x_size);
        let wrapped_y = wrap(y, self.y_size);
        return self.board[wrapped_x + wrapped_y*self.x_size];
        // return self.active.contains(&(wrapped_x, wrapped_y));
    }

    pub fn set(&mut self, x: isize, y: isize, v: bool) {
        let wrapped_x = wrap(x, self.x_size);
        let wrapped_y = wrap(y, self.y_size);
        if self.board[wrapped_x + wrapped_y * self.x_size] != v {
            if v {
                self.active.insert((wrapped_x, wrapped_y));
            } else {
                self.active.remove(&(wrapped_x, wrapped_y));
            }
            self.board[wrapped_x + wrapped_y*self.x_size] = v;
        }
    }
}

fn wrap(i : isize, max : usize) -> usize {
    ((i + max as isize) % max as isize) as usize
}