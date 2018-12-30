extern crate chrono;
extern crate exif;

use std::error;
use std::fs;
use std::io;
use std::io::BufReader;
use std::path::Path;

use chrono::{Datelike, NaiveDateTime};


pub fn file_handle(path: &str) -> Result<fs::File, io::Error> {
    let file = fs::File::open(path)?;
    Ok(file)
}


pub fn exif_data(entry: &fs::DirEntry) -> Result<exif::Reader, exif::Error> {
    let file = file_handle(&entry.path().to_str().unwrap())?;
    let reader = exif::Reader::new(&mut BufReader::new(file))?;
    Ok(reader)
}


pub fn is_jpeg(file: fs::DirEntry) -> Option<fs::DirEntry> {
    if file.path().is_file() {
        if let Some(ext) = file.path().extension().unwrap().to_str() {
            let lower_ext = String::from(ext).to_lowercase();
            if lower_ext == "jpeg" || lower_ext == "jpg" {
                return Some(file)
            }
        }
    }
    return None
}


pub fn jpegs(path: &Path) -> Result<Vec<fs::DirEntry>, io::Error> {
    // TODO return Iterator
    let jpegs = fs::read_dir(path)?
        .filter_map(|f| f.ok())
        .filter_map(|f| is_jpeg(f))
        .collect::<Vec<fs::DirEntry>>();
    println!("{:?}", jpegs);
    Ok(jpegs)
}


pub fn process_jpeg(file: &fs::DirEntry, src: &Path) -> Result<(), Box<error::Error>> {
    let exif_data = exif_data(file)?;
    let dt_field = exif_data.get_field(exif::Tag::DateTime, false);
    if let Some(dt_field) = dt_field {

        // 1st: get the DateTime information as string
        let dstr = format!("{}", dt_field.value.display_as(dt_field.tag));

        // 2nd: convert to DateTime object
        let dt = NaiveDateTime::parse_from_str(&dstr, "%Y-%m-%d %H:%M:%S")?;

        // 3rd: ensure directory
        let target_dir = src.join(format!("{}", dt.month()));

        println!("{:?} exists: {:?}", target_dir, target_dir.exists());
        if !target_dir.exists() {
            println!("creating directory: {:?}", target_dir);
            fs::create_dir_all(&target_dir)?;
        }

        // 4th: move file
        let source_file = src.join(file.file_name());
        let target_file = target_dir.join(file.file_name());
        println!("renaming {:?} to {:?}", source_file, target_file);
        // TODO copy if desired by user
        fs::rename(source_file, target_file)?;
    }
    Ok(())
}
