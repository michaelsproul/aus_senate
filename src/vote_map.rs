use ballot::*;
use candidate::*;
use util::*;
use arith::*;

/*
lazy_static! {
    static ref ONE: Frac = frac!(1);
}
*/

/// Map from transfer values to ballots with that transfer value.
pub type TransferMap<'a> = BTreeMap<Frac, Vec<&'a mut Ballot>>;

/// Map from (candidate_id => (transfer_value => ballots)).
pub type BallotMap<'a> = HashMap<CandidateId, TransferMap<'a>>;

/// Intermediate data structure mapping candidates to ballots.
pub struct VoteMap<'a> {
    pub tally: HashMap<CandidateId, Frac>,
    pub map: BallotMap<'a>,
    pub one: Frac,
}

fn new_transfer_map<'a>(one: Frac) -> TransferMap<'a> {
    let mut map = TransferMap::new();
    map.insert(one, vec![]);
    map
}

impl <'a> VoteMap<'a> {
    pub fn new(candidates: &CandidateMap) -> Result<VoteMap<'a>, String> {
        let mut v = VoteMap {
            tally: HashMap::new(),
            map: HashMap::new(),
            one: frac!(1),
        };
        for &id in candidates.keys() {
            let e1 = v.tally.insert(id, frac!(0));
            let e2 = v.map.insert(id, new_transfer_map(v.one.clone()));
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
        let tally = &mut self.tally;
        let one = &self.one;
        let mut vote_count = tally.get_mut(&candidate).expect("Candidate not found");
        *vote_count = &*vote_count + frac!(ballot.weight);

        // Add the ballot to the candidate's bucket.
        let mut ballot_map = self.map.get_mut(&candidate).unwrap();
        let mut bucket = ballot_map.get_mut(&one).unwrap();
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

    pub fn find_next_valid_preference(&self, b: &Ballot) -> Option<usize> {
        for (i, cand) in b.prefs[b.current .. ].iter().enumerate() {
            if self.tally.get(cand).is_some() {
                return Some(b.current + i);
            }
        }
        None
    }

    fn redistribute_votes(&mut self, candidate: CandidateId, transfer_value: Option<Frac>) {
        let ballots: TransferMap<'a> = self.map.remove(&candidate).unwrap();
        self.tally.remove(&candidate).unwrap();

        let grouped_ballots: BallotMap<'a> = group_by_candidate(&*self, ballots, &transfer_value);
        let tallies = compute_tallies(&grouped_ballots);

        for (candidate, transfer_map) in grouped_ballots {
            let mut candidate_map = self.map.get_mut(&candidate).unwrap();

            for (transfer_val, ballots) in transfer_map {
                let mut bucket = candidate_map.entry(transfer_val).or_insert_with(Vec::new);
                bucket.extend(ballots);
            }
        }

        for (candidate, vote_update) in tallies {
            let mut vote_count = self.tally.get_mut(&candidate).unwrap();
            *vote_count = &*vote_count + vote_update;
        }
    }

    pub fn elect_candidate(&mut self, candidate: CandidateId, quota: &Frac) {
        let transfer_value = {
            let num_votes = &self.tally[&candidate];
            (num_votes - quota) / num_votes
        };
        //transfer_value.normalize();
        trace!("Transferring at value: {:?}", transfer_value);
        self.redistribute_votes(candidate, Some(transfer_value))
    }

    pub fn knock_out_candidate(&mut self, candidate: CandidateId) {
        self.redistribute_votes(candidate, None)
    }
}
