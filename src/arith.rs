use candidate::*;
use ballot::*;
use util::*;
use vote_map::*;
use transfer_value::*;

use std::collections::BTreeMap;

pub type TransferMap<'a> = HashMap<CandidateId, Vec<&'a mut Ballot>>;

pub fn group_by_candidate<'a, 'b>(vote_map: &VoteMap<'b>, ballots: Vec<&'a mut Ballot>) -> TransferMap<'a> {
    let mut map = HashMap::new();

    for ballot in ballots {
        if let Some(i) = vote_map.find_next_valid_preference(&ballot) {
            ballot.current = i;

            let candidate = ballot.prefs[ballot.current];

            let mut bucket = map.entry(candidate).or_insert_with(Vec::new);
            bucket.push(ballot);
        }
    }

    map
}

pub fn compute_tallies<'a>(tmap: &TransferMap<'a>, tval: Option<TransferValue>) -> HashMap<CandidateId, u32> {
    tmap.iter()
        .map(|(&candidate, ballots)| (candidate, compute_single_tally(&ballots, &tval)))
        .collect()
}

// Compute the tally change for a single candidate using some clever tricks.
fn compute_single_tally<'a>(ballots: &[&'a mut Ballot], transfer_value: &Option<TransferValue>) -> u32 {
    let num_ballots = ballots.iter().map(|b| b.weight).sum();

    match *transfer_value {
        Some(ref transfer_value) => transfer_value.apply(num_ballots),
        None => num_ballots
    }
}
