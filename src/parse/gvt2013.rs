use super::prelude::*;

/// Group voting ticket description. Maps states to ticket names to preference lists.
pub type GVT = HashMap<String, HashMap<String, Vec<CandidateId>>>;

/// Holy moley.
#[derive(Deserialize, Debug)]
struct GVTRow {
    state: String,
    owner_group: u32,
    owner_group_name: String,
    owner_ticket: String,
    ticket_num: u32,
    candidate_id: u32,
    candidate_ticket: String,
    surname: String,
    first_name: String,
    ballot_pos: u32,
    party_ab: String,
    party_name: String,
    preference: u32
}

// NOTE: This is a tad slow, but it beats mucking around with manual row groupings.
pub fn parse<R: Read>(input: R) -> Result<GVT, Box<Error>> {
    let mut data: HashMap<String, HashMap<String, PrefMap>> = HashMap::new();

    let mut reader = ::csv::Reader::from_reader(input);

    for result in reader.deserialize::<GVTRow>() {
        let row = result?;
        let ticket_map = data.entry(row.state).or_insert_with(HashMap::new);
        let pref_map = ticket_map.entry(row.owner_ticket).or_insert_with(BTreeMap::new);
        pref_map.insert(row.preference, row.candidate_id);
    }

    // Convert inner preference maps into lists.
    let mut result = HashMap::new();
    for (state, ticket_map) in data {
        let new_ticket_map = result.entry(state).or_insert_with(HashMap::new);
        for (ticket, pref_map) in ticket_map {
            new_ticket_map.insert(ticket, flatten_pref_map(pref_map));
        }
    }
    Ok(result)
}
