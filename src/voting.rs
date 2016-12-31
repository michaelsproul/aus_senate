use std::cmp::Ordering::*;
use std::error::Error;
use std::collections::VecDeque;

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
    let mut result = Senate::new();

    // Ingest ballots.
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

    let mut preference_transfers = VecDeque::new();

    trace!("Count #1");
    let elected_on_first_prefs = vote_map.elect_candidates_with_quota(&quota);

    for CandidateElected { id, votes, transfers } in elected_on_first_prefs {
        trace!("Elected {:?} with {:?} votes", candidates[&id], votes);
        result.add_senator(id, votes, candidates);
        preference_transfers.extend(transfers);
    }

    for i in 2.. {
        trace!("Count #{}", i);

        if let Some(transfer) = preference_transfers.pop_front() {
            trace!("Transferring preferences for {:?} at value {:?}",
                candidates[&transfer.0], transfer.1
            );
            vote_map.transfer_preferences(transfer);

            let elected = vote_map.elect_candidates_with_quota(&quota);
            // TODO: put this in a method.
            for CandidateElected { id, votes, transfers } in elected {
                trace!("Elected {:?} with {:?} votes", candidates[&id], votes);
                result.add_senator(id, votes, candidates);
                preference_transfers.extend(transfers);
            }

            // TODO: exclude some candidates if nobody is elected.
        } else {
            break;
        }
    }

    Ok(result)
}

pub fn decide_election_old<'a, I>(candidates: &'a CandidateMap, ballot_stream: I, num_candidates: u32)
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

    info!("Quota: {}", quota);

    // Stage 1: Elect all candidates with a full quota.
    let elected_on_first_prefs = vote_map.get_candidates_with_quota(&quota);

    // Elect them all.
    for &id in &elected_on_first_prefs {
        let votes = vote_map.get_tally(id);
        info!("Elected candidate {:?} in the first round of voting with {} votes",
            candidates[&id],
            votes
        );
        result.add_senator(id, votes, candidates);
    }

    // Distribute their preferences.
    for &id in &elected_on_first_prefs {
        vote_map.mark_eliminated(id);
    }
    for &id in &elected_on_first_prefs {
        vote_map.elect_candidate(id, &quota);
    }

    // Stage 2: Winnow out the shithouse candidates until we've elected enough
    // candidates based on preferences, OR reached only two candidates.
    while result.num_elected() < num_candidates as usize {
        let candidates_remaining = vote_map.num_candidates_remaining();
        let positions_remaining = num_candidates as usize - result.num_elected();
        // If there is some number of candidates still to be elected, and all other
        // candidates have been eliminated, then elect all the remaining candidates.
        if candidates_remaining == positions_remaining {
            for (id, votes) in vote_map.drain() {
                result.add_senator(id, votes, candidates);
            }
            break;
        }

        // Otherwise, if there are 2 candidates remaining and only 1 left to be elected,
        // try to elect the candidate with the majority.
        if candidates_remaining == 2 {
            assert_eq!(positions_remaining, 1);
            let mut last_two = vote_map.drain();
            let (c1, v1) = last_two.pop().unwrap();
            let (c2, v2) = last_two.pop().unwrap();
            let (winner, winner_votes) = match Ord::cmp(&v1, &v2) {
                Equal => {
                    result.tied = true;
                    result.add_senator(c1, v1, candidates);
                    result.add_senator(c2, v2, candidates);
                    return Ok(result);
                }
                Greater => (c1, v1),
                Less => (c2, v2),
            };
            result.add_senator(winner, winner_votes, candidates);
            break;
        }

        let last_candidate = vote_map.get_last_candidate();
        info!("Eliminating candidate: {}, candidates remaining: {}", last_candidate, candidates_remaining);
        vote_map.knock_out_candidate(last_candidate);

        // If there is now a candidate with a full quota, elect them!
        while let Some(candidate) = vote_map.get_candidate_with_quota(&quota) {
            let votes = vote_map.get_tally(candidate);
            info!("Electing candidate: {} with {} votes", candidate, votes);
            result.add_senator(candidate, votes, candidates);
            vote_map.elect_candidate(candidate, &quota);
        }
    }

    assert_eq!(result.num_elected(), num_candidates as usize);

    Ok(result)
}
