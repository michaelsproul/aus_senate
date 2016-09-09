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

// FIXME: This macro is to avoid writing the iterator type. Can use a function once impl Trait lands.
#[macro_export]
macro_rules! parse_preferences_file {
    ($reader:expr, $groups:expr, $candidates:expr, $constraints:expr) => {
        $reader
            .decode::<$crate::parse::prefs2016::PrefRow>()
            .map(|raw_row| $crate::parse::prefs2016::parse_single_ballot(raw_row, $groups, $candidates, $constraints))
    }
}
