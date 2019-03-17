# tvb

https://gitlab.com/bfrydl/tvb

Project TVB is a 2D adventure RPG and its engine, Nova.

## Running from source

TVB requires Rust v1.34+ and either Vulkan, DirectX, or Metal development
libraries depending on your platform and what is bundled.

The game can be built and started with cargo:

    cargo run

## Building

The included Makefile can prepare a release build of the game:

    make

This currently assumes Linux.

## Pushing commits to the public Nova repository

The files in the `nova` folder are open source and should be published to the
public Nova repository.

To set this up, add a remote for Nova:

    git remote add nova git@gitlab.com:bfrydl/nova.git
    git remote set-url nova --add git@github.com:bfrydl/nova.git

Changes to files in the `nova` folder can then be published with a subtree push:

    git subtree push nova master --prefix nova

There is also a Makefile target for pushing to this repository and the Nova
repository:

    make push-all
