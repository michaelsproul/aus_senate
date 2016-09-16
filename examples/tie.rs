#[macro_use]
extern crate aus_senate;

use aus_senate::util::*;
use aus_senate::ballot::*;
use aus_senate::candidate::*;
use aus_senate::voting::*;

use std::error::Error;

fn main() {
    if let Err(e) = main_with_result() {
        println!("Error: {}", e);
    }
}

// TODO: make this a test rather than an example.
fn main_with_result() -> Result<(), Box<Error>> {
    let ballots = vec![
        // Major party 1
        MultiBallot::multi(4999, vec![1, 2]),
        // Major party 2
        MultiBallot::multi(5000, vec![2, 1]),
        // Minor party
        MultiBallot::single(vec![3, 1, 2]),
        MultiBallot::single(vec![3])
    ];

    let mut candidates = HashMap::new();
    let dummy_candidate = Candidate {
        id: 0,
        surname: "Candidate".to_string(),
        other_names: "Dummy".to_string(),
        group_name: "".to_string(),
        party: "".to_string(),
        state: "".to_string(),
    };
    candidates.insert(1, Candidate { id: 1, ..dummy_candidate.clone() });
    candidates.insert(2, Candidate { id: 2, ..dummy_candidate.clone() });
    candidates.insert(3, Candidate { id: 3, ..dummy_candidate.clone() });

    let ballots = ballots.into_iter().map(|x| Ok(x));
    let result = try!(decide_election(&candidates, ballots, 1));
    assert!(result.tied);
    println!("{:?}", result);

    Ok(())
}
