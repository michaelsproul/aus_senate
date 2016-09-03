#[macro_use] extern crate aus_senate;
extern crate csv;
extern crate rustc_serialize;

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io::Read;

use aus_senate::candidate::*;
use aus_senate::ballot::*;
use aus_senate::voting::*;
use aus_senate::util::*;
use aus_senate::parse::*;

/// Group voting ticket description. Maps states to ticket names to preference lists.
type GVT = HashMap<String, HashMap<String, Vec<CandidateId>>>;

/// Below the line voting map. Maps (batch, paper) pairs to preferences.
type BelowTheLine = HashMap<(u32, u32), PrefMap>;

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
            new_ticket_map.insert(ticket, pref_map_to_vec(pref_map));
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


#[derive(RustcDecodable, Debug)]
struct BTLRow {
    candidate_id: u32,
    preference: Option<u32>,
    batch: u32,
    paper: u32
}

fn parse_btl_votes<R: Read>(input: R) -> Result<BelowTheLine, Box<Error>> {
    let mut btl_votes = HashMap::new();
    let mut invalid_votes = HashSet::new();
    let mut reader = csv::Reader::from_reader(input);

    for raw_row in reader.decode::<BTLRow>() {
        let row = try!(raw_row);
        let vote_id = (row.batch, row.paper);
        match row.preference {
            Some(pref) => {
                let voter_prefs = btl_votes.entry(vote_id).or_insert_with(HashMap::new);
                let prev = voter_prefs.insert(row.candidate_id, pref);
                assert!(prev.is_none());
            }
            None => {
                invalid_votes.insert(vote_id);
            }
        }
    }

    // Remove invalid votes.
    println!("Invalid BTL votes: {}", invalid_votes.len());
    for vote_id in invalid_votes {
        btl_votes.remove(&vote_id);
    }
    println!("Valid BTL votes: {}", btl_votes.len());

    Ok(btl_votes)
}

// FIXME: use iterators instead
fn create_gvt_ballot_list(gvt: &GVT, gvt_usage: &GVTUsage, state: &str) -> Vec<IOBallot> {
    gvt_usage[state]
        .iter()
        // If the vote count is 0, then we can safely skip adding this bit of GVT usage.
        // The AEC files are strange in that some groups are included in the GVT usage with
        // with a count of 0, but absent are from the actual GVT description.
        .filter(|&(_, &vote_count)| vote_count != 0)
        // We then create a ballot with the right list of preferences from the GVT description.
        .map(|(group, &vote_count)| Ok(MultiBallot::multi(vote_count, gvt[state][group].clone())))
        .collect()
}

// TODO: Use this parser for 2016 candidate files as well.
#[derive(RustcDecodable, Debug)]
struct CandidateRow {
    state_ab: String,
    party_ab: String,
    party_name: String,
    candidate_id: CandidateId,
    surname: String,
    given_names: String,
    elected: String,
    historic_elected: String,
}

fn parse_candidates_file<R: Read>(input: R) -> Result<Vec<Candidate>, Box<Error>> {
    let mut result = vec![];
    let mut reader = csv::Reader::from_reader(input);

    for raw_row in reader.decode::<CandidateRow>() {
        let row = try!(raw_row);
        result.push(Candidate {
            id: row.candidate_id,
            surname: row.surname,
            other_names: row.given_names,
            group_name: row.party_ab,
            party: row.party_name,
            state: row.state_ab,
        });
    }

    Ok(result)
}

fn main_with_result() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        println!("Usage: ./election2013 <candidates file> <gvt file> <gvt usage file> <btl votes> <state>");
        try!(Err("invalid command line arguments.".to_string()));
    }

    let candidates_file_name = &args[1];
    let gvt_file_name = &args[2];
    let gvt_usage_file_name = &args[3];
    let btl_file_name = &args[4];
    let state = &args[5];

    let gvt_file = try!(open_aec_csv(gvt_file_name));
    let gvt = try!(parse_gvt(gvt_file));
    let gvt_usage_file = try!(open_aec_csv(gvt_usage_file_name));
    let gvt_usage = try!(parse_gvt_usage(gvt_usage_file));

    let btl_file = try!(open_aec_csv(btl_file_name));
    let btl_votes = try!(parse_btl_votes(btl_file));

    let candidates_file = try!(open_aec_csv(candidates_file_name));
    let all_candidates = try!(parse_candidates_file(candidates_file));

    let candidates = get_state_candidates(&all_candidates, state);

    // Construct the initial list of ballots according to the GVT.
    let mut ballots = create_gvt_ballot_list(&gvt, &gvt_usage, state);

    // Then extend it with the below the line votes.
    // TODO: Make this a lazy iterator like for 2016.
    ballots.extend(btl_votes.into_iter().map(|(_, pref_map)| {
        Ok(MultiBallot::single(pref_map_to_vec(pref_map)))
    }));

    let result = try!(decide_election(&candidates, ballots, 6));

    for s in result.senators.iter() {
        println!("Elected: {} {} ({})", s.other_names, s.surname, s.party);
    }

    if result.tied {
        println!("Those last two tied for the last seat.");
    }

    Ok(())
}

fn main() {
    if let Err(e) = main_with_result() {
        println!("Error: {:?}", e);
    }
}
