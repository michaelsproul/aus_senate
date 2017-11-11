use super::prelude::*;

// TODO: Use this parser for 2016 candidate files as well.

#[derive(Deserialize, Debug)]
struct CandidateRow {
    state_ab: String,
    party_ab: String,
    party_name: String,
    candidate_id: CandidateId,
    surname: String,
    given_names: String,
    elected: String,
    historic_elected: String,
}

pub fn parse<R: Read>(input: R) -> Result<Vec<Candidate>, Box<Error>> {
    let mut result = vec![];
    let mut reader = ::csv::Reader::from_reader(input);

    for raw_row in reader.deserialize::<CandidateRow>() {
        let row = raw_row?;
        result.push(Candidate {
            id: row.candidate_id,
            surname: row.surname,
            other_names: row.given_names,
            group_name: row.party_ab,
            party: row.party_name,
            state: row.state_ab,
        });
    }

    Ok(result)
}
