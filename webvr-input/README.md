#WEBVR-INPUT Rust Implementation

This is a library for representing input devices in a way to standardize commmunication


* [Installation of Rust](#installation)
* [Compiling](#compiling)
* [Usage](#usage)

## Installation

Follow the instructions at [www.rustup.rs][2] to install all the dependencies needed.

`rustup` installs `rustc`, `cargo`, `rustup` and other standard tools
to Cargo's `bin` directory. On Unix it is located at
`$HOME/.cargo/bin` and on Windows at `%USERPROFILE%\.cargo\bin`. This
is the same directory that `cargo install` will install Rust programs
and Cargo plugins.

This directory will be in your `$PATH` environment variable, which
means you can run them from the shell without further
configuration. Open a *new* shell and type the following:

```
rustc --version
```

If you see something like `rustc 1.7.0 (a5d1e7a59 2016-02-29)` then
you are ready to Rust. 


## Compiling

Since cargo is already installed you can simply run:

```
 cargo build --release
```
The compiled binary will then be placed under  `[project_folder]/target/release`. The binary will
include all the optimizations that rusc has to offer

## Usage

*TODO*
