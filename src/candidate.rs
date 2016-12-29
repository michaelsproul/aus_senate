use util::*;
use std::fmt::{Debug, Formatter, Error};

/// Integer representing a candidate.
pub type CandidateId = u32;

/// Description of a candidate including name and party affiliation.
#[derive(Clone)]
pub struct Candidate {
    pub id: CandidateId,
    pub surname: String,
    pub other_names: String,
    pub group_name: String,
    pub party: String,
    pub state: String,
}

impl Debug for Candidate {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{} {}", self.other_names, self.surname)
    }
}

/// Map from candidate IDs to candidate information.
pub type CandidateMap = HashMap<CandidateId, Candidate>;

pub fn get_state_candidates(all_candidates: &[Candidate], state: &str) -> CandidateMap {
    let mut result = HashMap::new();
    for c in all_candidates.iter().filter(|c| c.state == state) {
        result.insert(c.id, c.clone());
    }
    result
}
