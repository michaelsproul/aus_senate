use std::cmp::Ordering::*;

use util::*;
use candidate::*;
use ballot::*;
use vote_map::*;

#[derive(Debug)]
pub struct Senate<'a> {
    pub senators: Vec<&'a Candidate>,
    pub tied: bool,
}

impl<'a> Senate<'a> {
    pub fn new() -> Senate<'a> {
        Senate {
            senators: vec![],
            tied: false,
        }
    }

    pub fn add_senator(&mut self, id: CandidateId, candidates: &'a CandidateMap) {
        self.senators.push(&candidates[&id])
    }

    pub fn num_elected(&self) -> usize {
        self.senators.len()
    }
}

pub fn compute_quota(num_votes: u32, num_senators: u32) -> Frac {
    (frac!(num_votes) / frac!(num_senators + 1)).floor() + frac!(1)
}

pub fn decide_election<'a>(candidates: &'a CandidateMap, ballots: Vec<Ballot>, num_votes: u32, num_candidates: u32) -> Result<Senate<'a>, String> {
    let quota = compute_quota(num_votes, num_candidates);
    println!("Senate quota is: {}", quota);

    // TODO: Sanity check for all preferences (to make various unwraps safe).

    // Map from candidate IDs to numbers of votes.
    let mut vote_map = try!(VoteMap::new(candidates));

    // Allocate first preference votes.
    for ballot in ballots {
        vote_map.add(ballot);
    }

    // List of elected candidates, as a Senate struct.
    let mut result = Senate::new();
    // List of exhausted ballots.
    let mut exhausted_votes = vec![];

    // Stage 1: Elect all candidates with a full quota.
    while let Some(id) = vote_map.get_candidate_with_quota(&quota) {
        println!("Elected candidate {:?} in the first round of voting", candidates[&id]);
        result.add_senator(id, candidates);
        let mut exhausted = vote_map.elect_candidate(id, &quota);
        exhausted_votes.append(&mut exhausted);
    }

    // Stage 2: Winnow out the shithouse candidates until we've elected enough
    // candidates based on preferences, OR reached only two candidates.
    while result.num_elected() < num_candidates as usize {
        assert!(vote_map.tally.len() >= 2);
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
        println!("Eliminating candidate: {}, candidates remaining: {}", last_candidate, vote_map.tally.len());
        let mut ex = vote_map.knock_out_candidate(last_candidate);
        exhausted_votes.append(&mut ex);

        // If there is now a candidate with a full quota, elect them!
        if let Some(candidate) = vote_map.get_candidate_with_quota(&quota) {
            println!("Electing candidate: {}", candidate);
            result.add_senator(candidate, candidates);
            let mut ex = vote_map.elect_candidate(candidate, &quota);
            exhausted_votes.append(&mut ex);
        }
    }
    assert_eq!(result.num_elected(), num_candidates as usize);

    Ok(result)
}
