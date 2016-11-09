use ballot::*;
use candidate::*;
use util::*;

lazy_static! {
    static ref ONE: Frac = frac!(1);
}

/// Intermediate data structure mapping candidates to ballots.
pub struct VoteMap<'a> {
    pub tally: HashMap<CandidateId, Frac>,
    pub map: HashMap<CandidateId, Vec<&'a mut Ballot<Frac>>>
}

impl <'a> VoteMap<'a> {
    pub fn new(candidates: &CandidateMap) -> Result<VoteMap<'a>, String> {
        let mut v = VoteMap {
            tally: HashMap::new(),
            map: HashMap::new(),
        };
        for &id in candidates.keys() {
            let e1 = v.tally.insert(id, frac!(0));
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
    pub fn add(&mut self, ballot: &'a mut Ballot<Frac>) {
        let candidate = ballot.prefs[ballot.current];

        // Add to the candidate's tally.
        let mut vote_count = self.tally.get_mut(&candidate).expect("Candidate not found");
        *vote_count += ballot.weight.as_ref().unwrap_or_else(|| &ONE);

        // Add the ballot to the candidate's bucket.
        let mut bucket = self.map.get_mut(&candidate).unwrap();
        bucket.push(ballot);
    }

    /// Get the ID of a candidate whose vote exceeds the given quota.
    pub fn get_candidate_with_quota(&self, quota: &Frac) -> Option<CandidateId> {
        self.tally.iter().filter(|&(_, votes)| votes >= quota).map(|(&c, _)| c).next()
    }

    /// Get the ID of the candidate with the least votes.
    pub fn get_last_candidate(&self) -> CandidateId {
        self.tally.iter().min_by_key(|&(_, v)| v).map(|(&c, _)| c).unwrap()
    }

    pub fn find_next_valid_preference(&self, b: &Ballot<Frac>) -> Option<usize> {
        for (i, cand) in b.prefs[b.current .. ].iter().enumerate() {
            if self.tally.get(cand).is_some() {
                return Some(b.current + i);
            }
        }
        None
    }

    /// Redistribute the votes for the given candidate.
    /// Transfer value should be 1 if the candidate is being knocked out,
    /// or the (vote - quota) / (vote) if the candidate has been elected.
    fn redistribute_votes(&mut self, candidate: CandidateId, transfer_value: Option<&Frac>) {
        let ballots = self.map.remove(&candidate).unwrap();
        self.tally.remove(&candidate).unwrap();

        for ballot in ballots {
            if let Some(i) = self.find_next_valid_preference(&ballot) {
                ballot.current = i;
                ballot.apply_weighting(&transfer_value);
                self.add(ballot);
            }
        }
    }

    pub fn elect_candidate(&mut self, candidate: CandidateId, quota: &Frac) {
        let transfer_value = {
            let num_votes = &self.tally[&candidate];
            (num_votes - quota) / num_votes
        };
        trace!("Transferring at value: {}", transfer_value);
        self.redistribute_votes(candidate, Some(&transfer_value))
    }

    pub fn knock_out_candidate(&mut self, candidate: CandidateId) {
        self.redistribute_votes(candidate, None)
    }
}
