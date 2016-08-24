use util::*;

/// Integer representing a candidate.
pub type CandidateId = u32;

/// Description of a candidate including name and party affiliation.
#[derive(Clone, Debug)]
pub struct Candidate {
    pub id: CandidateId,
    pub surname: String,
    pub other_names: String,
    pub group_name: String,
    pub party: String,
    pub state: String,
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

