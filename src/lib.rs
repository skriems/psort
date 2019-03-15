//! # psort
//! A small utility for sorting jpeg files by date.
//!
//! `psort` looks up the year and month in which your JPEG files were shot,
//! creates the necessary folder structure and copies or moves your pictures there.
extern crate chrono;
extern crate exif;

use std::error;
use std::fs;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use chrono::{Datelike, NaiveDateTime};


/// Returns the file handle or possible Error
pub fn file_handle(path: &str) -> io::Result<fs::File> {
    let file = fs::File::open(path)?;
    Ok(file)
}


/// Returns the exif data of a JPEG file or a possible error
pub fn exif_data(entry: &fs::DirEntry) -> Result<exif::Reader, exif::Error> {
    let file = file_handle(&entry.path().to_str().unwrap())?;
    let reader = exif::Reader::new(&mut BufReader::new(file))?;
    Ok(reader)
}

/// Returns the file handle if the lowercase extension corresponds to
/// either "jpeg" or "jpg". Useful in `filter_map`.
pub fn is_jpeg(file: fs::DirEntry) -> Option<fs::DirEntry> {
    if let Some(extension) = file.path().extension() {
        if let Some(ext) = extension.to_str() {
            let lower_ext = String::from(ext).to_lowercase();
            if lower_ext == "jpeg" || lower_ext == "jpg" {
                return Some(file)
            }
        }
    }
    return None
}


/// Returns a vector of jepg files in a directory to which the user has permissions
pub fn jpeg_files(path: &Path) -> io::Result<Vec<fs::DirEntry>> {
    // TODO return Iterator
    let jpegs = fs::read_dir(path)?
        .filter_map(|f| f.ok())
        .filter_map(|f| is_jpeg(f))
        .collect::<Vec<fs::DirEntry>>();
    Ok(jpegs)
}


/// Returns the desired folder structure `<src|dest>/<year>/<month>` as `path::PathBuf`
///
/// The source folder is chosen by default unless you specify a destination folder.
/// # Examples
/// ```
/// extern crate chrono;
/// use std::path;
/// use chrono::NaiveDateTime;
///
/// let src = path::Path::new("src");
/// let dstr = "2019-01-01 10:00:00";
/// let datetime = NaiveDateTime::parse_from_str(&dstr, "%Y-%m-%d %H:%M:%S").unwrap();
/// // <dest> was not provided on the command line
/// let target_dir = psort::get_target_dir(&src, &None, &datetime);
/// assert_eq!(target_dir, path::PathBuf::from("src/2019/1"));
/// // now with <dest>
/// let dest = path::Path::new("dest");
/// let target_dir = psort::get_target_dir(&src, &Some(Box::new(dest)), &datetime);
/// assert_eq!(target_dir, path::PathBuf::from("dest/2019/1"));
/// ```
pub fn get_target_dir(src: &Path, dest: &Option<Box<&Path>>, datetime: &NaiveDateTime) -> PathBuf {
    let mut target_dir = src.join(format!("{}/{}", datetime.year(), datetime.month()));
    if let Some(dest) = dest {
        target_dir = dest.join(format!("{}/{}", datetime.year(), datetime.month()));
    }
    target_dir
}

/// The function that does the actual work
/// 1. get the datetime information from the jpeg file
/// 2. ensure the necessary folder structure: <dest>/<year>/<month>/
/// 3. copy or move the jpeg file there
pub fn process_jpeg(
    file: &fs::DirEntry,
    src: &Path,
    dest: &Option<Box<&Path>>,
    _move: &bool,
    overwrite: &bool) -> Result<(), Box<error::Error>> {

    let exif_data = exif_data(file)?;
    let dt_field = exif_data.get_field(exif::Tag::DateTime, false);
    if let Some(dt_field) = dt_field {

        let dstr = format!("{}", dt_field.value.display_as(dt_field.tag));
        let dt = NaiveDateTime::parse_from_str(&dstr, "%Y-%m-%d %H:%M:%S")?;

        let target_dir = get_target_dir(&src, &dest, &dt);

        if !target_dir.exists() {
            fs::create_dir_all(&target_dir)?;
        }

        let source_file = src.join(file.file_name());
        let target_file = target_dir.join(file.file_name());

        if target_file.exists() && !*overwrite {
            return Err(
                Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "file already exists")));
        }

        if !_move {
            fs::copy(source_file, target_file)?;
        } else {
            fs::rename(source_file, target_file)?;
        }
    }
    Ok(())
}
