# imgutils 🌸

Cross-platform command-line utility for quickly parsing & manipulating images powered by amazing crates like [`image-rs`](https://crates.io/crates/image) 💜

## Installation

### Cargo

The simplest way to install `imgutils` is via Cargo. Assuming you already have Cargo installed locally you can simply run following:

```sh
cargo install imgutils
```

### Prebuilt binaries

Pre-built binaries are made available on the repository's [Releases](https://github.com/Blooym/imgutils/releases) for as many platforms as possible. Simply grab the right release asset for your system and place it somewhere in your `$PATH`.

### Other package manegrs

`imgutils` is not available from any other package manager at this time. 3rd party packaging is welcomed.

## Usage

All commands that perform any permanent modifications to images are kept under the `modify` subcommand to provide a clear distinction. All other commands are either kept at the top-level or nested into subcommands when deemed appropriate.

```
Usage: imgutils <COMMAND>

Commands:
  details     Print detailed information about an image in a pretty format
  dimensions  Print an image's dimensions formatted as 'WidthxHeight'
  modify      A collection of commands that perform modifications to images
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

## Features

### Actions

* [x] Blur
* [x] Brighten
* [x] Constrast
* [x] Crop
* [x] Flip
* [x] Grayscale
* [x] Invert
* [x] Hue
* [x] Format
* [x] Resize
* [x] Rotate
* [x] Get Image Dimensions

### Codecs

Sourced from the [image-rs supported formats](https://docs.rs/image/latest/image/codecs/index.html#supported-formats) documentation.

* [x] AVIF
* [x] BMP
* [x] Farbfeld
* [x] GIF
* [x] HDR
* [x] ICO
* [x] JPEG
* [x] EXR
* [x] PNG
* [x] PNM
* [x] QOI
* [x] TGA
* [x] TIFF
* [x] WebP

## License

This crate is dual-licensed under both the [MIT License](./LICENSE-MIT) and [Apache 2.0 license](./LICENSE-APACHE).