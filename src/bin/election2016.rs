#[macro_use]
extern crate aus_senate;
extern crate csv;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use std::error::Error;
use std::fs::File;

use aus_senate::ballot_parse::*;
use aus_senate::candidate::*;
use aus_senate::group::*;
use aus_senate::parse::candidates2016;
use aus_senate::voting::*;

fn main_with_result() -> Result<(), Box<Error>> {
    env_logger::init()?;

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 && args.len() != 5 {
        println!("Usage: ./election2016 <candidates file> <prefs file> <state> [num candidates]");
        Err("invalid command line arguments.".to_string())?;
    }

    let candidates_file_name = &args[1];
    let prefs_file_name = &args[2];
    let state = &args[3];
    let num_candidates = match args.get(4) {
        Some(x) => x.parse::<usize>()?,
        None => 12,
    };

    let candidates_file = File::open(candidates_file_name)?;
    let all_candidates = candidates2016::parse(candidates_file)?;

    for c in &all_candidates {
        debug!("{}: {} {} ({})", c.id, c.other_names, c.surname, c.party);
    }

    // Extract candidate and group information from the complete list of ballots.
    let candidates = get_state_candidates(&all_candidates, state);
    let candidate_ids = get_candidate_id_list(&all_candidates, state);
    let groups = get_group_list(&all_candidates, state);

    let constraints = Constraints::official();

    debug!("Num groups: {}", groups.len());
    trace!("Groups: {:#?}", groups);

    let prefs_file = File::open(prefs_file_name)?;

    let mut csv_reader = csv::ReaderBuilder::new()
        .comment(Some('-' as u8))
        .from_reader(prefs_file);
    let ballots_iter = parse_preferences_file!(csv_reader, &groups, &candidate_ids, &constraints);

    let election_result = decide_election(&candidates, &[], ballots_iter, num_candidates)?;

    println!("=== Elected ===");
    for &(ref c, ref votes) in &election_result.senators {
        println!(
            "{} {} ({}) [{} votes]",
            c.other_names, c.surname, c.party, votes
        );
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
