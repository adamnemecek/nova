# nova

An experiment in creating a game with Rust.

Nova uses [ggez][1] as a low-level cross-platform engine and [specs][2] for an
Entity-Component-System design.

## Running

To run the game, use the included `run.sh` script, or run with cargo:

    ./run.sh
    # or
    cargo run -p nova-game

## Building

To create a release build, use the included `Makefile`:

    make

This currently only works on Linux (and possibly MacOS). Nova works perfectly
fine on Windows, but the Makefile assumes Linux.

[1]: https://ggez.rs/
[2]: https://slide-rs.github.io/specs/
