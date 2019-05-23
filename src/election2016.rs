use std::error::Error;
use std::fs::File;

use ballot_parse::*;
use candidate::*;
use group::*;
use parse::candidates2016;
use senate_result::Senate;
use voting::*;

/// Parse ballots and compute the election outcome (2016 edition)
pub fn run(
    candidates_file_name: &str,
    prefs_file_name: &str,
    state: &str,
    num_candidates: usize,
) -> Result<Senate, Box<Error>> {
    let candidates_file = File::open(candidates_file_name)?;
    let all_candidates = candidates2016::parse(candidates_file)?;

    for c in &all_candidates {
        debug!("{}: {} {} ({})", c.id, c.other_names, c.surname, c.party);
    }

    // Extract candidate and group information from the complete list of candidates.
    let candidates = get_state_candidates(&all_candidates, state);
    let candidate_ids = get_candidate_id_list(&all_candidates, state);
    let groups = get_group_list(&all_candidates, state);

    let constraints = Constraints::official();

    debug!("Num groups: {}", groups.len());
    trace!("Groups: {:#?}", groups);

    let prefs_file = File::open(prefs_file_name)?;

    let mut csv_reader = csv::ReaderBuilder::new()
        .comment(Some(b'-'))
        .from_reader(prefs_file);
    let ballots_iter = parse_preferences_file!(csv_reader, &groups, &candidate_ids, &constraints);

    decide_election(&candidates, &[], ballots_iter, num_candidates)
}
