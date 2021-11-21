use crate::core::Segment;
use std::fs;

pub struct Directory<'a> {
    dir: &'a str,
    latest_commit: String,
}

impl<'a> Directory<'a> {
    pub fn open(dir: &'a str) -> crate::Result<Directory<'a>> {
        let mut latest_commit = String::from("");
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            if let Some(0) = filename.find("segments_") {
                latest_commit = String::from(filename);
            }
        }
        let dir = Directory { dir, latest_commit };
        Segment::read_latest_commit(&dir.latest_commit);
        Ok(dir)
    }
}
