use std::error::Error;
use std::collections::VecDeque;

use util::*;
use candidate::*;
use vote_map::*;
use ballot_parse::*;
use senate_result::*;

pub fn compute_quota(num_votes: u32, num_positions: usize) -> Int {
    frac!(num_votes, num_positions + 1).ceil()
}

fn elect_candidates<'a, 'b: 'a>(elected: Vec<CandidateElected<'a>>,
                        result: &mut Senate<'b>,
                        preference_transfers: &mut VecDeque<PreferenceTransfer<'a>>,
                        candidates: &'b CandidateMap) -> () {
    for CandidateElected { id, votes, transfers } in elected {
        trace!("Elected {:?} with {:?} votes", candidates[&id], votes);
        result.add_senator(id, votes, candidates);
        preference_transfers.extend(transfers);
    }
}

fn exclude_candidates<'a, 'b: 'a>(excluded: Vec<CandidateExcluded<'a>>,
                        preference_transfers: &mut VecDeque<PreferenceTransfer<'a>>,
                        candidates: &'b CandidateMap) -> () {
    for CandidateExcluded { id, transfers } in excluded {
        info!("Excluded {:?}", candidates[&id]);
        preference_transfers.extend(transfers);
    }
}

pub fn decide_election<'a, I>(candidates: &'a CandidateMap, ballot_stream: I, num_positions: usize)
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
    let mut vote_map = VoteMap::new(candidates)?;

    // Allocate first preference votes.
    for ballot_ref in ballots.iter_mut() {
        vote_map.add(ballot_ref);
    }

    let quota = compute_quota(result.stats.num_valid_votes(), num_positions);

    let mut preference_transfers = VecDeque::new();

    info!("Count #1");
    let elected_on_first_prefs = vote_map.elect_candidates_with_quota(&quota);
    elect_candidates(elected_on_first_prefs, &mut result, &mut preference_transfers, candidates);

    for i in 2.. {
        info!("Count #{}", i);

        // Transfer pending preferences.
        let transfer = preference_transfers.pop_front()
            .expect("election should terminate before running out of preferences to transfer");

        trace!("Transferring preferences for {:?} at value {:?}",
            candidates[&transfer.0], transfer.1
        );
        vote_map.transfer_preferences(transfer);

        // Elect any candidates with a full quota, and stage their preference transfers.
        let elected = vote_map.elect_candidates_with_quota(&quota);
        elect_candidates(elected, &mut result, &mut preference_transfers, candidates);

        if preference_transfers.is_empty() {
            // If the number of candidates remaining is equal to the number of positions, elect
            // them all.
            let positions_remaining = num_positions - result.num_elected();
            if vote_map.num_candidates_remaining() == positions_remaining {
                let remaining = vote_map.elect_remaining();
                elect_candidates(remaining, &mut result, &mut preference_transfers, candidates);
                break;
            }

            // Exclude some candidates if we've run out of things to do.
            let excluded = vote_map.exclude_candidates();
            exclude_candidates(excluded, &mut preference_transfers, candidates);
        }

        vote_map.print_summary();
    }

    assert_eq!(result.num_elected(), num_positions);

    Ok(result)
}
