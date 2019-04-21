use candidate::*;
use stats::Stats;
use util::Int;

#[derive(Debug, Default)]
pub struct Senate {
    /// List of senators and the vote tally they were elected on.
    pub senators: Vec<(Candidate, Int)>,
    pub tied: bool,
    pub stats: Stats,
}

impl Senate {
    pub fn new() -> Senate {
        Senate {
            senators: vec![],
            tied: false,
            stats: Stats::new(),
        }
    }

    pub fn add_senator(&mut self, id: CandidateId, tally: Int, candidates: &CandidateMap) {
        self.senators.push((candidates[&id].clone(), tally))
    }

    pub fn num_elected(&self) -> usize {
        self.senators.len()
    }
}
