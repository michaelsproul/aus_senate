extern crate aus_senate;
extern crate csv;
extern crate rustc_serialize;

use std::error::Error;
use std::io::Read;
use std::env;

use aus_senate::ballot::*;
use aus_senate::voting::*;
use aus_senate::util::*;

#[derive(RustcDecodable, Debug)]
struct CandidateRow {
    txn_nm: String,
    nom_ty: String,
    state_ab: String,
    div_nm: String,
    ticket: String,
    ballot_position: u32,
    surname: String,
    other_names: String,
    party: String,
    occupation: String,
    address_1: String,
    address_2: String,
    postcode: String,
    suburb: String,
    address_state: String,
    contact_work_ph: String,
    contact_home_ph: String,
    postal_address_1: String,
    postal_address_2: String,
    postal_suburb: String,
    postal_postcode: String,
    contact_fax: String,
    postal_state_ab: String,
    contact_mobile: String,
    contact_email: String,
}

#[derive(Debug)]
pub struct Candidate {
    id: CandidateId,
    state: String,
    surname: String,
    other_names: String,
    party: String,
}

fn parse_candidates_file<R: Read>(input: R) -> Result<Vec<Candidate>, Box<Error>> {
    let mut result = vec![];
    let mut reader = csv::Reader::from_reader(input);

    for (id, raw_row) in reader.decode::<CandidateRow>().enumerate() {
        let row = try!(raw_row);
        result.push(Candidate {
            id: id as u32,
            state: row.state_ab,
            surname: row.surname,
            other_names: row.other_names,
            party: row.party,
        });
    }

    Ok(result)
}

fn main_with_result() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ./election2016 <candidates file> <state>");
        try!(Err("invalid command line arguments.".to_string()));
    }

    let candidates_file_name = &args[1];
    let state = &args[2];

    let candidates_file = try!(open_aec_csv(candidates_file_name));
    let all_candidates = try!(parse_candidates_file(candidates_file));

    println!("{:?}", all_candidates.into_iter().filter(|c| &c.state == state).collect::<Vec<_>>());

    Ok(())
}

fn main() {
    if let Err(e) = main_with_result() {
        println!("Error: {:?}", e);
    }
}
