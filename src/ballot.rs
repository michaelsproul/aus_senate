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
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Ballot {
    /// Ordering of candidates.
    pub prefs: Vec<CandidateId>,
    /// Index of the first candidate in `prefs` who is still in the running.
    pub current: usize
}

impl Ballot {
    pub fn new(prefs: Vec<CandidateId>) -> Ballot {
        Ballot {
            prefs: prefs,
            current: 0,
        }
    }

    pub fn is_exhausted(&self) -> bool {
        self.current == self.prefs.len() - 1
    }
}

/// During parsing, we sometimes know that a ballot has a value greater than 1.
///
/// For the main algorithm, we use a regular Ballot and a count stored in a HashMap,
/// in order to avoid the memory overhead of storing the vote value in the MultiBallot.
pub struct MultiBallot {
    /// Number of people that voted according to this set of preferences.
    pub value: Option<u32>,
    /// Preferences shared by everyone on this multi-ballot.
    pub ballot: Ballot,
}

impl MultiBallot {
    pub fn single(prefs: Vec<CandidateId>) -> MultiBallot {
        MultiBallot::multi(1, prefs)
    }

    pub fn multi(value: u32, prefs: Vec<CandidateId>) -> MultiBallot {
        MultiBallot {
            value: Some(value),
            ballot: Ballot::new(prefs),
        }
    }
}
