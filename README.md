![crates.io](https://img.shields.io/crates/v/psort.svg)

# README

> A small utility which sorts jpeg files according to their exif data.

You don't want to rely on some 3rd party software to organize your pictures? `psort` to the rescue!

`psort` looks up the month in which your JPEG files were shot, creates a folder per month and moves the pictures into their corresponding folders. If you prefer to have them copied, you can do that as well.

Usage
-----
The cli is pretty straight forward:

```
psort [FLAGS] <src> [dest]

FLAGS:
    -c, --copy       copy files instead of moving
    -h, --help       Prints help information
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
