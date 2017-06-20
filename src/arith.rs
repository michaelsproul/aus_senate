use ballot::Ballot;
use candidate::*;
use util::*;
use vote_map::*;

/// Given a list of ballots, group them by next valid candidate.
pub fn group_ballots_by_candidate<'a>(
    vote_map: &VoteMap<'a>,
    ballots: Vec<&'a mut Ballot>,
) -> HashMap<CandidateId, Vec<&'a mut Ballot>> {
    let mut map = HashMap::new();

    for ballot in ballots {
        if let Some(i) = vote_map.find_next_valid_preference(ballot) {
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
    let value = weight * frac!(num_ballots);
    value.floor()
}
