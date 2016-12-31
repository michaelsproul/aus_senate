use ballot::Ballot;
use candidate::*;
use util::*;
use vote_map::*;

/// Given a list of ballots, group them by next valid candidate.
pub fn group_ballots_by_candidate<'a>(vote_map: &VoteMap<'a>, ballots: Vec<&'a mut Ballot>)
    -> HashMap<CandidateId, Vec<&'a mut Ballot>>
{
    let mut map = HashMap::new();

    for ballot in ballots {
        if let Some(i) = vote_map.find_next_valid_preference(&ballot) {
            ballot.current = i;

            let continuing_candidate = ballot.prefs[ballot.current];

            let bucket = map.entry(continuing_candidate).or_insert_with(Vec::new);
            bucket.push(ballot);
        }
    }

    map
}

/// Compute the value of a list of ballots at a given weight (transfer value).
pub fn ballot_value<'a>(weight: &Frac, ballots: &[&'a mut Ballot]) -> Int {
    let num_ballots: u32 = ballots.iter().map(|b| b.weight).sum();
    trace!("num ballots: {}", num_ballots);
    let value = weight * frac!(num_ballots);
    value.floor()
}

pub fn group_by_candidate<'a>(vote_map: &VoteMap<'a>, all_ballots: TransferMap<'a>, new_transfer_value: &Option<Frac>) -> BallotMap<'a> {
    let mut map = BallotMap::new();

    for (mut transfer_val, ballots) in all_ballots {
        // Multiply the old transfer value by the new one.
        if let &Some(ref ntv) = new_transfer_value {
            transfer_val = transfer_val * ntv;
        }

        // Allocate ballots per candidate, and bucket them by transfer value.
        for ballot in ballots {
            if let Some(i) = vote_map.find_next_valid_preference(&ballot) {
                ballot.current = i;

                let candidate = ballot.prefs[ballot.current];

                let mut transfer_map = map.entry(candidate).or_insert_with(TransferMap::new);

                // FIXME: this is inefficient, use Frac interning.
                let mut bucket = transfer_map.entry(transfer_val.clone()).or_insert_with(Vec::new);
                bucket.push(ballot);
            }
        }
    }

    map
}

pub fn compute_tallies<'a>(ballot_map: &BallotMap<'a>) -> HashMap<CandidateId, Int> {
    ballot_map.iter()
        .map(|(&k, v)| (k, compute_single_tally(v)))
        .collect()
}

pub fn compute_single_tally<'a>(transfer_map: &TransferMap<'a>) -> Int {
    let mut total = Int::from(0);

    for (transfer_val, ballots) in transfer_map {
        let num_ballots: u32 = ballots.iter().map(|b| b.weight).sum();
        let vote_update = transfer_val * frac!(num_ballots);
        total = total + vote_update.floor();
    }

    total
}
