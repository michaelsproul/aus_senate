use std::fmt::{Debug, Error, Formatter};
use util::*;

/// Integer representing a candidate.
pub type CandidateId = u16;

/// Description of a candidate including name and party affiliation.
#[derive(Serialize, Clone)]
pub struct Candidate {
    pub id: CandidateId,
    pub surname: String,
    pub other_names: String,
    pub group_name: String,
    pub party: String,
    pub state: String,
}

/// User-input description of a candidate with first name and surname.
#[derive(Serialize, Clone, Debug)]
pub struct CandidateName {
    pub first: String,
    pub last: String,
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

/// Get the list of candidate IDs, in canonical order (used for parsing).
pub fn get_candidate_id_list(all_candidates: &[Candidate], state: &str) -> Vec<CandidateId> {
    all_candidates
        .iter()
        .filter(|c| c.state == state)
        .map(|c| c.id)
        .collect()
}

/// Convert a list of candidate names into a list of candidate IDs.
pub fn find_candidates_with_names(
    candidate_names: &[CandidateName],
    candidates: &CandidateMap,
) -> Vec<CandidateId> {
    candidate_names
        .iter()
        .flat_map(|name| {
            candidates
                .iter()
                .find(|&(_, cand)| {
                    cand.surname.to_lowercase() == name.last.to_lowercase()
                        && cand
                            .other_names
                            .to_lowercase()
                            .contains(&name.first.to_lowercase())
                })
                .map(|(&id, _)| id)
        })
        .collect()
}
