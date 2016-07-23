#[macro_use] extern crate aus_senate;
extern crate csv;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::Read;

use aus_senate::ballot::*;
use aus_senate::voting::*;
use aus_senate::util::*;

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
    temp.sort_by_key(|&(_, pref)| pref);
    temp.into_iter().map(|(cand, _)| cand).collect()
}

fn get_candidate_list(gvt: &GVT) -> Vec<CandidateId> {
    gvt["NSW"]["A"].clone()
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

/// GVT usage parsing.
#[derive(RustcDecodable, Debug)]
struct GVTUsageRow {
    state: String,
    ticket: String,
    group_ab: String,
    group_name: String,
    ticket_votes: u32,
    ticket_percentage: String,
    non_ticket_votes: String,
    non_ticket_percentage: String,
    total_votes: String
}

type GVTUsage = HashMap<String, HashMap<String, u32>>;

fn parse_gvt_usage<R: Read>(input: R) -> Result<GVTUsage, Box<Error>> {
    let mut gvt_usage = HashMap::new();

    let mut reader = csv::Reader::from_reader(input);

    for raw_row in reader.decode::<GVTUsageRow>() {
        let row = try!(raw_row);
        let ticket_map = gvt_usage.entry(row.state).or_insert_with(HashMap::new);
        // Skip ungrouped candidates with 0 vote.
        if &row.ticket == "UG" {
            continue;
        }
        let prev = ticket_map.insert(row.ticket, row.ticket_votes);
        assert!(prev.is_none());
    }

    Ok(gvt_usage)
}

fn main_with_result() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: ./election2013 <gvt file> <gvt usage file> <state>");
        try!(Err("invalid command line arguments.".to_string()));
    }

    let gvt_file_name = &args[1];
    let gvt_usage_file_name = &args[2];
    let state = &args[3];

    let gvt_file = try!(open_aec_csv(gvt_file_name));
    let gvt = try!(parse_gvt(gvt_file));
    let gvt_usage_file = try!(open_aec_csv(gvt_usage_file_name));
    let gvt_usage = try!(parse_gvt_usage(gvt_usage_file));

    let candidates = get_candidate_list(&gvt);

    // Construct the list of ballots according to the GVT.
    let ballots = gvt_usage[state].iter().map(|(group, &vote_count)| {
        Ballot::new(vote_count, gvt[state][group].clone())
    }).collect();

    println!("{:?}", decide_election(&candidates, ballots, 6));

    Ok(())
}

fn main() {
    if let Err(e) = main_with_result() {
        println!("Error: {:?}", e);
    }
}
