extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io;
use std::io::Read;

use utils::*;

const PC_START: usize = 0x200;

static FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[allow(dead_code)]
pub struct Chip8 {
    pub debug: bool,
    pub draw_flag: bool,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub key: [u8; 16],
    pub addr: u16,
    pub pc: u16,
    pub sp: u16,
    pub v: [u8; 16],
    pub stack: [u16; 16],
    pub memory: [u8; 4096],
    pub gfx: [u8; 64 * 32],
    pub rng: rand::ThreadRng,
}

#[allow(dead_code)]
impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            debug: false,
            draw_flag: true,
            delay_timer: 0,
            sound_timer: 0,
            key: [0; 16],
            addr: 0,
            pc: PC_START as u16,
            sp: 0,
            v: [0; 16],
            stack: [0; 16],
            memory: [0; 4096],
            gfx: [0; 64 * 32],
            rng: rand::thread_rng(),
        };
        chip8.memory[0..0x50].copy_from_slice(&FONTSET);
        chip8
    }

    pub fn load(&mut self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        let mut buf = [0u8; 1024];
        let mut n = file.read(&mut buf).unwrap();
        let mut i = PC_START;
        while n > 0 {
            self.memory[i..i + n].copy_from_slice(&buf[0..n]);
            i += n;
            n = file.read(&mut buf).unwrap();
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = self.fetch_opcode();
        self.handle_opcode(opcode);
    }

    #[inline(always)]
    fn fetch_opcode(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16
    }

    #[inline(always)]
    fn handle_opcode(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x000F {
                0x0000 => {
                    for i in 0..2048 {
                        self.gfx[i] = 0x0;
                    }
                    self.draw_flag = true;
                    self.pc += 2;
                },
                0x000E => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                    self.pc += 2;
                },
                _ => panic!("unimplemented {:x}", opcode),
            },
            0x1000 => self.pc = get_addr(opcode),
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = get_addr(opcode);
            },
            0x3000 => {
                if self.v[get_vx(opcode)] == (opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x4000 => {
                if self.v[get_vx(opcode)] != (opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x5000 => {
                if self.v[get_vx(opcode)] == self.v[get_vy(opcode)] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6000 => {
                self.v[get_vx(opcode)] = (opcode & 0x00FF) as u8;
                self.pc += 2;
            },
            0x7000 => {
                let vx = get_vx(opcode);
                self.v[vx] = self.v[vx].wrapping_add((opcode & 0x00FF) as u8);
                self.pc += 2;
            },
            0x8000 => match opcode & 0x000F {
                0x0000 => {
                    self.v[get_vx(opcode)] = self.v[get_vy(opcode)];
                    self.pc += 2;      
                },
                0x0001 => {
                    self.v[get_vx(opcode)] |= self.v[get_vy(opcode)];
                    self.pc += 2;      
                },
                0x0002 => {
                    self.v[get_vx(opcode)] &= self.v[get_vy(opcode)];
                    self.pc += 2;      
                },
                0x0003 => {
                    self.v[get_vx(opcode)] ^= self.v[get_vy(opcode)];
                    self.pc += 2;      
                },
                0x0004 => {
                    let vx = get_vx(opcode);
                    let vy = get_vy(opcode);
                    if self.v[vy] > 0xFF - self.v[vx] {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[vx] = self.v[vx].wrapping_add(self.v[vy]);
                    self.pc += 2;
                },
                0x0005 => {
                    let vx = get_vx(opcode);
                    let vy = get_vy(opcode);
                    if self.v[vy] > self.v[vx] {
                        self.v[0xF] = 0;
                    } else {
                        self.v[0xF] = 1;
                    }
                    self.v[vx] = self.v[vx].wrapping_sub(self.v[vy]);
                    self.pc += 2;
                },
                0x0006 => {
                    let vx = get_vx(opcode);
                    self.v[0xF] = self.v[vx] & 0x1;
                    self.v[vx] >>= 1;
                    self.pc += 2;
                },
                0x0007 => {
                    let vx = get_vx(opcode);
                    let vy = get_vy(opcode);
                    if self.v[vx] > self.v[vy] {
                        self.v[0xF] = 0;
                    } else {
                        self.v[0xF] = 1;
                    }
                    self.v[vx] = self.v[vy].wrapping_sub(self.v[vx]);
                    self.pc += 2;
                },
                0x000E => {
                    let vx = get_vx(opcode);
                    self.v[0xF] = self.v[vx] >> 7;
                    self.v[vx] <<= 1;
                    self.pc += 2;
                },
                _ => panic!("unimplemented {:x}", opcode),
            },
            0x9000 => {
                if self.v[get_vx(opcode)] != self.v[get_vy(opcode)] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0xA000 => {
                self.addr = get_addr(opcode);
                self.pc += 2;          
            },
            0xB000 => self.pc = get_addr(opcode).wrapping_add(self.v[0] as u16),
            0xC000 => {
                self.v[get_vx(opcode)] = ((self.rng.next_u32() % 0xFF) as u16 & opcode & 0xFF) as u8;
                self.pc += 2;
            },
            0xD000 => {
                let vx = self.v[get_vx(opcode)] as usize;
                let vy = self.v[get_vy(opcode)] as usize;
                let height = (opcode & 0x000F) as usize;
                
                self.v[0xF] = 0;
                for y in 0..height {
                    let pixel = self.memory[self.addr as usize + y];
                    for x in 0..8 {
                        if pixel & (0x80 >> x as u8) != 0 {
                            let i = vx + x + (vy + y) * 64;
                            if i < 2048 {
                                if self.gfx[i] != 0 {
                                    self.v[0xF] = 1;
                                }
                                self.gfx[i] ^= 1;
                            }
                        }
                    }
                }

                self.draw_flag = true;
                self.pc += 2;
            },
            0xE000 => match opcode &0x00FF {
                0x009E => {
                    if self.key[self.v[get_vx(opcode)] as usize] != 0 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                },
                0x00A1 => {
                    if self.key[self.v[get_vx(opcode)] as usize] == 0 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                },
                _ => panic!("unimplemented {:x}", opcode),
            },
            0xF000 =>  match opcode & 0x00FF {
                0x0007 => {
                    self.v[get_vx(opcode)] = self.delay_timer;
                    self.pc += 2;
                },
                0x000A => {
                    let vx = get_vx(opcode);
                    let mut pressed = false;
                    for i in 0..16 {
                        if self.key[i] != 0 {
                            self.v[vx] = i as u8;
                            pressed = true;
                        }
                    }
                    if pressed { 
                        self.pc += 2;
                    } else {
                        return;
                    }
                },
                0x0015 => {
                    self.delay_timer = self.v[get_vx(opcode)];
                    self.pc += 2;
                },
                0x0018 => {
                    self.sound_timer = self.v[get_vx(opcode)];
                    self.pc += 2;
                },
                0x001E => {
                    let vx = self.v[get_vx(opcode)] as u16;
                    self.v[0xF] =  if self.addr.wrapping_add(vx) > 0xFFF { 1 } else { 0 };
                    self.addr = self.addr.wrapping_add(vx);
                    self.pc += 2;
                },
                0x0029 => {
                    self.addr = (self.v[get_vx(opcode)] as u16) * 0x5;
                    self.pc += 2;
                },
                0x0033 => {
                    let vx = get_vx(opcode);
                    let addr = self.addr as usize;
                    self.memory[addr] = self.v[vx] / 100;
                    self.memory[addr + 1] = (self.v[vx] / 10) % 10;
                    self.memory[addr + 2] = (self.v[vx] % 100) % 10;
                    self.pc += 2;
                },
                0x0055 => {
                    let vx = get_vx(opcode) + 1;
                    let addr = self.addr as usize;
                    for i in 0..vx {
                        self.memory[i + addr] = self.v[i];
                    }
                    self.addr += vx as u16;
                    self.pc += 2;
                },
                0x0065 => {
                    let vx = get_vx(opcode) + 1;
                    let addr = self.addr as usize;
                    for i in 0..vx {
                        self.v[i] = self.memory[i + addr];
                    }
                    self.addr += vx as u16;
                    self.pc += 2;
                },
                _ => panic!("unimplemented {:x}", opcode),
            },
            _ => panic!("unimplemented {:x}", opcode),
        }
        if self.delay_timer > 0 {
            self.delay_timer -= 1; 
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}