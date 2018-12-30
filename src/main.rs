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
        .version("0.0.1")
        .author("Sebastian Kriems <bastoberto@gmx.de>")
        .about("Sorting your pictures by date")
        .arg(Arg::with_name("src")
             .required(true)
             .takes_value(true)
             .index(1)
             .help("source folder containing jpeg files"))
        // TODO
        // .arg(Arg::with_name("dest")
        //      .required(false)
        //      .takes_value(true)
        //      .index(2)
        //      .help("destination folder"))
        .get_matches();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}


pub fn run(args: ArgMatches) -> Result<(), Box<Error>> {
    let src = Path::new(args.value_of("src").unwrap());

    if src.is_file() {
        return err!("source argument cannot be a file: {:?}", src);
    }

    for pic in psort::jpegs(&src)? {
        // TODO match `Some` and `Err` here and return a summary of
        // errors that occured
        psort::process_jpeg(&pic, &src)?;
    }
    Ok(())
}
