use util::*;

/// A candidate ID is an integer representing a candidate.
pub type CandidateId = u32;

/// A Ballot represents an individual's order of preferences.
pub struct Ballot {
    /// Number of people who voted this way.
    pub count: u32,
    /// Ordering of candidates.
    pub prefs: Vec<CandidateId>,
    /// Index of the first candidate in `prefs` who is still in the running.
    pub current: usize
}
