use std::io::{self, BufRead, BufReader};
use std::fs::File;

pub use std::collections::HashMap;

/// Open an AEC CSV file for reading, whilst chomping the first line (a comment).
pub fn open_aec_csv(filename: &str) -> io::Result<BufReader<File>> {
    let f = try!(File::open(filename));
    let mut r = BufReader::new(f);
    try!(r.read_line(&mut String::new()));
    Ok(r)
}
