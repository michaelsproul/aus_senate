extern crate num;

#[macro_use]
mod util;
mod ballot;
mod quota;
mod voting;

use quota::*;
use util::*;
use ballot::*;
use voting::*;

fn main() {
    if let Err(e) = main_with_result() {
        println!("Error: {}", e);
    }
}

fn main_with_result() -> Result<(), String> {
    println!("{}", senate_quota(4376143, 6));

    let ballots = vec![Ballot {count: 5000, prefs: vec![1, 2], current: 0}, Ballot {count: 4999, prefs: vec![2, 1], current: 0}];

    let elected = try!(decide_election(&[1, 2], ballots, 1));
    println!("{:?}", elected);

    Ok(())
}
