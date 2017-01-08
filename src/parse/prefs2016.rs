use csv;
use group::Group;
use super::prelude::*;

#[derive(RustcDecodable, Debug)]
pub struct PrefRow {
    electorate_name: String,
    vote_collection_point: String,
    vote_collection_point_id: String,
    batch_num: String,
    paper_num: String,
    preferences: String,
}

pub fn parse_single_ballot(raw_row: csv::Result<PrefRow>, groups: &[Group], candidates: &[CandidateId], constraints: &Constraints) -> IOBallot {
    match raw_row {
        Ok(row) => parse_ballot_str(&row.preferences, groups, candidates, constraints),
        Err(e) => Err(InputError(From::from(e))),
    }
}

// NOTE: This macro is to avoid writing the iterator type.
// One day it may be possible to use `impl Trait`, but at the moment the compiler
// doesn't understand that the ballots being returned don't depend on the lifetime
// of the CSV reader's data at all (due to allocations/cloning).
// See: https://gist.github.com/michaelsproul/20e18f52fc1be2bd05b2767ab6bd166c
#[macro_export]
macro_rules! parse_preferences_file {
    ($reader:expr, $groups:expr, $candidates:expr, $constraints:expr) => {{
        use $crate::parse::prefs2016::{PrefRow, parse_single_ballot};
        $reader
            .decode::<PrefRow>()
            .map(|raw_row| parse_single_ballot(raw_row, $groups, $candidates, $constraints))
    }}
}
