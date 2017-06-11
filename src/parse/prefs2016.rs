use csv;
use group::Group;
use super::prelude::*;

#[derive(Deserialize, Debug)]
pub struct PrefRow {
    #[serde(rename = "ElectorateNm")]
    electorate_name: String,
    #[serde(rename = "VoteCollectionPointNm")]
    vote_collection_point: String,
    #[serde(rename = "VoteCollectionPointId")]
    vote_collection_point_id: String,
    #[serde(rename = "BatchNo")]
    batch_num: String,
    #[serde(rename = "PaperNo")]
    paper_num: String,
    #[serde(rename = "Preferences")]
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
            .deserialize::<PrefRow>()
            .map(|raw_row| parse_single_ballot(raw_row, $groups, $candidates, $constraints))
    }}
}
