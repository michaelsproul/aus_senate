use std::io::{self, BufRead, BufReader};
use std::fs::File;

pub use std::collections::{HashMap, BTreeMap};
pub use gmp::mpz::Mpz;
pub use gmp::mpq::Mpq;

pub type Int = Mpz;
pub type Frac = Mpq;

#[macro_export]
macro_rules! frac {
    ($e:expr) => {
        frac!($e, 1)
    };
    ($e1:expr, $e2:expr) => {
        Mpq::ratio(&Mpz::from($e1 as u64), &Mpz::from($e2 as u64))
    };
}

/// Open an AEC CSV file for reading, whilst chomping the first line (a comment).
// FIXME: consider using the CSV reader's comment functionality.
pub fn open_aec_csv(filename: &str) -> io::Result<BufReader<File>> {
    let f = File::open(filename)?;
    let mut r = BufReader::new(f);
    r.read_line(&mut String::new())?;
    Ok(r)
}
