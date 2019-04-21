use candidate::*;

/// A Ballot represents an individual's order of preferences.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Ballot {
    /// Ordering of candidates.
    pub prefs: Vec<CandidateId>,
    /// Index of the first candidate in `prefs` who is still in the running.
    current: usize,
    #[cfg(feature = "support2013")]
    weight: u32,
}

impl Ballot {
    pub fn single(prefs: Vec<CandidateId>) -> Ballot {
        Ballot {
            prefs,
            current: 0,
            #[cfg(feature = "support2013")]
            weight: 1,
        }
    }

    #[cfg(feature = "support2013")]
    pub fn multi(weight: u32, prefs: Vec<CandidateId>) -> Ballot {
        Ballot {
            prefs,
            current: 0,
            weight,
        }
    }

    #[cfg(feature = "support2013")]
    pub fn weight(&self) -> u32 {
        self.weight
    }

    #[cfg(not(feature = "support2013"))]
    pub fn weight(&self) -> u32 {
        1
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn set_current(&mut self, idx: usize) {
        // NOTE: could use a smaller type to store `self.current` but it probably isn't worth
        // it because the `Ballot` struct gets packed with padding bytes for alignment.
        self.current = idx;
    }
}
