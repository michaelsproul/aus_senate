#[macro_use]
extern crate aus_senate;

use aus_senate::quota::*;
use aus_senate::util::*;
use aus_senate::ballot::*;
use aus_senate::voting::*;

fn main() {
    if let Err(e) = main_with_result() {
        println!("Error: {}", e);
    }
}

fn main_with_result() -> Result<(), String> {
    println!("{}", senate_quota(frac!(4376143), 6));

    let ballots = vec![
        // Major party 1
        Ballot::new(4999, vec![1, 2]),
        // Major party 2
        Ballot::new(5000, vec![2, 1]),
        // Minor party
        Ballot::new(1, vec![3, 1, 2]),
        Ballot::new(1, vec![3])
    ];

    let elected = try!(decide_election(&[1, 2, 3, 4], ballots, 1));
    println!("{:?}", elected);

    Ok(())
}
