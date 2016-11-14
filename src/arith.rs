use candidate::*;
use ballot::*;
use util::*;
use vote_map::*;
use memo::*;

use std::collections::BTreeMap;

pub type TransferMap<'a> = HashMap<CandidateId, Vec<&'a mut Ballot<Frac>>>;

pub fn group_by_candidate<'a, 'b>(vote_map: &VoteMap<'b>, ballots: Vec<&'a mut Ballot<Frac>>, transfer_value: Option<&Frac>) -> TransferMap<'a> {
    let mut map = HashMap::new();

    let mut memo = MultMemo::new(transfer_value);

    for ballot in ballots {
        if let Some(i) = vote_map.find_next_valid_preference(&ballot) {
            ballot.current = i;

            if let Some(update) = memo.mult(ballot.weight.as_ref()) {
                ballot.weight = Some(update);
            }

            let candidate = ballot.prefs[ballot.current];

            let mut bucket = map.entry(candidate).or_insert_with(Vec::new);
            bucket.push(ballot);
        }
    }

    info!("# of multiplication hits: {}", memo.hits);

    map
}

pub fn compute_tallies<'a>(tmap: &TransferMap<'a>) -> HashMap<CandidateId, Frac> {
    tmap.iter()
        .map(|(&k, v)| (k, compute_single_tally(&v)))
        .collect()
}

// Compute the tally change for a single candidate using some clever tricks.
fn compute_single_tally<'a>(ballots: &[&'a mut Ballot<Frac>]) -> Frac {
    // Group by the number of occurrences of each weighting.
    let mut freq_map: BTreeMap<&Frac, u32> = BTreeMap::new();

    let mut ones = 0;

    for ballot in ballots {
        match ballot.weight {
            Some(ref weight) => {
                let mut freq = freq_map.entry(weight).or_insert(0);
                *freq += 1;
            }
            None => {
                ones += 1;
            }
        }
    }

    let sum = freq_map.into_iter().fold(frac!(0), |acc, (weight, count)| {
        acc + weight * frac!(count)
    });

    let result = sum + frac!(ones);
    //result.normalize();
    result
}
