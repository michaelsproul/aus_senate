extern crate aus_senate;
extern crate csv;

use std::env;
use std::error::Error;

use aus_senate::candidate::*;
use aus_senate::ballot::*;
use aus_senate::voting::*;
use aus_senate::util::*;
use aus_senate::ballot_parse::*;
use aus_senate::parse::*;
use aus_senate::parse::gvt2013::GVT;
use aus_senate::parse::gvt_usage2013::GVTUsage;

// FIXME: use iterators instead
fn create_gvt_ballot_list(gvt: &GVT, gvt_usage: &GVTUsage, state: &str) -> Vec<IOBallot> {
    gvt_usage[state]
        .iter()
        // If the vote count is 0, then we can safely skip adding this bit of GVT usage.
        // The AEC files are strange in that some groups are included in the GVT usage with
        // with a count of 0, but absent are from the actual GVT description.
        .filter(|&(_, &vote_count)| vote_count != 0)
        // We then create a ballot with the right list of preferences from the GVT description.
        .map(|(group, &vote_count)| Ok(Ballot::multi(vote_count, gvt[state][group].clone())))
        .collect()
}

fn main_with_result() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        println!(
            "Usage: ./election2013 <candidates file> <gvt file> <gvt usage file> \
             <btl votes> <state>"
        );
        Err("invalid command line arguments.".to_string())?
    }

    let candidates_file_name = &args[1];
    let gvt_file_name = &args[2];
    let gvt_usage_file_name = &args[3];
    let btl_file_name = &args[4];
    let state = &args[5];

    let gvt_file = open_aec_csv(gvt_file_name)?;
    let gvt = gvt2013::parse(gvt_file)?;
    let gvt_usage_file = open_aec_csv(gvt_usage_file_name)?;
    let gvt_usage = gvt_usage2013::parse(gvt_usage_file)?;

    let btl_file = open_aec_csv(btl_file_name)?;
    let btl_votes = btl2013::parse(btl_file)?;

    let candidates_file = open_aec_csv(candidates_file_name)?;
    let all_candidates = candidates2013::parse(candidates_file)?;

    let candidates = get_state_candidates(&all_candidates, state);

    // Construct the initial list of ballots according to the GVT.
    let mut ballots = create_gvt_ballot_list(&gvt, &gvt_usage, state);

    // Then extend it with the below the line votes.
    ballots.extend(btl_votes.into_iter().map(|(_, pref_map)| {
        Ok(Ballot::single(flatten_pref_map(pref_map)))
    }));

    let result = decide_election(&candidates, &[], ballots, 6)?;

    for &(ref s, _) in &result.senators {
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
