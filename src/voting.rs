use std::collections::HashMap;

use ballot::*;
use util::*;
use quota::*;

pub fn compute_num_votes(ballots: &[Ballot]) -> u32 {
    ballots.iter().fold(0, |acc, b| acc + b.count)
}

pub fn decide_election(candidates: &[CandidateId], ballots: Vec<Ballot>, num_candidates: u32) -> Result<Vec<CandidateId>, String> {
    let num_votes = compute_num_votes(&ballots);
    let quota = senate_quota(num_votes, num_candidates);

    // TODO: Sanity check for all preferences.

    // Map from candidate IDs to numbers of votes.
    let mut vote_map: HashMap<CandidateId, Frac> = HashMap::new();

    // Initialise all candidate votes to an initial value of zero.
    for &id in candidates.iter() {
        if let Some(_) = vote_map.insert(id, frac!(0)) {
            return Err(format!("Candidate ID {} appears more than once", id));
        }
    }
    debug_assert!(vote_map.len() == candidates.len());

    // Allocate first preference votes.
    for ballot in &ballots {
        let mut vote_count = vote_map.get_mut(&ballot.prefs[0]).unwrap();
        *vote_count = &*vote_count + frac!(ballot.count);
    }

    // If any candidate has a full quota, elect them.
    let mut elected_candidates = vec![];

    while elected_candidates.len() < num_candidates {
        for (&candidate, votes) in vote_map.iter_mut() {
            if *votes >= quota {
                elected_candidates.push(candidate);

                // Distribute preferences.
                let redistrib_weight = *votes - quota; // FIXME
                *votes = frac!(0);

                // Distribute prefs.
                let mut exhausted_votes = vec![];
                for (i, ballot) in ballots.iter_mut().enumerate() {
                    if ballot.prefs[ballot.current] == candidate {
                        // If the vote is exhausted, add it to the list to be removed.
                        if ballot.current == ballot.prefs.len() - 1 {
                            exhausted_votes.push(i);
                        }
                        // Otherwise, distribute the vote to its next preference.
                        else {
                            ballot.current += 1;
                        }
                    }
                }
            }
        }

        println!("{:#?}", vote_map);
    }

    Ok(elected_candidates)
}
