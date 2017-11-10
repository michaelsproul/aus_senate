use ballot::*;
use candidate::*;
use util::*;
use arith::*;
use vote_log::*;

use itertools::Itertools;
use std::mem;

/// Map from transfer values to ballots with that transfer value.
pub type TransferMap<'a> = BTreeMap<Frac, Vec<&'a mut Ballot>>;

/// Intermediate data structure mapping candidates to ballots.
pub struct VoteMap<'a> {
    info: HashMap<CandidateId, VoteInfo<'a>>,
    candidates: &'a CandidateMap,
    one: Frac,
}

/// Per-candidate intermediate data.
struct VoteInfo<'a> {
    votes: VoteLog,
    ballots: TransferMap<'a>,
    eliminated: bool,
}

pub struct PreferenceTransfer<'a>(pub CandidateId, pub Frac, pub Vec<&'a mut Ballot>);

pub struct CandidateElected<'a> {
    pub id: CandidateId,
    pub votes: Int,
    pub transfers: Vec<PreferenceTransfer<'a>>,
}

pub struct CandidateExcluded<'a> {
    pub id: CandidateId,
    pub transfers: Vec<PreferenceTransfer<'a>>,
}

impl<'a> VoteInfo<'a> {
    fn new() -> Self {
        VoteInfo {
            votes: VoteLog::new(),
            ballots: new_transfer_map(),
            eliminated: false,
        }
    }

    fn take_ballots(&mut self) -> TransferMap<'a> {
        mem::replace(&mut self.ballots, new_transfer_map())
    }
}

fn new_transfer_map<'a>() -> TransferMap<'a> {
    let mut map = TransferMap::new();
    map.insert(frac!(1), vec![]);
    map
}

impl<'a> VoteMap<'a> {
    pub fn new(candidates: &'a CandidateMap) -> Result<VoteMap<'a>, String> {
        let mut v = VoteMap {
            info: HashMap::new(),
            candidates: candidates,
            one: frac!(1),
        };
        for &id in candidates.keys() {
            let prev = v.info.insert(id, VoteInfo::new());
            if prev.is_some() {
                return Err(format!("Candidate ID {} appears more than once", id));
            }
        }
        debug_assert_eq!(v.info.len(), candidates.len());
        Ok(v)
    }

    /// Add votes to a candidate's tally according to the weight and current preference of a ballot.
    pub fn add(&mut self, idx: usize, ballot: &'a mut Ballot) {
        let candidate = ballot.prefs[ballot.current];

        let all_info = &mut self.info;
        let info = all_info.get_mut(&candidate).expect("Candidate not found");

        // Add to the candidate's tally.
        info.votes.update_vote(idx, Int::from(ballot.weight));

        // Add the ballot to the appropriate bucket.
        let bucket = info.ballots.get_mut(&self.one).unwrap();
        bucket.push(ballot);
    }

    /// Get the IDs of all candidates whose vote exceeds the quota.
    pub fn get_candidates_with_quota(&self, quota: &Int) -> Vec<CandidateId> {
        let mut candidates_with_quota = self.info
            .iter()
            .filter(|&(_, info)| !info.eliminated)
            .map(|(id, info)| (id, info.votes.latest()))
            .filter(|&(_, votes)| votes >= quota)
            .collect::<Vec<_>>();

        // Sort by vote descending.
        candidates_with_quota.sort_by(|&(_, v1), &(_, v2)| v1.cmp(v2).reverse());

        candidates_with_quota.into_iter().map(|(c, _)| *c).collect()
    }

    /// Get the ID of the candidate with the least votes.
    pub fn get_last_candidate(&self) -> CandidateId {
        let mut sorted_candidates: Vec<_> = self.candidates_remaining().collect();
        sorted_candidates.sort_by_key(|&(_, info)| info.votes.latest());

        let min_vote = sorted_candidates[0].1.votes.latest().clone();

        // Collect all candidates with the minimum vote.
        let min_candidates: Vec<_> = sorted_candidates
            .into_iter()
            .take_while(|&(_, info)| info.votes.latest() == &min_vote)
            .collect();

        if min_candidates.len() == 1 {
            let (candidate, _) = min_candidates[0];
            return candidate;
        }

        // Try to break the tie based on past tallies.
        let historical_min = min_candidates
            .iter()
            .map(|&(_, info)| &info.votes)
            .min()
            .unwrap();

        let hist_min_candidates: Vec<_> = min_candidates
            .into_iter()
            .filter(|&(_, info)| &info.votes == historical_min)
            .map(|(candidate, _)| candidate)
            .sorted();

        if hist_min_candidates.len() == 1 {
            return hist_min_candidates[0];
        }

        // Ask the user...
        println!("Uhoh, there's been a tie for last...");
        println!(
            "Which of these hapless candidates would you like to use your ill-gotten \
             faux-democratic power to exclude?"
        );
        for (idx, candidate) in hist_min_candidates.iter().enumerate() {
            println!("{}: {:?}", idx, self.candidates[candidate]);
        }

        let mut index: usize;
        loop {
            index = read!();
            if index < hist_min_candidates.len() {
                let candidate = hist_min_candidates[index];
                println!("Ok, excluding {:?}", self.candidates[&candidate]);
                return candidate;
            }
        }
    }

    pub fn find_next_valid_preference(&self, b: &Ballot) -> Option<usize> {
        for (i, cand) in b.prefs[b.current..].iter().enumerate() {
            if !self.info[cand].eliminated {
                return Some(b.current + i);
            }
        }
        None
    }

    pub fn num_candidates_remaining(&self) -> usize {
        self.candidates_remaining().count()
    }

    fn candidates_remaining<'b>(&'b self) -> impl Iterator<Item = (CandidateId, &'b VoteInfo<'a>)> {
        self.info
            .iter()
            .filter(|&(_, info)| !info.eliminated)
            .map(|(id, info)| (*id, info))
    }

    pub fn elect_remaining(self) -> Vec<CandidateElected<'a>> {
        self.info
            .into_iter()
            .filter(|&(_, ref info)| !info.eliminated)
            .map(|(id, info)| {
                CandidateElected {
                    id: id,
                    votes: info.votes.latest().clone(),
                    transfers: vec![],
                }
            })
            .collect()
    }

    pub fn transfer_preferences(&mut self, idx: usize, transfer: PreferenceTransfer<'a>) {
        let PreferenceTransfer(_, transfer_val, all_ballots) = transfer;

        let grouped_ballots = group_ballots_by_candidate(&*self, all_ballots);

        for (continuing_id, ballots) in grouped_ballots {
            let mut info = self.info.get_mut(&continuing_id).unwrap();

            assert!(!info.eliminated);

            let incr = ballot_value(&transfer_val, &ballots);
            info.votes.update_vote(idx, incr.clone());
            if !incr.is_zero() {
                trace!(
                    "+{:?} votes for {:?}, brings total to {:?}",
                    incr,
                    self.candidates[&continuing_id],
                    info.votes.latest()
                );
            }

            let bucket = info.ballots
                .entry(transfer_val.clone())
                .or_insert_with(Vec::new);
            bucket.extend(ballots);
        }
    }

    pub fn elect_candidates_with_quota(&mut self, quota: &Int) -> Vec<CandidateElected<'a>> {
        let candidates = self.get_candidates_with_quota(quota);
        let mut elected = vec![];

        for candidate in candidates {
            let info = self.info.get_mut(&candidate).unwrap();

            // Mark eliminated.
            info.eliminated = true;

            let num_votes = info.votes.latest().clone();

            // Create `PreferenceTransfer` events for each transfer value.
            let transfer_map = info.take_ballots();

            // Collect all ballots (erasing existing transfer values).
            let all_ballots: Vec<_> = transfer_map
                .into_iter()
                .flat_map(|(_, ballots)| ballots)
                .collect();

            let num_ballots: u32 = all_ballots.iter().map(|b| b.weight).sum();

            // Aggregate transfer value that accounts for the ones we just threw out...
            let transfer_value = Frac::ratio(&(&num_votes - quota), &Int::from(num_ballots));

            let pref_transfers = vec![PreferenceTransfer(candidate, transfer_value, all_ballots)];

            elected.push(CandidateElected {
                id: candidate,
                votes: num_votes,
                transfers: pref_transfers,
            });
        }

        elected
    }

    /// Panics if the `id` is not the `CandidateId` of a real candidate.
    pub fn exclude_candidate_by_id(&mut self, candidate: CandidateId) -> CandidateExcluded<'a> {
        let info = self.info.get_mut(&candidate).unwrap();

        info.eliminated = true;

        let transfer_map = info.take_ballots();

        let mut pref_transfers: Vec<_> = transfer_map
            .into_iter()
            .map(|(transfer_val, ballots)| {
                PreferenceTransfer(candidate, transfer_val, ballots)
            })
            .collect();

        // Reverse the preference transfer events so they're ordered from largest to
        // smallest transfer value.
        pref_transfers.reverse();

        CandidateExcluded {
            id: candidate,
            transfers: pref_transfers,
        }
    }

    // TODO: bulk exclusions.
    pub fn exclude_candidates(&mut self) -> Vec<CandidateExcluded<'a>> {
        let candidate = self.get_last_candidate();
        vec![self.exclude_candidate_by_id(candidate)]
    }

    pub fn print_summary(&self) {
        trace!("Vote tallies");
        for (candidate, info) in self.info.iter().filter(|&(_, i)| !i.eliminated) {
            trace!(
                "{:?}: {:?} votes",
                self.candidates[candidate],
                info.votes.latest()
            );
        }
    }
}
