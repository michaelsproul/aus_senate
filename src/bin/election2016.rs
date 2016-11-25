#[macro_use]
extern crate aus_senate;
extern crate csv;
extern crate rustc_serialize;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::error::Error;
use std::io::Read;
use std::env;
use std::fs::File;

use aus_senate::group::*;
use aus_senate::candidate::*;
use aus_senate::voting::*;
use aus_senate::util::*;
use aus_senate::ballot_parse::*;

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

fn parse_candidates_file<R: Read>(input: R) -> Result<Vec<Candidate>, Box<Error>> {
    let mut result = vec![];
    let mut reader = csv::Reader::from_reader(input);

    for (id, raw_row) in reader.decode::<CandidateRow>().enumerate() {
        let row = try!(raw_row);
        if row.nom_ty != "S" {
            continue;
        }
        result.push(Candidate {
            id: id as u32,
            surname: row.surname,
            other_names: row.other_names,
            group_name: row.ticket,
            party: row.party,
            state: row.state_ab,
        });
    }

    Ok(result)
}

fn get_candidate_id_list(candidates: &[Candidate], state: &String) -> Vec<CandidateId> {
    candidates.iter()
        .filter(|c| &c.state == state)
        .map(|c| c.id)
        .collect()
}

fn main_with_result() -> Result<(), Box<Error>> {
    try!(env_logger::init());

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 && args.len() != 5 {
        println!("Usage: ./election2016 <candidates file> <prefs file> <state> [num candidates]");
        try!(Err("invalid command line arguments.".to_string()));
    }

    let candidates_file_name = &args[1];
    let prefs_file_name = &args[2];
    let state = &args[3];
    let num_candidates = match args.get(4) {
        Some(x) => try!(x.parse::<u32>()),
        None => 12
    };

    let candidates_file = try!(File::open(candidates_file_name));
    let all_candidates = try!(parse_candidates_file(candidates_file));

    for c in all_candidates.iter() {
        debug!("{}: {} {} ({})", c.id, c.other_names, c.surname, c.party);
    }

    // Extract candidate and group information from the complete list of ballots.
    let candidates = get_state_candidates(&all_candidates, state);
    let candidate_ids = get_candidate_id_list(&all_candidates, state);
    let groups = get_group_list(&all_candidates, state);

    // TODO: command-line configurable constraints
    let constraints = Constraints::lax2016();

    debug!("Num groups: {}", groups.len());
    trace!("Groups: {:#?}", groups);

    let prefs_file = try!(open_aec_csv(prefs_file_name));

    let mut csv_reader = csv::Reader::from_reader(prefs_file);
    let ballots_iter = parse_preferences_file!(csv_reader, &groups, &candidate_ids, &constraints);

    let election_result = try!(decide_election(&candidates, ballots_iter, num_candidates));

    println!("=== Elected ===");
    for &(c, ref votes) in election_result.senators.iter() {
        println!("{} {} ({}) [{} votes]", c.other_names, c.surname, c.party, votes);
    }

    if election_result.tied {
        println!("Tie for the last place");
    }

    Ok(())
}

fn main() {
    if let Err(e) = main_with_result() {
        error!("Error: {:?}", e);
    }
}
