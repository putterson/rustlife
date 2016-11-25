extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate time;
extern crate gfx;
extern crate gfx_graphics;

extern crate gfx_device_gl;

mod life;

use time::PreciseTime;
use time::Duration;
use life::LifeBoard;
use rand::Rng;
use opengl_graphics::GlGraphics;
use std::env;
use piston_window::*;
use glutin_window::GlutinWindow;
use gfx::format::{DepthStencil, Formatted, Srgba8};
use gfx::Typed;
// use gfx_graphics::{Flip, Gfx2d, Texture, TextureSettings};
use gfx_graphics::{Gfx2d};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    speed : f64,
    staleness : f64,
    scale: f64,
    cells : LifeBoard
}

impl App {
    //Returns true if a true update was done
    fn update(&mut self, args: &UpdateArgs) -> bool {
        self.staleness += args.dt;
        if self.staleness >= self.speed {
            self.staleness = 0.0;//self.staleness - self.speed;
            let mut actions : Vec<(isize,isize,bool)> = vec![];
            {
                let ref cells = self.cells;

                let board_size_x = cells.x_size;
                let board_size_y = cells.y_size;
                // println!("Stale at {}", self.staleness);
                for &(x,y) in &cells.active {
                    let ix = x as isize;
                    let iy = y as isize;
                    actions.extend(update_cell((ix, iy), board_size_x, board_size_y, cells));
                }
            }
            
            for (x, y, state) in actions {
                self.cells.set(x, y, state);
            }

            return true;
        } else {
            return false;
        }
        
    }


}

    fn update_cell((x,y) : (isize, isize), board_size_x : usize, board_size_y : usize, cells : &LifeBoard) -> Vec<(isize, isize, bool)> {
        let coords = surrounding_cells(x,y,board_size_x, board_size_y);
        let current_cell = cells.get(x,y);
        let mut further_updates = vec![];
        let mut v = vec![];
        {
            let surr = coords.iter().map(|&(x,y)| {return match cells.get(x,y) {
                false => {
                    further_updates.push((x,y).clone());
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
                v.extend(update_cell(*update, board_size_x, board_size_y, cells));
            }
        }

        return v;
    }

fn surrounding_cells(x : isize, y : isize, xmax : usize, ymax : usize) -> Vec<(isize, isize)>{
    let mut v : Vec<(isize,isize)> = vec![];
    for xs in [x-1, x, x+1].iter() {
        for ys in [y-1, y, y+1].iter() {
            //& (*xs <= x+1) & (*ys <= y+1)
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

    // Create an Piston window.
    let mut window: GlutinWindow = WindowSettings::new(
            "life",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();


    let (mut device, mut factory) = gfx_device_gl::create(|s|
        window.get_proc_address(s) as *const std::os::raw::c_void);

    // Create a new game and run it.

    let mut new_board = LifeBoard::new(size, size);
    let initial_scale = 0.5;
    
    let mut updates = 0;

    for x in 0..new_board.x_size {
        for y in 0..new_board.y_size {
            //Should be calling distribution::Range according to docs
            let chance = rand::thread_rng().gen_range(1, 1001);
            let mut state = false;
            if chance > 900 { 
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
        speed: 0.1,
        staleness: 0.0,
        scale: initial_scale,
        cells: new_board
    };
    
    let mut update_count = 0;
    let mut update_acc = Duration::zero();
    let mut update_max = 0.0;

    let mut render_count = 0;
    let mut render_acc = Duration::zero();
    let mut render_max = 0.0;

    let mut encoder = factory.create_command_buffer().into();
    let mut g2d = Gfx2d::new(opengl, &mut factory);
    let mut events = window.events();

    let samples = 4;


    let draw_size = window.draw_size();
    let aa = samples as gfx::tex::NumSamples;

    let color_format = <Srgba8 as Formatted>::get_format();
    let depth_format = <DepthStencil as Formatted>::get_format();
    


            let dim = (draw_size.width as u16, draw_size.height as u16, 1, aa.into());
            let (output_color, output_stencil) =
                gfx_device_gl::create_main_targets_raw(dim, color_format.0, depth_format.0);

    let output_color = Typed::new(output_color);
    let output_stencil = Typed::new(output_stencil);

    const WHITE:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const DARKGREY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

    let size = 10.0 * app.scale;
    let mut pad = 2.0 * app.scale;
    if pad < 1.0 {
        pad = 0.0;
    }

    // let ref cells = app.cells;

    let board_size_x = app.cells.x_size as u32;
    let board_size_y = app.cells.y_size as u32;
    
    let square = rectangle::square(0.0, 0.0, size);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            let start = PreciseTime::now();

                
            g2d.draw(&mut encoder, &output_color, &output_stencil, r.viewport(), |c, gl| {

                let (gx, gy) = ((r.width / 2) as f64,
                                (r.height / 2) as f64);
                // Clear the screen.
                clear(BLACK, gl);

                let board_x_pixels = f64::from(board_size_x)*(size+pad);
                let board_y_pixels = f64::from(board_size_y)*(size+pad);

                let transform = c.transform.trans(gx, gy)
                                        .trans(-(board_x_pixels/2.0), -(board_y_pixels/2.0));

                // for x in 0..board_size_x {
                //     for y in 0..board_size_y {
                //         let xfloat : f64 = f64::from(x);
                //         let yfloat : f64 = f64::from(y);
                //         rectangle(DARKGREY, square, transform.trans(xfloat*(size+pad),yfloat*(size+pad)), gl);

                //     }
                // }

                for &(x,y) in &app.cells.active {
                    let xfloat : f64 = f64::from(x as u32);
                    let yfloat : f64 = f64::from(y as u32);
                    rectangle(RED, square, transform.trans(xfloat*(size+pad),yfloat*(size+pad)), gl);
                }
            });

            encoder.flush(&mut device);

            let end = PreciseTime::now();
            render_count += 1;
            render_acc = render_acc + start.to(end);
        }

        if let Some(u) = e.update_args() {
            let start = PreciseTime::now();
            // whatever you want to do
            if app.update(&u) {
                let end = PreciseTime::now();
                update_count += 1;
                update_acc = update_acc + start.to(end);
            }
        }

        if let Some(k) = e.press_args() {
            use piston_window::Button::Keyboard;
            use piston_window::Key;

            match k {
                Keyboard(Key::S) => {
                    app.scale = initial_scale;
                    app.scale += 10.0;
                    println!("Changing scale");
                },
                Keyboard(Key::Plus) => {
                    app.scale += 1.0;
                },
                Keyboard(Key::Minus) => {
                    app.scale -= 1.0;
                },
                _ => {}
            }


        }

        if let Some(s) = e.mouse_scroll_args() {
            app.scale += s[1] * (app.scale / 10.0);
            if app.scale < 0.0 {
                app.scale = 0.0;
                println!("Scrolling scale");
            }
        }
    }

    println!("Average render time: {}", render_acc / render_count);
    println!("Average update time: {}", update_acc / update_count);
    println!("Goodbye");
}
