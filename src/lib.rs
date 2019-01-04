//! # psort
//! A small utility sorting jpeg files by date
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


/// Returns either the Exif data or a possible Error
pub fn exif_data(entry: &fs::DirEntry) -> Result<exif::Reader, exif::Error> {
    let file = file_handle(&entry.path().to_str().unwrap())?;
    let reader = exif::Reader::new(&mut BufReader::new(file))?;
    Ok(reader)
}


/// Utility function for `filter_map` which returns the file handle if the lowercase
/// extension corresponds to either "jpeg" or "jpg"
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


/// Returns a vector of jepg files in a directory to which the user has permissions
pub fn jpegs(path: &Path) -> Result<Vec<fs::DirEntry>, io::Error> {
    // TODO return Iterator
    let jpegs = fs::read_dir(path)?
        .filter_map(|f| f.ok())
        .filter_map(|f| is_jpeg(f))
        .collect::<Vec<fs::DirEntry>>();
    Ok(jpegs)
}


/// This function that does the actual work
/// 1. get the datetime information from the jpeg file
/// 2. ensure a folder for the corresponding month a present
/// 3. move the jpeg file into that
pub fn process_jpeg(file: &fs::DirEntry, src: &Path) -> Result<(), Box<error::Error>> {
    let exif_data = exif_data(file)?;
    let dt_field = exif_data.get_field(exif::Tag::DateTime, false);
    if let Some(dt_field) = dt_field {

        let dstr = format!("{}", dt_field.value.display_as(dt_field.tag));
        let dt = NaiveDateTime::parse_from_str(&dstr, "%Y-%m-%d %H:%M:%S")?;
        let target_dir = src.join(format!("{}", dt.month()));

        if !target_dir.exists() {
            fs::create_dir_all(&target_dir)?;
        }

        let source_file = src.join(file.file_name());
        let target_file = target_dir.join(file.file_name());
        // TODO copy if desired by user
        fs::rename(source_file, target_file)?;
    }
    Ok(())
}
