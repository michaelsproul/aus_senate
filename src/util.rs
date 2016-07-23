use std::io::{self, BufRead, BufReader};
use std::fs::File;

pub use num::rational::Ratio;
pub use num::*;

pub type Uint = BigUint;
pub type Frac = Ratio<Uint>;

#[macro_export]
macro_rules! uint {
    ($e:expr) => {
        BigUint::from($e as u64)
    }
}

#[macro_export]
macro_rules! frac {
    ($e:expr) => {
        frac!($e, 1)
    };
    ($e1:expr, $e2:expr) => {
        Ratio::new(BigUint::from($e1 as u64), BigUint::from($e2 as u64))
    };
}

/// Open an AEC CSV file for reading, whilst chomping the first line (a comment).
pub fn open_aec_csv(filename: &str) -> io::Result<BufReader<File>> {
    let f = try!(File::open(filename));
    let mut r = BufReader::new(f);
    try!(r.read_line(&mut String::new()));
    Ok(r)
}
