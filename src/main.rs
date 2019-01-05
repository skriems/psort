extern crate clap;
extern crate chrono;
extern crate exif;
extern crate psort;
mod tests;

use std::error;
use std::path::Path;
use std::process;

use clap::{App, Arg, ArgMatches};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<error::Error>::from(format!($($tt)*))) }
}


fn main() {
    let args = App::new("psort")
        .version("0.1.2")
        .author("Sebastian Kriems <bastoberto@gmx.de>")
        .about("Sorting your pictures by date")
        .arg(Arg::with_name("src")
             .required(true)
             .takes_value(true)
             .help("source folder"))
        .arg(Arg::with_name("dest")
             .takes_value(true)
             .help("destination folder (optional)"))
        .arg(Arg::with_name("move")
             .short("m")
             .long("move")
             .help("move files instead of copying"))
        .arg(Arg::with_name("overwrite")
             .short("o")
             .long("overwrite")
             .help("overwrite existing files"))
        .get_matches();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}


pub fn run(args: ArgMatches) -> Result<(), Box<error::Error>> {
    let src = Path::new(args.value_of("src").unwrap());

    let mut dest: Option<Box<&Path>> = None;
    if let Some(_dest) = args.value_of("dest") {
        dest = Some(Box::new(Path::new(_dest)));
    }

    let _move = args.is_present("move");
    let overwrite = args.is_present("overwrite");

    if src.is_file() {
        return err!("source argument cannot be a file: {:?}", src);
    }

    for pic in psort::jpegs(&src)? {
        match psort::process_jpeg(&pic, &src, &dest, &_move, &overwrite) {
            Ok(()) => continue,
            Err(e) => eprintln!("{}: {:?}", e, &pic.file_name()),
        }
    }
    Ok(())
}
