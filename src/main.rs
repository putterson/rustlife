extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate time;

mod life;

use time::PreciseTime;
use time::Duration;
use life::LifeBoard;
use rand::Rng;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::env;
use std::collections::HashSet;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    speed : f64,
    staleness : f64,
    scale: f64,
    cells : LifeBoard
}

impl App {

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const DARKGREY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

        let size = 10.0 * self.scale;
        let mut pad = 2.0 * self.scale;
        if pad < 1.0 {
            pad = 0.0;
        }

        let ref cells = self.cells;

        let board_size_x = cells.x_size as u32;
        let board_size_y = cells.y_size as u32;
        
        let square = rectangle::square(0.0, 0.0, size);

        let (gx, gy) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);


        let board_x_pixels = f64::from(board_size_x)*(size+pad);
        let board_y_pixels = f64::from(board_size_y)*(size+pad);
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(gx, gy)
                .trans(-(board_x_pixels/2.0), -(board_y_pixels/2.0));

            // for x in 0..board_size_x {
            //     for y in 0..board_size_y {
            //         let xfloat : f64 = f64::from(x);
            //         let yfloat : f64 = f64::from(y);
            //         rectangle(DARKGREY, square, transform.trans(xfloat*(size+pad),yfloat*(size+pad)), gl);

            //     }
            // }

            for &(x,y) in &cells.active {
                let xfloat : f64 = f64::from(x as u32);
                let yfloat : f64 = f64::from(y as u32);
                rectangle(RED, square, transform.trans(xfloat*(size+pad),yfloat*(size+pad)), gl);
            }

        });
    }

    //Returns true if a true update was done
    fn update(&mut self, args: &UpdateArgs) -> bool {
        self.staleness += args.dt;
        if self.staleness >= self.speed {
            self.staleness = 0.0;
            let mut actions : Vec<(isize,isize,bool)> = vec![];
            {
                let ref cells = self.cells;

                let board_size_x = cells.x_size;
                let board_size_y = cells.y_size;
                // println!("Stale at {}", self.staleness);
                for &(x,y) in &cells.active {
                    let ix = x as isize;
                    let iy = y as isize;
                    update_cell((ix, iy), board_size_x, board_size_y, cells, &mut actions);
                }
            }
//            let mut actions : HashSet<(isize,isize,bool)> = HashSet::with_capacity(actions.len());
            for (x, y, state) in actions {
                self.cells.set(x, y, state);
            }

            return true;
        } else {
            return false;
        }
        
    }


}

    fn update_cell((x,y) : (isize, isize), board_size_x : usize, board_size_y : usize, cells : &LifeBoard, v : &mut Vec<(isize, isize, bool)>) {
        let coords = surrounding_cells(x,y);
        let current_cell = cells.get(x,y);
        let mut further_updates = vec![];

        {
            let surr = coords.iter().map(|&(x,y)| {return match cells.get(x,y) {
                false => {
                    if current_cell {further_updates.push((x,y).clone());}
                    return 0;
                },
                true => 1,
            }});
            let surr_sum : u8 = surr.sum();


            if surr_sum > 3 {
                v.push((x,y,false));
            } else if surr_sum < 2 {
                v.push((x,y,false));
            } else if surr_sum == 3 {
                v.push((x,y,true));
            }
        }

        if current_cell {
            for update in further_updates.iter() {
                update_cell(*update, board_size_x, board_size_y, cells, v);
            }
        }
    }

fn surrounding_cells(x : isize, y : isize) -> Vec<(isize, isize)>{
    let mut v : Vec<(isize,isize)> = vec![];
    for xs in [x-1, x, x+1].iter() {
        for ys in [y-1, y, y+1].iter() {
            if !((*xs == x) & (*ys == y)) {
                v.push((*xs,*ys));
            }
        }
    }
    return v;
} 

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut size = 100;
    if let Some(arg1) = env::args().nth(1) {
        if let Ok(i) = arg1.parse::<usize>() {
            size = i;
        }
        println!("The first argument is {}", arg1);
    }

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "life",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.

    let mut new_board = LifeBoard::new(size, size);
    let initial_scale = 0.5;

    for x in 0..new_board.x_size {
        for y in 0..new_board.y_size {
            //Should be calling distribution::Range according to docs
            let chance = rand::thread_rng().gen_range(1, 1001);
            let mut state = false;
            if chance > 999 {
                state = true;
                // new_board.set(x as isize,y as isize, state);
                let ix = x as isize;
                let iy = y as isize;
                new_board.set(1+ix,0+iy,true);
                new_board.set(0+ix,1+iy,true);
                new_board.set(2+ix,2+iy,true);
                new_board.set(1+ix,2+iy,true);
                new_board.set(0+ix,2+iy,true);
            }
        }
    }

    // new_board.set(1,0,true);
    // new_board.set(2,1,true);
    // new_board.set(0,2,true);
    // new_board.set(1,2,true);
    // new_board.set(2,2,true);

    // new_board.set(1+10,0+10,true);
    // new_board.set(0+10,1+10,true);
    // new_board.set(2+10,2+10,true);
    // new_board.set(1+10,2+10,true);
    // new_board.set(0+10,2+10,true);

    let mut app = App {
        gl: GlGraphics::new(opengl),
        speed: 0.05,
        staleness: 0.0,
        scale: initial_scale,
        cells: new_board
    };
    
    let mut update_count = 0;
    let mut update_acc = Duration::zero();

    let mut render_count = 0;
    let mut render_acc = Duration::zero();

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            let start = PreciseTime::now();
            app.render(&r);
            let end = PreciseTime::now();
            render_count += 1;
            render_acc = render_acc + start.to(end);
        }

        if let Some(u) = e.update_args() {
            let start = PreciseTime::now();
            if app.update(&u) {
                let end = PreciseTime::now();
                update_count += 1;
                update_acc = update_acc + start.to(end);
            }
        }

        if let Some(k) = e.press_args() {
            use piston_window::Button::Keyboard;
            use piston_window::Key;

            if k == Keyboard(Key::S) {
                app.scale = initial_scale;
            }
        }

        if let Some(s) = e.mouse_scroll_args() {
            app.scale += s[1] * (app.scale / 10.0);
            if app.scale < 0.0 {
                app.scale = 0.0;
            }
        }
    }

    println!("Average render time: {}", render_acc / render_count);
    println!("Average update time: {}", update_acc / update_count);
    println!("Goodbye");
}
