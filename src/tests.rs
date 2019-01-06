#![allow(unused_imports)]

#[cfg(test)]
extern crate psort;

use std::path;
use chrono::NaiveDateTime;


#[test]
fn get_target_dir_without_destination() {
    let src = path::Path::new("src");
    let dstr = "2019-01-01 10:00:00";
    let datetime = NaiveDateTime::parse_from_str(&dstr, "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(
        psort::get_target_dir(&src, &None, &datetime),
        path::PathBuf::from("src/2019/1"))
}

#[test]
fn get_target_dir_with_destination() {
    let src = path::Path::new("src");
    let dest = path::Path::new("dest");
    let dstr = "2019-01-01 10:00:00";
    let datetime = NaiveDateTime::parse_from_str(&dstr, "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(
        psort::get_target_dir(&src, &Some(Box::new(dest)), &datetime),
        path::PathBuf::from("dest/2019/1"))
}
