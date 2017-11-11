use super::prelude::*;

#[derive(Deserialize, Debug)]
struct CandidateRow {
    txn_nm: String,
    nom_ty: String,
    state_ab: String,
    div_nm: String,
    ticket: String,
    ballot_position: u32,
    surname: String,
    ballot_given_nm: String,
    party_ballot_nm: String,
    occupation: String,
    address_1: String,
    address_2: String,
    postcode: String,
    suburb: String,
    address_state_ab: String,
    contact_work_ph: String,
    contact_home_ph: String,
    postal_address_1: String,
    postal_address_2: String,
    postal_suburb: String,
    postal_postcode: String,
    contact_fax: String,
    postal_state_ab: String,
    contact_mobile_no: String,
    contact_email: String,
}

pub fn parse<R: Read>(input: R) -> Result<Vec<Candidate>, Box<Error>> {
    let mut result = vec![];
    let mut reader = ::csv::Reader::from_reader(input);

    for (id, raw_row) in reader.deserialize::<CandidateRow>().enumerate() {
        let row = raw_row?;
        if row.nom_ty != "S" {
            continue;
        }
        result.push(Candidate {
            id: id as u32,
            surname: row.surname,
            other_names: row.ballot_given_nm,
            group_name: row.ticket,
            party: row.party_ballot_nm,
            state: row.state_ab,
        });
    }

    Ok(result)
}
