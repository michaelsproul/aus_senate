use stats::Stats;
use util::Int;
use candidate::*;

#[derive(Debug)]
pub struct Senate<'a> {
    /// List of senators and the vote tally they were elected on.
    pub senators: Vec<(&'a Candidate, Int)>,
    pub tied: bool,
    pub stats: Stats,
}

impl<'a> Senate<'a> {
    pub fn new() -> Senate<'a> {
        Senate {
            senators: vec![],
            tied: false,
            stats: Stats::new(),
        }
    }

    pub fn add_senator(&mut self, id: CandidateId, tally: Int, candidates: &'a CandidateMap) {
        self.senators.push((&candidates[&id], tally))
    }

    pub fn num_elected(&self) -> usize {
        self.senators.len()
    }
}
