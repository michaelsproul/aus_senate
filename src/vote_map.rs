use ballot::*;
use candidate::*;
use util::*;
use arith::*;

use std::mem;

/// Map from transfer values to ballots with that transfer value.
pub type TransferMap<'a> = BTreeMap<Frac, Vec<&'a mut Ballot>>;

/// Map from (candidate_id => (transfer_value => ballots)).
pub type BallotMap<'a> = HashMap<CandidateId, TransferMap<'a>>;

/// Intermediate data structure mapping candidates to ballots.
pub struct VoteMap<'a> {
    info: HashMap<CandidateId, VoteInfo<'a>>,
    candidates: &'a CandidateMap,
    one: Frac,
}

/// Per-candidate intermediate data.
struct VoteInfo<'a> {
    votes: Int,
    ballots: TransferMap<'a>,
    eliminated: bool,
}

impl<'a> VoteInfo<'a> {
    fn new(one: Frac) -> Self {
        VoteInfo {
            votes: Int::from(0),
            ballots: new_transfer_map(one),
            eliminated: false,
        }
    }
}

fn new_transfer_map<'a>(one: Frac) -> TransferMap<'a> {
    let mut map = TransferMap::new();
    map.insert(one, vec![]);
    map
}

impl <'a> VoteMap<'a> {
    pub fn new(candidates: &'a CandidateMap) -> Result<VoteMap<'a>, String> {
        let mut v = VoteMap {
            info: HashMap::new(),
            candidates: candidates,
            one: frac!(1),
        };
        for &id in candidates.keys() {
            let prev = v.info.insert(id, VoteInfo::new(v.one.clone()));
            if prev.is_some() {
                return Err(format!("Candidate ID {} appears more than once", id));
            }
        }
        debug_assert!(v.info.len() == candidates.len());
        Ok(v)
    }

    /// Add votes to a candidate's tally according to the weight and current preference of a ballot.
    pub fn add(&mut self, ballot: &'a mut Ballot) {
        let candidate = ballot.prefs[ballot.current];

        let all_info = &mut self.info;
        let mut info = all_info.get_mut(&candidate).expect("Candidate not found");

        // Add to the candidate's tally.
        info.votes = &info.votes + Int::from(ballot.weight);

        // Add the ballot to the appropriate bucket.
        // TODO: kill this "one" business?
        let bucket = info.ballots.get_mut(&self.one).unwrap();
        bucket.push(ballot);
    }

    /// Get the IDs of all candidates whose vote exceeds the quota.
    pub fn get_candidates_with_quota(&self, quota: &Int) -> Vec<CandidateId> {
        let mut candidates_with_quota = self.info.iter()
            .map(|(id, info)| (id, &info.votes))
            .filter(|&(_, votes)| votes >= quota)
            .collect::<Vec<_>>();

        // Sort by vote descending.
        candidates_with_quota.sort_by(|&(_, v1), &(_, v2)| v1.cmp(v2).reverse());

        candidates_with_quota.into_iter().map(|(c, _)| *c).collect()
    }

    /// Get the ID of a candidate whose vote exceeds the given quota.
    pub fn get_candidate_with_quota(&self, quota: &Int) -> Option<CandidateId> {
        self.get_candidates_with_quota(quota).first().cloned()
    }

    /// Get the ID of the candidate with the least votes.
    pub fn get_last_candidate(&self) -> CandidateId {
        self.candidates_remaining()
            .min_by_key(|&(_, info)| &info.votes)
            .map(|(id, _)| id)
            .unwrap()
    }

    /// Get the integer tally for a candidate (assuming they're in the map).
    pub fn get_tally(&self, candidate: CandidateId) -> Int {
        self.info[&candidate].votes.clone()
    }

    pub fn find_next_valid_preference(&self, b: &Ballot) -> Option<usize> {
        for (i, cand) in b.prefs[b.current .. ].iter().enumerate() {
            if !self.info[cand].eliminated {
                return Some(b.current + i);
            }
        }
        None
    }

    fn redistribute_votes(&mut self, candidate: CandidateId, transfer_value: Option<Frac>) {
        info!("Distributing preferences for {:?}", self.candidates[&candidate]);

        // Zero the elected candidate's vote, mark them eliminated and sieze their ballots.
        let ballots = {
            let info = self.info.get_mut(&candidate).unwrap();
            info.votes = Int::from(0);
            info.eliminated = true;

            mem::replace(&mut info.ballots, new_transfer_map(frac!(1)))
        };

        let grouped_ballots: BallotMap<'a> = group_by_candidate(&*self, ballots, &transfer_value);
        let tallies = compute_tallies(&grouped_ballots);

        // FIXME: clean this up.
        for (cand, transfer_map) in grouped_ballots {
            let candidate_map = &mut self.info.get_mut(&cand).unwrap().ballots;

            for (transfer_val, ballots) in transfer_map {
                let mut bucket = candidate_map.entry(transfer_val).or_insert_with(Vec::new);
                trace!("Transferring {} ballots from {:?} to {:?}",
                    ballots.len(), self.candidates[&candidate], self.candidates[&cand]
                );
                bucket.extend(ballots);
            }
        }

        for (cand, vote_update) in tallies {
            let vote_count = &mut self.info.get_mut(&cand).unwrap().votes;

            trace!("Votes for candidate {:?}: {:?} + {:?} = {:?}",
                self.candidates[&cand], vote_count, vote_update, &*vote_count + vote_update.clone()
            );
            *vote_count = &*vote_count + vote_update;
        }
    }

    pub fn elect_candidate(&mut self, candidate: CandidateId, quota: &Int) {
        let transfer_value = {
            let num_votes = &self.info[&candidate].votes;
            Frac::ratio(&(num_votes - quota), &num_votes)
        };
        //transfer_value.normalize();
        trace!("Transferring at value: {:?}", transfer_value);
        self.redistribute_votes(candidate, Some(transfer_value))
    }

    pub fn knock_out_candidate(&mut self, candidate: CandidateId) {
        self.redistribute_votes(candidate, None)
    }

    pub fn num_candidates_remaining(&self) -> usize {
        self.candidates_remaining().count()
    }

    fn candidates_remaining<'b>(&'b self) -> impl Iterator<Item=(CandidateId, &'b VoteInfo<'a>)> {
        self.info
            .iter()
            .filter(|&(_, info)| !info.eliminated)
            .map(|(id, info)| (*id, info))
    }

    pub fn drain(self) -> Vec<(CandidateId, Int)> {
        self.info
            .into_iter()
            .filter(|&(_, ref info)| !info.eliminated)
            .map(|(id, info)| (id, info.votes))
            .collect()
    }

    pub fn mark_eliminated(&mut self, candidate: CandidateId) {
        let info = self.info.get_mut(&candidate).unwrap();
        info.eliminated = true;
    }
}
