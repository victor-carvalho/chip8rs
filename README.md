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

![windows_prt_sc](https://photos-6.dropbox.com/t/2/AAAccGGzQ-NuFv_D79VlnJJ9XUpssfza8r84AHt7a0_99A/12/51598192/png/32x32/3/1505253600/0/2/windows_prt_sc.png/EIL03CcYmCcgAigC/ug_x7Ke08alhAFCt5epX1H6cqWK-vFnzgKH6y-P_zGU?dl=0&size=800x600&size_mode=3)
