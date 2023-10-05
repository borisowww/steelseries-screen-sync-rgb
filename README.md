# Steelseries RGB screen sync.

An  experimental rust program that syncs your Steelseries RGB keyboard with your screen. Currently, it is quite slow
as the screen capture library works in image mode, something that streams video in MPEG-4 would be much faster.

## Usage

Compile the program with `cargo build --release` and run it with `./target/release/steelseries-rgb-screen-sync`.