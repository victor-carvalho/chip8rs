extern crate rand;
extern crate sdl2_window;
extern crate piston_window;

mod chip8;
mod utils;

use chip8::*;
use piston_window::*;
use sdl2_window::Sdl2Window;

static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let mut chip8 = Chip8::new();

    if let Some(filename) = std::env::args().nth(1) {
        chip8.load(&filename);
    } else {
        panic!("File not found!");
    }

    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("CHIP-8", (640, 320))
        .opengl(OpenGL::V4_4)
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_ups(300);
    window.set_max_fps(300);
    window.set_swap_buffers(false);

    while let Some(e) = window.next() {
        match e {
            Event::Update(_) => chip8.emulate_cycle(),
            Event::Render(_) => {
                if chip8.draw_flag {
                    window.draw_2d(&e, |c, g| {
                        clear(BLACK, g);
                        for y in 0..32 {
                            for x in 0..64 {
                                if chip8.gfx[x + y * 64] == 1 {
                                    rectangle(WHITE, rectangle::square(x as f64 * 10.0, y as f64 * 10.0, 10.0), c.transform, g);
                                }
                            }
                        }
                        chip8.draw_flag = false;
                    });
                    window.window.swap_buffers();
                }
            },
            Event::Input(Input::Press(Button::Keyboard(key))) => match key {
                Key::D1 => chip8.key[0x1] = 1,
                Key::D2 => chip8.key[0x2] = 1,
                Key::D3 => chip8.key[0x3] = 1,
                Key::D4 => chip8.key[0xC] = 1,
                Key::Q => chip8.key[0x4] = 1,
                Key::W => chip8.key[0x5] = 1,
                Key::E => chip8.key[0x6] = 1,
                Key::R => chip8.key[0xD] = 1,
                Key::A => chip8.key[0x7] = 1,
                Key::S => chip8.key[0x8] = 1,
                Key::D => chip8.key[0x9] = 1,
                Key::F => chip8.key[0xE] = 1,
                Key::Z => chip8.key[0xA] = 1,
                Key::X => chip8.key[0x0] = 1,
                Key::C => chip8.key[0xB] = 1,
                Key::V => chip8.key[0xF] = 1,
                _ => continue,
            },
            Event::Input(Input::Release(Button::Keyboard(key))) => match key {
                Key::Space => chip8.debug = !chip8.debug,
                Key::D1 => chip8.key[0x1] = 0,
                Key::D2 => chip8.key[0x2] = 0,
                Key::D3 => chip8.key[0x3] = 0,
                Key::D4 => chip8.key[0xC] = 0,
                Key::Q => chip8.key[0x4] = 0,
                Key::W => chip8.key[0x5] = 0,
                Key::E => chip8.key[0x6] = 0,
                Key::R => chip8.key[0xD] = 0,
                Key::A => chip8.key[0x7] = 0,
                Key::S => chip8.key[0x8] = 0,
                Key::D => chip8.key[0x9] = 0,
                Key::F => chip8.key[0xE] = 0,
                Key::Z => chip8.key[0xA] = 0,
                Key::X => chip8.key[0x0] = 0,
                Key::C => chip8.key[0xB] = 0,
                Key::V => chip8.key[0xF] = 0,
                _ => continue,
            },
            _ => continue,
        }
    }
}