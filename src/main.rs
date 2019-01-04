extern crate clap;
extern crate chrono;
extern crate exif;
extern crate psort;
mod tests;

use std::error::Error;
use std::path::Path;
use std::process;

use clap::{App, Arg, ArgMatches};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}


fn main() {
    let args = App::new("psort")
        .version("0.1.0")
        .author("Sebastian Kriems <bastoberto@gmx.de>")
        .about("Sorting your pictures by date")
        .arg(Arg::with_name("src")
             .required(true)
             .takes_value(true)
             .help("source folder"))
        .arg(Arg::with_name("dest")
             .takes_value(true)
             .help("destination folder (optional)"))
        .arg(Arg::with_name("copy")
             .short("c")
             .long("copy")
             .help("copy files instead of moving"))
        .get_matches();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}


pub fn run(args: ArgMatches) -> Result<(), Box<Error>> {
    let src = Path::new(args.value_of("src").unwrap());

    let mut dest: Option<Box<&Path>> = None;
    if let Some(_dest) = args.value_of("dest") {
        dest = Some(Box::new(Path::new(_dest)));
    }

    let copy = args.is_present("copy");

    if src.is_file() {
        return err!("source argument cannot be a file: {:?}", src);
    }

    for pic in psort::jpegs(&src)? {
        match psort::process_jpeg(&pic, &src, &dest, &copy) {
            Ok(()) => continue,
            Err(e) => eprintln!("{}: {:?}", e, &pic.file_name()),
        }
    }
    Ok(())
}
