extern crate rand;
extern crate graphics;
extern crate gfx_graphics;
extern crate gfx;
extern crate gfx_device_gl;
extern crate input;
extern crate window;
extern crate glutin_window;

mod chip8;
mod utils;

use gfx::traits::*;
use gfx::memory::Typed;
use gfx::format::{DepthStencil, Formatted, Srgba8};
use input::{Button, Input};
use window::{OpenGLWindow, Window, WindowSettings, Size};
use glutin_window::{GlutinWindow, OpenGL};
use gfx_graphics::{Gfx2d};
use graphics::*;
use std::process;
use chip8::*;

static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let mut chip8 = Chip8::new();

    if let Some(filename) = std::env::args().nth(1) {
        chip8.load(&filename);
    } else {
        println!("File not found!");
        process::exit(1);
    }

    let opengl = OpenGL::V4_4;
    let size = Size { width: 640, height: 320 };
    let samples = 0;

    let mut window: GlutinWindow =
        WindowSettings::new("CHIP-8", size)
        .opengl(opengl)
        .samples(samples)
        .resizable(false)
        .exit_on_esc(true)
        .build().unwrap();
    
    let draw_size = window.draw_size();

    let (mut device, mut factory) =
        gfx_device_gl::create(|s| window.get_proc_address(s) as *const _);
    
    let (output_color, output_stencil) = {
        let aa = samples as gfx::texture::NumSamples;
        let dim = (draw_size.width as u16, draw_size.height as u16, 1, aa.into());
        let color_format = <Srgba8 as Formatted>::get_format();
        let depth_format = <DepthStencil as Formatted>::get_format();
        gfx_device_gl::create_main_targets_raw(dim, color_format.0, depth_format.0)
    };
    let output_color = Typed::new(output_color);
    let output_stencil = Typed::new(output_stencil);

    let mut encoder = factory.create_command_buffer().into();
    let mut g2d = Gfx2d::new(opengl, &mut factory);

    let viewport =
        Viewport {
            rect: [0, 0, draw_size.width as i32, draw_size.height as i32],
            window_size: [size.width, size.height],
            draw_size: [draw_size.width, draw_size.height],
        };

    loop {
        if window.should_close() { process::exit(0); }
        while let Some(e) = window.poll_event() {
            match e {
                Input::Press(Button::Keyboard(key)) => chip8.handle_button_press(key),
                Input::Release(Button::Keyboard(key)) => chip8.handle_button_release(key),
                Input::Close => process::exit(0),
                _ => continue,
            }
        }
        chip8.emulate_cycle();
        if chip8.draw_flag {
            g2d.draw(&mut encoder, &output_color, &output_stencil, viewport, |c, g| {
                clear(BLACK, g);
                for y in 0..32 {
                    for x in 0..64 {
                        if chip8.gfx[x + y * 64] == 1 {
                            rectangle(
                                WHITE, 
                                rectangle::square(x as f64 * 10.0, y as f64 * 10.0, 10.0),
                                c.transform,
                                g);
                        }
                    }
                }
            });

            encoder.flush(&mut device);
            Window::swap_buffers(&mut window);
            device.cleanup();
            chip8.draw_flag = false;
        }
    }
}