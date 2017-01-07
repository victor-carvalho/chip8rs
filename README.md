chip8rs is my first hobby project written in [Rust](https://www.rust-lang.org).

Build Instructions
------------------

## Requirements

### Rust

I'm currently using Rust nightly 1.16. You can install with [rustup](https://www.rustup.rs/).

### SDL2

You need SDL2 installed to run the emulator. Read the instructions [here](https://github.com/AngryLawyer/rust-sdl2#sdl20-development-libraries).

### On Windows

```
build.cmd
target\release\chip8rs.exe roms\INVADERS
```

### Linux, MacOS

```
cargo build --release
cargo run --release roms/INVADERS
```

![windows_prt_sc](https://dl.dropboxusercontent.com/u/51598192/windows_prt_sc.png)