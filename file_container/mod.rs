#![allow(dead_code)]

use std::path::Path;
use std::fs::File as STDFile;
use std::io::{Read, Write};

pub struct File {
    file: STDFile,
    directory: String,
    name: String,
    pub contents: String,
}

impl File {
    pub fn new(full_path: String) -> File {
        let file = Path::new(&full_path);
        let filename = file.file_name().unwrap();
        let filepath = file.parent().unwrap();

        let mut file = STDFile::open(&full_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        File {
            file: file,
            directory: filepath.to_str().unwrap().to_string(),
            name: filename.to_str().unwrap().to_string(),
            contents: contents,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_directory(&self) -> &str {
        &self.directory
    }

    pub fn save(&mut self) {
        self.file.write_all(self.contents.as_bytes()).unwrap();
    }
}