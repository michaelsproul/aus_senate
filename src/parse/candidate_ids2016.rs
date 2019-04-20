use super::prelude::*;
use std::io::{BufRead, BufReader};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct CandidateRow {
    state_ab: String,
    #[serde(rename = "DivisionID")]
    division_id: String,
    division_nm: String,
    party_ab: String,
    party_nm: String,
    #[serde(rename = "CandidateID")]
    candidate_id: CandidateId,
    surname: String,
    #[serde(rename = "GivenNm")]
    given_names: String,
    elected: String,
    historic_elected: String,
}

/// Map from surname to (given_names, candidate_id)
pub type CandidateIdLookup = HashMap<String, Vec<(String, CandidateId)>>;

pub fn lookup_candidate_id(
    id_lookup: &CandidateIdLookup,
    surname: &str,
) -> Result<CandidateId, Box<Error>> {
    let candidates = id_lookup
        .get(surname)
        .ok_or_else(|| format!("candidate not found {}", surname))?;

    assert!(candidates.len() == 1);
    Ok(candidates[0].1)
}

pub fn parse<R: Read>(input: R) -> Result<CandidateIdLookup, Box<Error>> {
    let mut result = HashMap::new();
    let mut reader = BufReader::new(input);
    // Skip the first line
    reader.read_line(&mut String::new())?;
    let mut csv_reader = ::csv::Reader::from_reader(reader);

    for raw_row in csv_reader.deserialize::<CandidateRow>() {
        let row = raw_row?;

        result
            .entry(row.surname)
            .or_insert_with(Vec::new)
            .push((row.given_names, row.candidate_id));
    }

    Ok(result)
}
