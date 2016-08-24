use util::*;
use candidate::*;

/// Temporary preference map type mapping candidate IDs to preferences.
pub type PrefMap = HashMap<CandidateId, u32>;

pub fn pref_map_to_vec(pref_map: PrefMap) -> Vec<CandidateId> {
    let mut temp: Vec<_> = pref_map.into_iter().collect();
    temp.sort_by_key(|&(_, pref)| pref);
    temp.into_iter().map(|(cand, _)| cand).collect()
}

/// A Ballot represents an individual's order of preferences.
#[derive(Debug)]
pub struct Ballot {
    /// Ballot weight, initially the number of people who voted this way.
    pub weight: Frac,
    /// Ordering of candidates.
    pub prefs: Vec<CandidateId>,
    /// Index of the first candidate in `prefs` who is still in the running.
    pub current: usize
}

impl Ballot {
    pub fn new(count: u32, prefs: Vec<CandidateId>) -> Ballot {
        Ballot {
            weight: frac!(count),
            prefs: prefs,
            current: 0,
        }
    }

    pub fn is_exhausted(&self) -> bool {
        self.current == self.prefs.len() - 1
    }
}
