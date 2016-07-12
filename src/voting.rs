use std::cmp::Ordering::*;

use ballot::*;
use util::*;
use quota::*;
use vote_map::*;

use self::Senate::*;

pub fn compute_num_votes(ballots: &[Ballot]) -> Frac {
    ballots.iter().fold(frac!(0), |acc, b| acc + &b.weight)
}

#[derive(Debug)]
pub enum Senate {
    Regular(Vec<CandidateId>),
    Tied(Vec<CandidateId>, CandidateId, CandidateId)
}

pub fn decide_election(candidates: &[CandidateId], ballots: Vec<Ballot>, num_candidates: u32) -> Result<Senate, String> {
    let num_votes = compute_num_votes(&ballots);
    let quota = senate_quota(num_votes, num_candidates);

    // TODO: Sanity check for all preferences (to make various unwraps safe).

    // Map from candidate IDs to numbers of votes.
    let mut vote_map = try!(VoteMap::new(candidates));

    // Allocate first preference votes.
    for ballot in ballots {
        vote_map.add(ballot);
    }

    // List of elected candidates.
    let mut elected_candidates = vec![];
    // List of exhausted ballots.
    let mut exhausted_votes = vec![];

    // Stage 1: Elect all candidates with a full quota.
    while let Some(candidate) = vote_map.get_candidate_with_quota(&quota) {
        elected_candidates.push(candidate);
        let mut exhausted = vote_map.elect_candidate(candidate, &quota);
        exhausted_votes.append(&mut exhausted);
    }

    // Stage 2: Winnow out the shithouse candidates until we've elected enough
    // candidates based on preferences, OR reached only two candidates.
    while elected_candidates.len() < num_candidates as usize {
        assert!(vote_map.tally.len() >= 2);
        // If only two candidates remain, try to elect the one with the majority.
        if vote_map.tally.len() == 2 {
            let last_two: Vec<_> = vote_map.tally.drain().collect();
            let (c1, ref v1) = last_two[0];
            let (c2, ref v2) = last_two[1];
            let winner = match Ord::cmp(v1, v2) {
                Equal => {
                    return Ok(Tied(elected_candidates, c1, c2));
                }
                Greater => c1,
                Less => c2,
            };
            elected_candidates.push(winner);
            break;
        }

        let last_candidate = vote_map.get_last_candidate();
        let mut ex = vote_map.knock_out_candidate(last_candidate);
        exhausted_votes.append(&mut ex);

        // If there is now a candidate with a full quota, elect them!
        if let Some(candidate) = vote_map.get_candidate_with_quota(&quota) {
            elected_candidates.push(candidate);
            let mut ex = vote_map.elect_candidate(candidate, &quota);
            exhausted_votes.append(&mut ex);
        }
    }
    assert_eq!(elected_candidates.len(), num_candidates as usize);

    Ok(Regular(elected_candidates))
}
