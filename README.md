![Rust](https://github.com/acemuzzy/martian/actions/workflows/rust.yml/badge.svg)

# Martian runner!

## How to run

The runner takes a single command-line attribute, which must be the path of a file containing valid input data:

`martian <filename>`

On successful execution, the run output is printed directly to screen.  If no filename is given, or its contents are invalid, the program will fail with an error message.

## How to build

The code must be compiled using the standard Rust toolchain.
* See https://www.rust-lang.org/tools/install for details
  * NOTE: Code is intended to be fully platform agnostic, but has only be tested on WSL Ubuntu
* Once installed, you can either:
  * compile with e.g. `cargo build --release`, and then either move the executable or run as e.g. `./target/release/martian <filename>`
  * run directly via `cargo run <filename>`

## How to extend

The core function is driven by `lib::run_file`, which is the likely place to begin extensions.

There are separate modules for each of `Direction`, `Movement`, handling the `Map` (bounds constraints & scents) and `Martian` movement - each named after their central object.

UT and FV files must be extended for any functional changes.
