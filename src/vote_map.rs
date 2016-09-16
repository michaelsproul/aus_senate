use ballot::*;
use candidate::*;
use util::*;

/// The value of a ballot is given by a rational number, and we store ballots bucketed by candidate.
pub type BallotMap = HashMap<Ballot, Frac>;

/// Intermediate data structure mapping candidates to ballots.
pub struct VoteMap {
    pub tally: HashMap<CandidateId, Frac>,
    pub map: HashMap<CandidateId, BallotMap>
}

impl VoteMap {
    pub fn new(candidates: &CandidateMap) -> Result<VoteMap, String> {
        let mut v = VoteMap {
            tally: HashMap::new(),
            map: HashMap::new(),
        };
        for &id in candidates.keys() {
            let e1 = v.tally.insert(id, frac!(0));
            let e2 = v.map.insert(id, HashMap::new());
            if e1.is_some() || e2.is_some() {
                return Err(format!("Candidate ID {} appears more than once", id));
            }
        }
        debug_assert!(v.map.len() == candidates.len());
        debug_assert!(v.tally.len() == candidates.len());
        Ok(v)
    }

    /// Add votes to a candidate's tally according to the weight and current preference of a ballot.
    pub fn add(&mut self, ballot: Ballot, weight: &Frac) {
        let candidate = ballot.prefs[ballot.current];
        // Add to the candidate's tally.
        let mut vote_count = self.tally.get_mut(&candidate).expect(&format!("Candidate not found: {}", candidate));
        *vote_count = &*vote_count + weight;
        // Add the ballot to the candidate's bucket.
        let mut bucket = self.map.get_mut(&candidate).unwrap();
        let mut ballot_value = bucket.entry(ballot).or_insert_with(|| frac!(0));
        *ballot_value = &*ballot_value + weight;
    }

    pub fn add_multi_ballot(&mut self, MultiBallot { ballot, value }: MultiBallot) {
        self.add(ballot, &frac!(value));
    }

    /// Get the ID of a candidate whose vote exceeds the given quota.
    pub fn get_candidate_with_quota(&self, quota: &Frac) -> Option<CandidateId> {
        self.tally.iter().filter(|&(_, votes)| votes >= quota).map(|(&c, _)| c).next()
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

    /// Redistribute the votes for the given candidate.
    /// Transfer value should be 1 if the candidate is being knocked out,
    /// or the (vote - quota) / (vote) if the candidate has been elected.
    fn redistribute_votes(&mut self, candidate: CandidateId, transfer_value: &Frac)
    -> Vec<Ballot>
    {
        let ballots = self.map.remove(&candidate).unwrap();
        self.tally.remove(&candidate).unwrap();

        let mut exhausted_ballots = vec![];

        for (mut ballot, value) in ballots {
            match self.find_next_valid_preference(&ballot) {
                Some(i) => {
                    ballot.current = i;
                    let new_value = value * transfer_value;
                    self.add(ballot, &new_value);
                }
                None => {
                    exhausted_ballots.push(ballot);
                }
            }
        }

        exhausted_ballots
    }

    pub fn elect_candidate(&mut self, candidate: CandidateId, quota: &Frac) -> Vec<Ballot> {
        let transfer_value = {
            let num_votes = &self.tally[&candidate];
            (num_votes - quota) / num_votes
        };
        println!("Transferring at value: {}", transfer_value);
        self.redistribute_votes(candidate, &transfer_value)
    }

    pub fn knock_out_candidate(&mut self, candidate: CandidateId) -> Vec<Ballot> {
        self.redistribute_votes(candidate, &frac!(1))
    }
}
