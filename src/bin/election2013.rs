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

/// Group voting ticket description. Maps states to ticket names to preference lists.
type GVT = HashMap<String, HashMap<String, Vec<CandidateId>>>;

/// Temporary preference map type mapping candidate IDs to preferences.
type PrefMap = HashMap<CandidateId, u32>;

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

fn pref_to_vec(pref_map: PrefMap) -> Vec<CandidateId> {
    let mut temp: Vec<_> = pref_map.into_iter().collect();
    temp.sort_by_key(|&(cand, pref)| pref);
    temp.into_iter().map(|(cand, pref)| cand).collect()
}

// NOTE: This is a tad slow, but it beats mucking around with manual row groupings.
fn parse_gvt<R: Read>(input: R) -> Result<GVT, Box<Error>> {
    let mut data: HashMap<String, HashMap<String, PrefMap>> = HashMap::new();

    let mut reader = csv::Reader::from_reader(input);

    for result in reader.decode::<GVTRow>() {
        let row = try!(result);
        let ticket_map = data.entry(row.state).or_insert_with(HashMap::new);
        let pref_map = ticket_map.entry(row.owner_ticket).or_insert_with(HashMap::new);
        pref_map.insert(row.candidate_id, row.preference);
    }

    // Convert inner preference maps into lists.
    let mut result = HashMap::new();
    for (state, ticket_map) in data {
        let new_ticket_map = result.entry(state).or_insert_with(HashMap::new);
        for (ticket, pref_map) in ticket_map {
            new_ticket_map.insert(ticket, pref_to_vec(pref_map));
        }
    }
    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let f = File::open(&args[1]).unwrap();
    let gvt = parse_gvt(f).unwrap();
    println!("{:?}", gvt["NSW"]["AO"]);
}
