[![Build Status](https://travis-ci.org/skriems/psort.svg?branch=master)](https://travis-ci.org/skriems/psort)
![crates.io](https://img.shields.io/crates/v/psort.svg)

# README

> A small utility which sorts jpeg files according to their exif data.

You don't want to rely on some 3rd party software to organize your pictures? `psort` to the rescue!

`psort` looks up the year and month in which your JPEG files were shot, creates the necessary folder structure and copies or moves your pictures there.

Installation
------------
To install `psort` use cargo and add `~/.cargo/bin` to your PATH:

```
cargo install psort
```

Usage
-----
The cli is pretty straight forward:

```
psort [FLAGS] <src> [dest]

FLAGS:
    -m, --move       move files instead of copying
    -h, --help       Prints help information
    -o, --overwrite  overwrite existing files
    -V, --version    Prints version information

ARGS:
    <src>     source folder
    <dest>    destination folder (optional)
```

The source folder is mandatory. If you want to sort pictures in the current directory use `.`:
```
psort .
```

Otherwise the pictures get sorted inside the source folder unless you specify a destination folder.

Cheers!
