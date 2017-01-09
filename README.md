chip8rs is my first hobby project written in [Rust](https://www.rust-lang.org).

# Requirements

## Rust

I'm currently using Rust nightly 1.16. You can install with [rustup](https://www.rustup.rs/).

# Build Instructions

## On Windows (MSVC)

```
build.cmd
target\release\chip8rs.exe roms\INVADERS
```

## Linux, MacOS

```
cargo build --release
cargo run --release roms/INVADERS
```

This will run the INVADERS rom as the image below (Windows 10)

![windows_prt_sc](https://dl.dropboxusercontent.com/u/51598192/windows_prt_sc.png)
