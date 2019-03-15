extern crate chrono;
extern crate exif;
extern crate psort;

extern crate structopt;

mod tests;

use std::error;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use std::process;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<error::Error>::from(format!($($tt)*))) }
}


#[derive(StructOpt, Debug)]
#[structopt(name = "psort", about = "Utility for sorting jpegs by date")]
pub struct Opt {
    /// source folder
    #[structopt(parse(from_os_str))]
    src: PathBuf,

    /// destination folder
    #[structopt(parse(from_os_str))]
    dest: Option<PathBuf>,

    /// move files instead of copying
    #[structopt(short = "m", long = "move")]
    r#move: bool,

    /// overwrite existing files
    #[structopt(short = "o", long = "overwrite")]
    overwrite: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    if let Err(e) = run(opt) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}


pub fn run(opt: Opt) -> Result<(), Box<error::Error>> {
    let src = Path::new(&opt.src);

    let mut dest: Option<Box<&Path>> = None;
    if let Some(_dest) = &opt.dest {
        dest = Some(Box::new(Path::new(_dest)));
    }

    if src.is_file() {
        return err!("source argument cannot be a file: {:?}", src);
    }

    for pic in psort::jpeg_files(&src)? {
        match psort::process_jpeg(&pic, &src, &dest, &opt.r#move, &opt.overwrite) {
            Ok(()) => continue,
            Err(e) => eprintln!("{}: {:?}", e, &pic.file_name()),
        }
    }
    Ok(())
}
