use std::fs::File;
use std::io::{self, Write};

pub struct FileLogger {
    file: File,
}

impl FileLogger {
    pub fn new(filename: &str) -> Result<Self, io::Error> {
        Ok(FileLogger {
            file: File::create(filename)?,
        })
    }

    pub fn write<T: AsRef<str>>(&self, line: T) {
        // Ignore failures
        let mut f = &self.file;
        let _ = f.write_all(line.as_ref().as_bytes());
        let _ = f.write_all(&[b'\n']);
    }
}
