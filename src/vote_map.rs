use ballot::*;
use candidate::*;
use util::*;
use arith::*;
use transfer_value::*;

/// Intermediate data structure mapping candidates to ballots.
pub struct VoteMap<'a> {
    pub tally: HashMap<CandidateId, u32>,
    pub map: HashMap<CandidateId, Vec<&'a mut Ballot>>
}

impl <'a> VoteMap<'a> {
    pub fn new(candidates: &CandidateMap) -> Result<VoteMap<'a>, String> {
        let mut v = VoteMap {
            tally: HashMap::new(),
            map: HashMap::new(),
        };
        for &id in candidates.keys() {
            let e1 = v.tally.insert(id, 0);
            let e2 = v.map.insert(id, vec![]);
            if e1.is_some() || e2.is_some() {
                return Err(format!("Candidate ID {} appears more than once", id));
            }
        }
        debug_assert!(v.map.len() == candidates.len());
        debug_assert!(v.tally.len() == candidates.len());
        Ok(v)
    }

    /// Add votes to a candidate's tally according to the weight and current preference of a ballot.
    pub fn add(&mut self, ballot: &'a mut Ballot) {
        let candidate = ballot.prefs[ballot.current];

        // Add to the candidate's tally.
        let mut vote_count = self.tally.get_mut(&candidate).expect("Candidate not found");
        *vote_count += ballot.weight;

        // Add the ballot to the candidate's bucket.
        let mut bucket = self.map.get_mut(&candidate).unwrap();
        bucket.push(ballot);
    }

    /// Get the ID of a candidate whose vote exceeds the given quota.
    pub fn get_candidate_with_quota(&self, quota: u32) -> Option<CandidateId> {
        self.tally.iter().filter(|&(_, &votes)| votes >= quota).map(|(&c, _)| c).next()
    }

    /// Get the ID of the candidate with the least votes.
    pub fn get_last_candidate(&self) -> CandidateId {
        self.tally.iter().min_by_key(|&(_, v)| v).map(|(&c, _)| c).unwrap()
    }

    pub fn find_next_valid_preference(&self, b: &Ballot) -> Option<usize> {
        for (i, cand) in b.prefs[b.current .. ].iter().enumerate() {
            if self.tally.get(cand).is_some() {
                return Some(b.current + i);
            }
        }
        None
    }

    fn redistribute_votes(&mut self, candidate: CandidateId, transfer_value: Option<TransferValue>) {
        let ballots = self.map.remove(&candidate).unwrap();
        self.tally.remove(&candidate).unwrap();

        let grouped_ballots = group_by_candidate(&*self, ballots);
        let tallies = compute_tallies(&grouped_ballots, transfer_value);

        for (candidate, ballots) in grouped_ballots {
            let mut bucket = self.map.get_mut(&candidate).unwrap();
            bucket.extend(ballots);
        }

        for (candidate, vote_update) in tallies {
            let mut vote_count = self.tally.get_mut(&candidate).unwrap();
            *vote_count += vote_update;
        }
    }

    pub fn elect_candidate(&mut self, candidate: CandidateId, quota: u32) {
        let mut transfer_value = {
            let num_votes = self.tally[&candidate];
            TransferValue::new(num_votes - quota, num_votes)
        };
        trace!("Transferring at value: {:?}", transfer_value);
        self.redistribute_votes(candidate, Some(transfer_value))
    }

    pub fn knock_out_candidate(&mut self, candidate: CandidateId) {
        self.redistribute_votes(candidate, None)
    }
}
