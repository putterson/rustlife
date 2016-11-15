extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use rand::Rng;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    speed : f64,
    staleness : f64,
    cells : Vec<Vec<bool>>
}

impl App {

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const DARKGREY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

        let size = 10.0;
        let pad = 2.0;
        let board_size = (size+pad)*120.0; //This assumes square board
        let square = rectangle::square(0.0, 0.0, size);

        let ref mut cells = self.cells;

        // let rotation = self.rotation;
        let (gx, gy) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);



            let transform = c.transform.trans(gx, gy)
                                       .trans(-(board_size/2.0), -(board_size/2.0));

            for (row, x) in cells.iter().zip(0..) {
                for (column, y) in row.iter().zip(0..) {
                    let xfloat : f64 = f64::from(x);
                    let yfloat : f64 = f64::from(y);
                    let mut colour = RED;
                    if *column == false {
                        colour = DARKGREY;
                    }
                    rectangle(colour, square, transform.trans(xfloat*(size+pad),yfloat*(size+pad)), gl);

                }
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.staleness += args.dt;
        if self.staleness >= self.speed {
            self.staleness = self.staleness - self.speed;
            let mut actions : Vec<(usize,usize,bool)> = vec![];
            {
                let ref cells = self.cells;
                // println!("Stale at {}", self.staleness);
                for (row, x) in cells.iter().zip(0..) {
                    for (column, y) in row.iter().zip(0..) {
                        let coords = surrounding_cells(x,y,cells.len(), row.len());
                        let surr = coords.iter().map(|&(x,y)| {return match cells[x][y] {
                            false => 0,
                            true => 1,
                        }});
                        let surr_sum : u8 = surr.sum();

                        if surr_sum > 3 {
                            actions.push((x,y,false))
                        } else if surr_sum < 2 {
                            actions.push((x,y,false))
                        } else if surr_sum == 3 {
                            actions.push((x,y,true))
                        }
                        
                    }
                }
            }
            
            for (x, y, state) in actions {
                self.cells[x][y] = state;
            }
        }
        
    }


}

fn surrounding_cells(x : usize, y : usize, xmax : usize, ymax : usize) -> Vec<(usize, usize)>{
    let mut v : Vec<(usize,usize)> = vec![];
    for xs in [x.wrapping_sub(1), x, x+1].iter() {
        for ys in [y.wrapping_sub(1), y, y+1].iter() {
            if (*xs < xmax) & (*ys < ymax) & !((*xs == x) & (*ys == y)) & (*xs <= x+1) & (*ys <= y+1){
                v.push((*xs,*ys));
            }
        }
    }
    return v;
} 

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

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

    let size = 120;

    let mut initial_board : Vec<Vec<bool>> = vec![];

    for x in 0..size {
        let mut column : Vec<bool> = vec![];
        for y in 0..size {
            let chance = rand::thread_rng().gen_range(1, 101);
            let mut state = false;
            if chance > 80 { state = true; }
            column.push(state);
        }
        initial_board.push(column);
    }

    let mut app = App {
        gl: GlGraphics::new(opengl),
        speed: 0.1,
        staleness: 0.0,
        cells: initial_board
    };
    
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
