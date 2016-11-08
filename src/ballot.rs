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
    pub weight: Option<Frac>,
}

impl Ballot {
    pub fn single(prefs: Vec<CandidateId>) -> Ballot {
        Ballot {
            prefs: prefs,
            current: 0,
            weight: None,
        }
    }

    pub fn multi(weight: u32, prefs: Vec<CandidateId>) -> Ballot {
        Ballot {
            prefs: prefs,
            current: 0,
            weight: Some(frac!(weight)),
        }
    }

    pub fn is_exhausted(&self) -> bool {
        self.current == self.prefs.len() - 1
    }

    pub fn apply_weighting(&mut self, weighting: &Option<&Frac>) {
        if let &Some(weighting) = weighting {
            self.weight = match self.weight {
                Some(ref x) => Some(x * weighting),
                None => Some(weighting.clone())
            };
        }
    }
}

/*
pub struct InputBallot {
    /// Number of people that voted according to this set of preferences.
    pub value: u32,
    /// Preferences shared by everyone on this multi-ballot.
    pub ballot: Ballot,
}

impl MultiBallot {
    pub fn single(prefs: Vec<CandidateId>) -> MultiBallot {
        MultiBallot::multi(1, prefs)
    }

    pub fn multi(value: u32, prefs: Vec<CandidateId>) -> MultiBallot {
        MultiBallot {
            value: value,
            ballot: Ballot::new(prefs),
        }
    }
}
*/
