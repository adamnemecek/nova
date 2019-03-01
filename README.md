# tvb

https://gitlab.com/bfrydl/tvb

Project TVB is a 2D adventure RPG and its engine, Nova.

## Running from source

TVB requires Rust v1.32+ and either Vulkan, DirectX, or Metal development
libraries depending on your platform and what is bundled.

The game can be built and started with cargo:

    cargo run

## Building

The included Makefile can prepare a release build of the game:

    make

This currently assumes Linux.
