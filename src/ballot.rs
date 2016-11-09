use candidate::*;
use util::*;

/// A Ballot represents an individual's order of preferences.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Ballot<T> {
    /// Ordering of candidates.
    pub prefs: Vec<CandidateId>,
    /// Index of the first candidate in `prefs` who is still in the running.
    pub current: usize,
    /// Ballot weighting, where None means 1.
    pub weight: Option<T>,
}

impl <T> Ballot<T> {
    pub fn single(prefs: Vec<CandidateId>) -> Ballot<T> {
        Ballot {
            prefs: prefs,
            current: 0,
            weight: None,
        }
    }

    pub fn multi(weight: T, prefs: Vec<CandidateId>) -> Ballot<T> {
        Ballot {
            prefs: prefs,
            current: 0,
            weight: Some(weight),
        }
    }

    pub fn is_exhausted(&self) -> bool {
        self.current == self.prefs.len() - 1
    }
}

impl Ballot<u32> {
    pub fn to_fractional(self) -> Ballot<Frac> {
        Ballot {
            prefs: self.prefs,
            current: self.current,
            weight: self.weight.map(|x| frac!(x))
        }
    }
}

impl Ballot<Frac> {
    pub fn apply_weighting(&mut self, weighting: &Option<&Frac>) {
        if let &Some(weighting) = weighting {
            self.weight = match self.weight {
                Some(ref x) => Some(x * weighting),
                None => Some(weighting.clone())
            };
        }
    }
}
