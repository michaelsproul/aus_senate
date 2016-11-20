use std::cmp::Ordering::*;
use std::error::Error;

use util::*;
use candidate::*;
use vote_map::*;
use ballot_parse::*;
use senate_result::*;

pub fn compute_quota(num_votes: u32, num_senators: u32) -> Int {
    frac!(num_votes, num_senators + 1).ceil()
}

pub fn decide_election<'a, I>(candidates: &'a CandidateMap, ballot_stream: I, num_candidates: u32)
    -> Result<Senate<'a>, Box<Error>>
    where I: IntoIterator<Item=IOBallot>
{
    // TODO: Sanity check for all preferences (to make various unwraps safe).

    // List of elected candidates, as well as algorithm statistics.
    let mut result = Senate::new();

    // Vector of owned ballots.
    let mut ballots = vec![];

    for maybe_ballot in ballot_stream {
        match maybe_ballot {
            Ok(ballot) => {
                result.stats.record_valid_vote(&ballot);
                ballots.push(ballot);
            }
            Err(InvalidBallot(err)) => {
                // TODO: make ballot parsing errors a hard failure.
                result.stats.record_invalid_vote(err);
            }
            Err(InputError(e)) => {
                return Err(e);
            }
        };
    }

    // Map from candidate IDs to numbers of votes.
    let mut vote_map = try!(VoteMap::new(candidates));

    // Allocate first preference votes.
    for ballot_ref in ballots.iter_mut() {
        vote_map.add(ballot_ref);
    }

    let quota = compute_quota(result.stats.num_valid_votes(), num_candidates);

    // Stage 1: Elect all candidates with a full quota.
    while let Some(id) = vote_map.get_candidate_with_quota(&quota) {
        info!("Elected candidate {:?} in the first round of voting", candidates[&id]);
        result.add_senator(id, candidates);
        vote_map.elect_candidate(id, &quota);
    }

    // Stage 2: Winnow out the shithouse candidates until we've elected enough
    // candidates based on preferences, OR reached only two candidates.
    while result.num_elected() < num_candidates as usize {
        let positions_remaining = num_candidates as usize - result.num_elected();
        // If there is some number of candidates still to be elected, and all other
        // candidates have been eliminated, then elect all the remaining candidates.
        if vote_map.tally.len() == positions_remaining {
            for (id, _) in vote_map.tally.drain() {
                result.add_senator(id, candidates);
            }
            break;
        }

        // Otherwise, if there are 2 candidates remaining and only 1 left to be elected,
        // try to elect the candidate with the majority.
        if vote_map.tally.len() == 2 {
            assert_eq!(positions_remaining, 1);
            let last_two: Vec<_> = vote_map.tally.drain().collect();
            let (c1, ref v1) = last_two[0];
            let (c2, ref v2) = last_two[1];
            let winner = match Ord::cmp(v1, v2) {
                Equal => {
                    result.tied = true;
                    result.add_senator(c1, candidates);
                    result.add_senator(c2, candidates);
                    return Ok(result);
                }
                Greater => c1,
                Less => c2,
            };
            result.add_senator(winner, candidates);
            break;
        }

        let last_candidate = vote_map.get_last_candidate();
        info!("Eliminating candidate: {}, candidates remaining: {}", last_candidate, vote_map.tally.len());
        vote_map.knock_out_candidate(last_candidate);

        // If there is now a candidate with a full quota, elect them!
        if let Some(candidate) = vote_map.get_candidate_with_quota(&quota) {
            info!("Electing candidate: {}", candidate);
            result.add_senator(candidate, candidates);
            vote_map.elect_candidate(candidate, &quota);
        }
    }
    assert_eq!(result.num_elected(), num_candidates as usize);

    Ok(result)
}
