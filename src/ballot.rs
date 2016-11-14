use candidate::*;
use util::*;

/// A Ballot represents an individual's order of preferences.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Ballot {
    /// Ordering of candidates.
    pub prefs: Vec<CandidateId>,
    /// Index of the first candidate in `prefs` who is still in the running.
    pub current: usize,
    /// Ballot weighting, where None means 1.
    pub weight: u32,
}

impl Ballot {
    pub fn single(prefs: Vec<CandidateId>) -> Ballot {
        Ballot {
            prefs: prefs,
            current: 0,
            weight: 1,
        }
    }

    pub fn multi(weight: u32, prefs: Vec<CandidateId>) -> Ballot {
        Ballot {
            prefs: prefs,
            current: 0,
            weight: weight,
        }
    }

    pub fn is_exhausted(&self) -> bool {
        self.current == self.prefs.len() - 1
    }
}
