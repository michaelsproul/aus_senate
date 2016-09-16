use stats::Stats;
use candidate::*;

#[derive(Debug)]
pub struct Senate<'a> {
    pub senators: Vec<&'a Candidate>,
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

    pub fn add_senator(&mut self, id: CandidateId, candidates: &'a CandidateMap) {
        self.senators.push(&candidates[&id])
    }

    pub fn num_elected(&self) -> usize {
        self.senators.len()
    }
}
