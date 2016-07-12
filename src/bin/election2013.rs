#[macro_use] extern crate aus_senate;
extern crate csv;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::fs::File;
use std::env;
use std::error::Error;
use std::io::{self, Read};

use aus_senate::util::*;
use aus_senate::ballot::*;
use aus_senate::voting::*;

/// Group voting ticket description. Maps ticket names (e.g. "A", to preference lists).
type GVT = HashMap<String, Vec<CandidateId>>;

/// Holy moley.
#[derive(RustcDecodable, Debug)]
struct GVTRow {
    state: String,
    owner_group: u32,
    owner_group_name: String,
    owner_ticket: String,
    ticket_num: u32,
    candidate_id: u32,
    candidate_ticket: String,
    surname: String,
    first_name: String,
    ballot_pos: u32,
    party_ab: String,
    party_name: String,
    preference: u32
}

fn convert_pref_map_to_vec(pref_map: &mut HashMap<CandidateId, u32>) -> Vec<CandidateId> {
    let mut temp: Vec<_> = pref_map.drain().collect();
    temp.sort_by_key(|&(cand, pref)| pref);
    temp.into_iter().map(|(cand, pref)| cand).collect()
}

fn parse_gvt<R: Read>(input: R, state: String) -> Result<GVT, Box<Error>> {
    let mut reader = csv::Reader::from_reader(input);

    let mut gvt = HashMap::new();
    let mut current_ticket = None;
    let mut current_ballot: HashMap<CandidateId, u32> = HashMap::new();

    for result in reader.decode::<GVTRow>() {
        let row = try!(result);

        if row.state != state {
            continue;
        }

        if current_ticket.is_none() {
            current_ticket = Some(row.owner_ticket.clone());
        }

        // If we're still reading records for the current ticket, add to the prefs map.
        if &row.owner_ticket == current_ticket.as_ref().unwrap() {
            current_ballot.insert(row.candidate_id, row.preference);
        }
        // Otherwise, emit the current preferences and start on a new one.
        else {
            let preferences = convert_pref_map_to_vec(&mut current_ballot);
            gvt.insert(current_ticket.unwrap(), preferences); // YICK
            current_ticket = Some(row.owner_ticket);
            current_ballot.insert(row.candidate_id, row.preference);
        }
    }

    // FIXME: Don't forget the last ticket!

    Ok(gvt)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let f = File::open(&args[1]).unwrap();
    let gvt = parse_gvt(f, "NSW".into()).unwrap();
    println!("{:?}", gvt["AO"]);
}
